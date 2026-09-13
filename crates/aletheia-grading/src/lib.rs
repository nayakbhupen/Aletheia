pub mod cascade;
pub mod date;
pub mod nli;
pub mod numeric;
pub mod wikidata;

pub use cascade::{GradingCascade, GradingDecision, GradingTier};
pub use date::DateNormalizer;
pub use nli::{DeterministicNli, NliVerdict};
pub use numeric::NumericNormalizer;
pub use wikidata::{EntityDetails, WikidataResolver};

#[cfg(test)]
mod tests {
    use super::*;
    use aletheia_core::{BenchmarkItem, Domain, Modality, QidGrounding};

    #[test]
    fn test_numeric_normalizer() {
        assert!(NumericNormalizer::is_equivalent(100.0, 100.5, 0.01));
        assert!(!NumericNormalizer::is_equivalent(100.0, 105.0, 0.01));
        assert!(NumericNormalizer::matches_numeric_target("The density is 3.14 g/cm3", "3.14", 0.01));
    }

    #[test]
    fn test_date_normalizer() {
        assert_eq!(DateNormalizer::extract_canonical_date("Born on 1949-05-23 in Bonn"), Some("1949-05-23".to_string()));
        assert!(DateNormalizer::matches_date("Event occurred in 1949", "1949"));
    }

    #[test]
    fn test_nli_entailment() {
        let (verdict, score) = DeterministicNli::evaluate_entailment(
            "Penicillin inhibits bacterial cell wall synthesis.",
            "Penicillin stops bacterial cell wall synthesis.",
        );
        assert_eq!(verdict, NliVerdict::Entailment);
        assert!(score > 0.6);
    }

    #[test]
    fn test_grading_cascade() {
        let cascade = GradingCascade::in_memory().unwrap();
        let mut item = BenchmarkItem::new(
            "test-1",
            1,
            Domain::Wikipedia,
            Modality::Fabrication,
            "Who designed Guggenheim Bilbao?",
            "Designed by Frank Gehry.",
            "Frank Gehry",
            "Frank Gehry",
            "Zaha Hadid",
        );
        item.target_answer = "Frank Gehry".to_string();
        item.qid_grounding = Some(QidGrounding {
            subject_qid: "Q132993".to_string(),
            subject_label: "Guggenheim Museum Bilbao".to_string(),
            property_pid: "P84".to_string(),
            property_label: "architect".to_string(),
            object_qid: None,
            object_value: "Frank Gehry".to_string(),
            aliases: vec!["Gehry".to_string(), "Frank Owen Gehry".to_string()],
        });

        let res = cascade.grade(&item, "The architect was Frank Owen Gehry");
        assert!(res.is_correct);
    }
}
