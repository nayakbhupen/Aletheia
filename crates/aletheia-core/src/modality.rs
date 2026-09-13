use serde::{Deserialize, Serialize};
use std::fmt;

/// The 5 fundamental modalities of hallucination in the Aletheia taxonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    /// Pure invention of non-existent entities, citations, papers, or properties
    Fabrication,
    /// Direct logical or factual negation of a verified premise in the source
    Contradiction,
    /// Attribute swapping: authentic entities paired with each other's actual attributes
    Transmutation,
    /// Temporal or logical sequence reversal (A caused B becomes B caused A)
    CausalInversion,
    /// Critical qualification drop that turns a nuanced truth into a dangerous untruth
    Omission,
}

impl Modality {
    pub fn all() -> &'static [Modality] {
        &[
            Modality::Fabrication,
            Modality::Contradiction,
            Modality::Transmutation,
            Modality::CausalInversion,
            Modality::Omission,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Modality::Fabrication => "fabrication",
            Modality::Contradiction => "contradiction",
            Modality::Transmutation => "transmutation",
            Modality::CausalInversion => "causal_inversion",
            Modality::Omission => "omission",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Modality::Fabrication => "Invention of non-existent entities, bogus citations, fictitious metrics",
            Modality::Contradiction => "Direct factual clash against the verified reference passage",
            Modality::Transmutation => "Swapping relations/entities between real items (hardest to detect)",
            Modality::CausalInversion => "Inverting antecedent and consequent in directional relationships",
            Modality::Omission => "Dropping vital scope constraints, conditionals, or safety disclaimers",
        }
    }
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
