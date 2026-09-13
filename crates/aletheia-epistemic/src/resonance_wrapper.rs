use crate::abduction::AbductiveEpistemicEngine;
use crate::resonance::PhaseSpaceResonanceEngine as NativeResonanceEngine;

#[derive(Debug, Clone)]
pub struct EpistemicEvaluation {
    pub rsc: f64,
    pub epistemic_state: String,
    pub is_safe: bool,
    pub dominant_answer: String,
    pub agreement_ratio: f64,
    pub attractor_collapse: bool,
    pub attractor_score: f64,
    pub latency_micros: f64,
    pub abductive_consistency: f64,
    pub attractor_curvature: f64,
    pub counterfactual_stable: bool,
    pub abductive_hallucination: bool,
}

pub struct EpistemicAuditor {
    threshold: f64,
}

impl EpistemicAuditor {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn default_threshold() -> Self {
        Self { threshold: 0.35 }
    }

    /// Run sub-microsecond Phase-Space Resonance and Abductive Latent Grounding evaluation on K generated samples
    pub fn audit_samples(&self, samples: &[String], context: Option<&str>) -> EpistemicEvaluation {
        let receipt = NativeResonanceEngine::evaluate(samples, context, self.threshold);

        let abductive = AbductiveEpistemicEngine::verify_abduction(
            &receipt.dominant_answer,
            samples,
            context,
            receipt.rsc,
        );

        let collapsed = receipt.attractor_collapse || abductive.abductive_hallucination_detected;

        EpistemicEvaluation {
            rsc: receipt.rsc,
            epistemic_state: receipt.state_name,
            is_safe: receipt.is_safe && !abductive.abductive_hallucination_detected,
            dominant_answer: receipt.dominant_answer,
            agreement_ratio: receipt.agreement_ratio,
            attractor_collapse: collapsed,
            attractor_score: abductive.attractor_curvature,
            latency_micros: receipt.latency_micros,
            abductive_consistency: abductive.abductive_consistency,
            attractor_curvature: abductive.attractor_curvature,
            counterfactual_stable: abductive.counterfactual_stable,
            abductive_hallucination: abductive.abductive_hallucination_detected,
        }
    }
}
