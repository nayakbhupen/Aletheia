pub mod aggregator;
pub mod arena;

pub use aggregator::{BenchmarkSummary, MetricsAggregator};
pub use arena::{ArenaEngine, FrameworkComparison};

#[cfg(test)]
mod tests {
    use super::*;
    use aletheia_core::EvaluationReceipt;

    #[test]
    fn test_truth_tensor_aggregation() {
        let receipts = vec![
            EvaluationReceipt {
                item_id: "1".to_string(),
                model_id: "test-model".to_string(),
                model_response: "A".to_string(),
                is_correct: true,
                grade_tier: "TIER_0_EXACT".to_string(),
                grade_confidence: 1.0,
                resonance_rsc: 0.0,
                epistemic_state: "CONSISTENT".to_string(),
                mode_collapse_detected: false,
                reasoning_chain_valid: true,
                latency_nanos: 500,
                abductive_consistency: 1.0,
                attractor_curvature: 1.0,
                truncation_detected: false,
            },
            EvaluationReceipt {
                item_id: "2".to_string(),
                model_id: "test-model".to_string(),
                model_response: "B".to_string(),
                is_correct: false,
                grade_tier: "TIER_FAILED".to_string(),
                grade_confidence: 0.2,
                resonance_rsc: 0.8,
                epistemic_state: "DISPERSION".to_string(),
                mode_collapse_detected: false,
                reasoning_chain_valid: false,
                latency_nanos: 600,
                abductive_consistency: 0.2,
                attractor_curvature: 0.1,
                truncation_detected: false,
            },
        ];

        let summary = MetricsAggregator::aggregate("test-model", &receipts);
        assert_eq!(summary.truth_tensor.factual_fidelity, 0.5);
        assert!(summary.truth_tensor.aletheia_score > 0.0);
    }
}
