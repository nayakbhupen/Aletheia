use crate::date::DateNormalizer;
use crate::nli::{DeterministicNli, NliVerdict};
use crate::numeric::NumericNormalizer;
use crate::wikidata::WikidataResolver;
use aletheia_core::{BenchmarkItem, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradingTier {
    ExactMatch,
    WikidataEntity,
    NumericDateNorm,
    NliEntailment,
    Failed,
}

impl GradingTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            GradingTier::ExactMatch => "TIER_0_EXACT",
            GradingTier::WikidataEntity => "TIER_1_WIKIDATA",
            GradingTier::NumericDateNorm => "TIER_2_NUMERIC_DATE",
            GradingTier::NliEntailment => "TIER_3_NLI",
            GradingTier::Failed => "TIER_FAILED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingDecision {
    pub is_correct: bool,
    pub tier: GradingTier,
    pub confidence: f64,
    pub rationale: String,
}

pub struct GradingCascade {
    wikidata_resolver: WikidataResolver,
    numeric_tolerance: f64,
}

impl GradingCascade {
    pub fn new(wikidata_resolver: WikidataResolver) -> Self {
        Self {
            wikidata_resolver,
            numeric_tolerance: 0.01,
        }
    }

    pub fn in_memory() -> Result<Self> {
        let resolver = WikidataResolver::in_memory()?;
        Ok(Self::new(resolver))
    }

    #[inline(always)]
    fn sound_match(candidate: &str, target: &str) -> bool {
        if target.is_empty() || candidate.is_empty() {
            return false;
        }
        if candidate == target {
            return true;
        }
        if target.len() <= 3 {
            candidate.split(|c: char| !c.is_alphanumeric()).any(|w| w == target)
        } else {
            candidate.contains(target)
        }
    }

    /// Grade a candidate response against a benchmark item
    pub fn grade(&self, item: &BenchmarkItem, candidate: &str) -> GradingDecision {
        let cand_trim = candidate.trim().to_lowercase();
        let target_trim = item.target_answer.trim().to_lowercase();
        let truth_trim = item.ground_truth.trim().to_lowercase();

        // 1. Exact Match / Sound Substring on canonical target
        if Self::sound_match(&cand_trim, &target_trim) {
            return GradingDecision {
                is_correct: true,
                tier: GradingTier::ExactMatch,
                confidence: 1.0,
                rationale: "Exact canonical answer match".to_string(),
            };
        }

        if Self::sound_match(&cand_trim, &truth_trim) {
            return GradingDecision {
                is_correct: true,
                tier: GradingTier::ExactMatch,
                confidence: 0.99,
                rationale: "Exact ground truth match".to_string(),
            };
        }

        // 2. Wikidata QID Grounding Match
        if let Some(grounding) = &item.qid_grounding {
            if self.wikidata_resolver.matches_entity(grounding, candidate) {
                return GradingDecision {
                    is_correct: true,
                    tier: GradingTier::WikidataEntity,
                    confidence: 0.98,
                    rationale: format!(
                        "Wikidata QID entity match on QID {} ({})",
                        grounding.subject_qid, grounding.object_value
                    ),
                };
            }
        }

        // 3. Numeric & Date Normalizer
        if NumericNormalizer::matches_numeric_target(candidate, &item.ground_truth, self.numeric_tolerance) {
            return GradingDecision {
                is_correct: true,
                tier: GradingTier::NumericDateNorm,
                confidence: 0.95,
                rationale: "Numeric value matches ground truth within 1% tolerance".to_string(),
            };
        }

        if DateNormalizer::matches_date(candidate, &item.ground_truth) {
            return GradingDecision {
                is_correct: true,
                tier: GradingTier::NumericDateNorm,
                confidence: 0.95,
                rationale: "Normalized date matches ground truth".to_string(),
            };
        }

        // 4. Deterministic Frozen NLI Entailment
        let (verdict, score) = DeterministicNli::evaluate_entailment(&item.passage, candidate);
        match verdict {
            NliVerdict::Entailment => GradingDecision {
                is_correct: true,
                tier: GradingTier::NliEntailment,
                confidence: score,
                rationale: format!("Passage entails candidate with score {:.2}", score),
            },
            NliVerdict::Contradiction => GradingDecision {
                is_correct: false,
                tier: GradingTier::NliEntailment,
                confidence: score,
                rationale: format!("Candidate contradicts passage with score {:.2}", score),
            },
            NliVerdict::Neutral => GradingDecision {
                is_correct: false,
                tier: GradingTier::Failed,
                confidence: 0.5,
                rationale: "Insufficient evidence or neutral statement".to_string(),
            },
        }
    }
}
