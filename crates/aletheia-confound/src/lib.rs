pub mod auroc_gate;
pub mod feature;
pub mod gate;
pub mod jsd;

pub use auroc_gate::AurocGate;
pub use feature::{compute_surface_metrics, FeatureVector};
pub use gate::{ConfoundEngine, ConfoundGateReport};
pub use jsd::JsdGate;

#[cfg(test)]
mod tests {
    use super::*;
    use aletheia_core::{BenchmarkItem, Domain, Modality};

    #[test]
    fn test_isomorphic_pairs_pass_gate() {
        let mut items = Vec::new();
        for i in 0..50 {
            let correct = format!("The chemical element with atomic number {} is verified.", i);
            let hallucinated = format!("The chemical element with atomic number {} is computed.", i + 100);
            items.push(BenchmarkItem::new(
                format!("item-{}", i),
                i,
                Domain::Wikipedia,
                Modality::Fabrication,
                "What element?",
                "Verified passage with elements.",
                "verified",
                correct,
                hallucinated,
            ));
        }

        let engine = ConfoundEngine::new(0.55, 0.05);
        let report = engine.audit_dataset(&items).unwrap();
        assert!(report.mean_isomorphism_score > 0.8);
    }
}
