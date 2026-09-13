use serde::{Deserialize, Serialize};

/// The Aletheia Truth Tensor: Three orthogonal axes of model veracity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TruthTensor {
    /// Axis 1: Factual Fidelity F in [0.0, 1.0] (deterministic grading accuracy)
    pub factual_fidelity: f64,
    /// Axis 2: Epistemic Calibration E in [0.0, 1.0] (correlation between Phase Resonance R_sc and truth)
    pub epistemic_calibration: f64,
    /// Axis 3: Adversarial Robustness A in [0.0, 1.0] (resistance to sycophancy & drift)
    pub adversarial_robustness: f64,
    /// Overall Aletheia Score: Geometric mean of F, E, A = (F * E * A)^(1/3)
    pub aletheia_score: f64,
    /// Detailed diagnostic metrics
    pub diagnostics: TensorDiagnostics,
}

impl TruthTensor {
    pub fn compute(
        f: f64,
        e: f64,
        a: f64,
        diagnostics: TensorDiagnostics,
    ) -> Self {
        let f_clamped = f.clamp(0.0001, 1.0);
        let e_clamped = e.clamp(0.0001, 1.0);
        let a_clamped = a.clamp(0.0001, 1.0);
        let score = (f_clamped * e_clamped * a_clamped).cbrt();

        Self {
            factual_fidelity: f,
            epistemic_calibration: e,
            adversarial_robustness: a,
            aletheia_score: score,
            diagnostics,
        }
    }
}

/// Fine-grained statistical diagnostics across all axes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TensorDiagnostics {
    pub standard_auroc: f64,
    pub epistemic_auroc: f64,
    pub expected_calibration_error: f64,
    pub brier_score: f64,
    pub mode_collapse_rate: f64,
    pub confident_hallucination_rate: f64,
    pub sycophancy_drift_slope: f64,
    pub total_evaluated_items: usize,
    pub mean_latency_micros: f64,
    #[serde(default)]
    pub mean_abductive_consistency: f64,
    #[serde(default)]
    pub mean_attractor_curvature: f64,
    #[serde(default)]
    pub truncation_rate: f64,
}

impl Default for TensorDiagnostics {
    fn default() -> Self {
        Self {
            standard_auroc: 0.5,
            epistemic_auroc: 0.5,
            expected_calibration_error: 0.0,
            brier_score: 0.25,
            mode_collapse_rate: 0.0,
            confident_hallucination_rate: 0.0,
            sycophancy_drift_slope: 0.0,
            total_evaluated_items: 0,
            mean_latency_micros: 0.0,
            mean_abductive_consistency: 1.0,
            mean_attractor_curvature: 1.0,
            truncation_rate: 0.0,
        }
    }
}

/// Audit receipt returned for an evaluated model completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReceipt {
    pub item_id: String,
    pub model_id: String,
    pub model_response: String,
    pub is_correct: bool,
    pub grade_tier: String,
    pub grade_confidence: f64,
    #[serde(alias = "spectral_rsc")]
    pub resonance_rsc: f64,
    pub epistemic_state: String,
    pub mode_collapse_detected: bool,
    pub reasoning_chain_valid: bool,
    pub latency_nanos: u128,
    #[serde(default)]
    pub abductive_consistency: f64,
    #[serde(default)]
    pub attractor_curvature: f64,
    #[serde(default)]
    pub truncation_detected: bool,
}

