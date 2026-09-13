pub mod abduction;
pub mod cadence;
pub mod calibration;
pub mod confluence;
pub mod invariants;
pub mod resonance;
pub mod resonance_wrapper;

pub use abduction::{AbductiveEpistemicEngine, AbductiveReceipt};
pub use cadence::{CadenceFracture, CadenceReceipt, CombinatorialCadenceEngine, MetricWeight};
pub use calibration::EpistemicCalibration;
pub use confluence::{ConfluenceStreamKind, EpistemicVector, TriadicConfluenceOperator, TriadicConfluenceReceipt, TriadicStatus};
pub use invariants::{BezoutIdentity, DiophantineInvariant, DiophantineInvariantSolver, InvariantVerificationReceipt};
pub use resonance::{PhaseSpaceResonanceEngine, PhaseResonanceAuditReceipt};
pub use resonance_wrapper::{EpistemicAuditor, EpistemicEvaluation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_resonance_submicrosecond_audit() {
        let auditor = EpistemicAuditor::default_threshold();
        let samples = vec![
            "42".to_string(),
            "42".to_string(),
            "The answer is 42".to_string(),
            "42".to_string(),
        ];
        let eval = auditor.audit_samples(&samples, None);
        assert!(eval.is_safe);
        assert_eq!(eval.rsc, 0.0);
        assert_eq!(eval.epistemic_state, "CONSISTENT");
    }

    #[test]
    fn test_epistemic_auroc_and_ece() {
        let confidences = vec![0.95, 0.90, 0.85, 0.20, 0.15];
        let correctness = vec![true, true, true, false, false];

        let auroc = EpistemicCalibration::compute_e_auroc(&confidences, &correctness);
        assert_eq!(auroc, 1.0);

        let ece = EpistemicCalibration::compute_ece(&confidences, &correctness, 10);
        assert!(ece < 0.2);

        let brier = EpistemicCalibration::compute_brier_score(&confidences, &correctness);
        assert!(brier < 0.05);
    }
}
