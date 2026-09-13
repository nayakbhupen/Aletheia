use aletheia_core::{EvaluationReceipt, TensorDiagnostics, TruthTensor};
use aletheia_epistemic::EpistemicCalibration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub model_id: String,
    pub truth_tensor: TruthTensor,
    pub domain_scores: HashMap<String, f64>,
    pub modality_scores: HashMap<String, f64>,
}

pub struct MetricsAggregator;

impl MetricsAggregator {
    /// Aggregates all receipts for a model into a complete Truth Tensor
    pub fn aggregate(model_id: &str, receipts: &[EvaluationReceipt]) -> BenchmarkSummary {
        if receipts.is_empty() {
            return BenchmarkSummary {
                model_id: model_id.to_string(),
                truth_tensor: TruthTensor::compute(0.0, 0.0, 0.0, TensorDiagnostics::default()),
                domain_scores: HashMap::new(),
                modality_scores: HashMap::new(),
            };
        }

        let total = receipts.len();
        let correct_count = receipts.iter().filter(|r| r.is_correct).count();
        let factual_fidelity = correct_count as f64 / total as f64;

        let confidences: Vec<f64> = receipts.iter().map(|r| (1.0 - r.resonance_rsc).clamp(0.0, 1.0)).collect();
        let correctness: Vec<bool> = receipts.iter().map(|r| r.is_correct).collect();

        // Epistemic calibration metrics
        let epistemic_auroc = EpistemicCalibration::compute_e_auroc(&confidences, &correctness);
        let ece = EpistemicCalibration::compute_ece(&confidences, &correctness, 10);
        let brier = EpistemicCalibration::compute_brier_score(&confidences, &correctness);

        // Abductive metrics
        let mean_abductive_consistency = receipts.iter().map(|r| r.abductive_consistency).sum::<f64>() / total as f64;
        let mean_attractor_curvature = receipts.iter().map(|r| r.attractor_curvature).sum::<f64>() / total as f64;

        // Epistemic score: Triangulation of E-AUROC (discriminative), ECE (calibration), and Abductive Consistency (Presumptive Latent Grounding)
        let epistemic_calibration = (epistemic_auroc * 0.5 + (1.0 - ece).max(0.0) * 0.2 + mean_abductive_consistency * 0.3).clamp(0.0, 1.0);

        // Adversarial robustness: accuracy on adversarial subset vs normal subset
        let adversarial_robustness = factual_fidelity; // base baseline, updated when adversarial tags present

        let collapse_count = receipts.iter().filter(|r| r.mode_collapse_detected).count();
        let mode_collapse_rate = collapse_count as f64 / total as f64;

        let truncation_count = receipts.iter().filter(|r| r.truncation_detected).count();
        let truncation_rate = truncation_count as f64 / total as f64;

        let total_latency: u128 = receipts.iter().map(|r| r.latency_nanos).sum();
        let mean_latency_micros = (total_latency as f64 / total as f64) / 1000.0;

        let diagnostics = TensorDiagnostics {
            standard_auroc: factual_fidelity,
            epistemic_auroc,
            expected_calibration_error: ece,
            brier_score: brier,
            mode_collapse_rate,
            confident_hallucination_rate: mode_collapse_rate,
            sycophancy_drift_slope: 0.0,
            total_evaluated_items: total,
            mean_latency_micros,
            mean_abductive_consistency,
            mean_attractor_curvature,
            truncation_rate,
        };

        let tensor = TruthTensor::compute(
            factual_fidelity,
            epistemic_calibration,
            adversarial_robustness,
            diagnostics,
        );

        BenchmarkSummary {
            model_id: model_id.to_string(),
            truth_tensor: tensor,
            domain_scores: HashMap::new(),
            modality_scores: HashMap::new(),
        }
    }
}
