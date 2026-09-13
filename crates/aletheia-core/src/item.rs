use crate::domain::Domain;
use crate::modality::Modality;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Surface-level syntactic and lexical metrics computed on text pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurfaceMetrics {
    pub word_count_correct: usize,
    pub word_count_hallucinated: usize,
    pub word_count_ratio: f64,
    pub char_count_correct: usize,
    pub char_count_hallucinated: usize,
    pub char_count_ratio: f64,
    pub hedge_count_correct: usize,
    pub hedge_count_hallucinated: usize,
    pub punct_count_correct: usize,
    pub punct_count_hallucinated: usize,
    pub syntactic_isomorphism_score: f64,
}

impl Default for SurfaceMetrics {
    fn default() -> Self {
        Self {
            word_count_correct: 0,
            word_count_hallucinated: 0,
            word_count_ratio: 1.0,
            char_count_correct: 0,
            char_count_hallucinated: 0,
            char_count_ratio: 1.0,
            hedge_count_correct: 0,
            hedge_count_hallucinated: 0,
            punct_count_correct: 0,
            punct_count_hallucinated: 0,
            syntactic_isomorphism_score: 1.0,
        }
    }
}

/// Wikidata knowledge grounding for deterministic grading
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QidGrounding {
    pub subject_qid: String,
    pub subject_label: String,
    pub property_pid: String,
    pub property_label: String,
    pub object_qid: Option<String>,
    pub object_value: String,
    pub aliases: Vec<String>,
}

/// A single atomic item in the Aletheia v∞ Benchmark (25,000 items target)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkItem {
    pub id: String,
    pub index: usize,
    pub domain: Domain,
    pub modality: Modality,
    pub language: String,
    pub prompt: String,
    pub passage: String,
    pub ground_truth: String,
    pub correct_response: String,
    pub hallucinated_response: String,
    pub target_answer: String,
    pub qid_grounding: Option<QidGrounding>,
    pub surface_metrics: SurfaceMetrics,
    pub is_adversarial: bool,
    pub adversarial_type: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl BenchmarkItem {
    pub fn new(
        id: impl Into<String>,
        index: usize,
        domain: Domain,
        modality: Modality,
        prompt: impl Into<String>,
        passage: impl Into<String>,
        ground_truth: impl Into<String>,
        correct_response: impl Into<String>,
        hallucinated_response: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            index,
            domain,
            modality,
            language: "en".to_string(),
            prompt: prompt.into(),
            passage: passage.into(),
            ground_truth: ground_truth.into(),
            correct_response: correct_response.into(),
            hallucinated_response: hallucinated_response.into(),
            target_answer: String::new(),
            qid_grounding: None,
            surface_metrics: SurfaceMetrics::default(),
            is_adversarial: false,
            adversarial_type: None,
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}
