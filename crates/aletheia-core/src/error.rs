use thiserror::Error;

#[derive(Error, Debug)]
pub enum AletheiaError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Confound gate failed: {metric} value {value:.4} exceeds limit {threshold:.4}")]
    ConfoundGateViolation {
        metric: String,
        value: f64,
        threshold: f64,
    },

    #[error("Grading error: {0}")]
    Grading(String),

    #[error("Wikidata lookup error: {0}")]
    Wikidata(String),

    #[error("Model inference error: {0}")]
    ModelInference(String),

    #[error("Epistemic evaluation error: {0}")]
    Epistemic(String),

    #[error("Passage integrity error: {0}")]
    PassageIntegrity(String),

    #[error("General error: {0}")]
    General(String),
}

pub type Result<T> = std::result::Result<T, AletheiaError>;
