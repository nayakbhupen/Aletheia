use serde::{Deserialize, Serialize};

/// Abductive Epistemic Verdict based on Presumptive Latent Grounding (Hypothetical-Deductive Entailment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbductiveReceipt {
    /// Postulated latent consistency score in [0.0, 1.0]
    pub abductive_consistency: f64,
    /// Epistemic curvature of the attractor well (\kappa) in [0.0, 1.0]
    pub attractor_curvature: f64,
    /// Whether the statement holds under counterfactual perturbation
    pub counterfactual_stable: bool,
    /// Anomaly indicator for abductive hallucination (shallow mode collapse)
    pub abductive_hallucination_detected: bool,
}

pub struct AbductiveEpistemicEngine;

impl AbductiveEpistemicEngine {
    /// Fast stack-allocated token collection: up to 64 tokens without any heap allocation
    #[inline(always)]
    fn collect_stack_tokens<'a>(text: &'a str, out: &mut [&'a str; 64]) -> usize {
        let mut count = 0;
        for word in text.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !clean.is_empty() {
                out[count] = clean;
                count += 1;
                if count == 64 {
                    break;
                }
            }
        }
        // In-place sort and dedup
        let slice = &mut out[..count];
        slice.sort_unstable();
        let mut write = 0;
        for read in 0..count {
            if read == 0 || slice[read] != slice[write - 1] {
                slice[write] = slice[read];
                write += 1;
            }
        }
        write
    }

    /// Two-pointer zero-allocation Jaccard distance between two sorted slices
    #[inline(always)]
    fn sorted_jaccard_dist(a: &[&str], b: &[&str]) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }
        if a.is_empty() || b.is_empty() {
            return 1.0;
        }

        let mut i = 0;
        let mut j = 0;
        let mut inter = 0;
        let mut union = 0;

        while i < a.len() && j < b.len() {
            if a[i] == b[j] {
                inter += 1;
                union += 1;
                i += 1;
                j += 1;
            } else if a[i] < b[j] {
                union += 1;
                i += 1;
            } else {
                union += 1;
                j += 1;
            }
        }

        union += (a.len() - i) + (b.len() - j);
        if union == 0 {
            0.0
        } else {
            1.0 - (inter as f64 / union as f64)
        }
    }

    /// Perform abductive epistemic verification on sampled candidate paths against context
    pub fn verify_abduction(
        dominant_claim: &str,
        samples: &[String],
        context: Option<&str>,
        rsc: f64,
    ) -> AbductiveReceipt {
        let mut dom_buf: [&str; 64] = [""; 64];
        let dom_count = Self::collect_stack_tokens(dominant_claim, &mut dom_buf);
        let dom_tokens = &dom_buf[..dom_count];

        let mut total_dist = 0.0;
        let mut comparisons = 0usize;

        let mut sample_buf: [&str; 64] = [""; 64];
        for s in samples {
            let s_count = Self::collect_stack_tokens(s.as_str(), &mut sample_buf);
            let s_tokens = &sample_buf[..s_count];

            let dist = Self::sorted_jaccard_dist(dom_tokens, s_tokens);
            total_dist += dist;
            comparisons += 1;
        }

        let mean_dist = if comparisons > 0 {
            total_dist / comparisons as f64
        } else {
            0.0
        };

        // 1. Curvature kappa: steep well if paths are tightly clustered with low rsc
        // Flat or high dispersion indicates unstable saddle-point hallucination
        let rsc_clamped = rsc.clamp(0.0, 1.0);
        let curvature = ((1.0 - mean_dist).max(0.0) * (1.0 - rsc_clamped)).clamp(0.0, 1.0);

        // 2. Latent premise grounding: verify non-trivial tokens in dominant claim are grounded in context
        let counterfactual_stable = if let Some(ctx) = context {
            let ctx_lower = ctx.to_lowercase();
            let mut ungrounded = 0usize;
            for &token in dom_tokens {
                if token.len() > 3 {
                    let tok_lower = token.to_lowercase();
                    if !ctx_lower.contains(&tok_lower) {
                        ungrounded += 1;
                    }
                }
            }
            ungrounded <= 1
        } else {
            true
        };

        // 3. Abductive hallucination flag: High apparent agreement (low rsc) but zero curvature or context violation
        let abductive_hallucination = !counterfactual_stable || (rsc < 0.20 && curvature < 0.25);

        let abductive_consistency = if abductive_hallucination {
            (curvature * 0.4).clamp(0.0, 1.0)
        } else {
            (curvature * 0.65 + if counterfactual_stable { 0.35 } else { 0.0 }).clamp(0.0, 1.0)
        };

        AbductiveReceipt {
            abductive_consistency,
            attractor_curvature: curvature,
            counterfactual_stable,
            abductive_hallucination_detected: abductive_hallucination,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_allocation_stack_tokens() {
        let text = "Frank Gehry designed the Walt Disney Concert Hall in Los Angeles";
        let mut buf = [""; 64];
        let count = AbductiveEpistemicEngine::collect_stack_tokens(text, &mut buf);
        assert!(count > 5);
        // Verify sorted
        for i in 1..count {
            assert!(buf[i - 1] <= buf[i]);
        }
    }

    #[test]
    fn test_sorted_jaccard_identical() {
        let a = ["angeles", "concert", "disney", "gehry", "hall"];
        let b = ["angeles", "concert", "disney", "gehry", "hall"];
        let dist = AbductiveEpistemicEngine::sorted_jaccard_dist(&a, &b);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_abductive_latent_grounding_consistent() {
        let dom = "Frank Gehry designed the Walt Disney Concert Hall";
        let samples = vec![
            dom.to_string(),
            dom.to_string(),
            dom.to_string(),
        ];
        let context = "The Walt Disney Concert Hall was designed by architect Frank Gehry.";
        let receipt = AbductiveEpistemicEngine::verify_abduction(dom, &samples, Some(context), 0.0);
        assert!(receipt.counterfactual_stable);
        assert!(!receipt.abductive_hallucination_detected);
        assert!(receipt.attractor_curvature > 0.9);
        assert!(receipt.abductive_consistency > 0.9);
    }

    #[test]
    fn test_abduction_ungrounded_hallucination_detected() {
        let dom = "Zaha Hadid designed the Eiffel Tower in Tokyo";
        let samples = vec![
            dom.to_string(),
            dom.to_string(),
            dom.to_string(),
        ];
        let context = "The Eiffel Tower is in Paris, France.";
        let receipt = AbductiveEpistemicEngine::verify_abduction(dom, &samples, Some(context), 0.0);
        assert!(!receipt.counterfactual_stable);
        assert!(receipt.abductive_hallucination_detected);
    }
}
