use serde::{Deserialize, Serialize};
use std::fmt;

/// The 8 primary domains of Aletheia v∞
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Wikipedia,
    Wikidata,
    PubMed,
    ArXiv,
    Legal,
    Finance,
    Code,
    Dialogue,
}

impl Domain {
    pub fn all() -> &'static [Domain] {
        &[
            Domain::Wikipedia,
            Domain::Wikidata,
            Domain::PubMed,
            Domain::ArXiv,
            Domain::Legal,
            Domain::Finance,
            Domain::Code,
            Domain::Dialogue,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Domain::Wikipedia => "wikipedia",
            Domain::Wikidata => "wikidata",
            Domain::PubMed => "pubmed",
            Domain::ArXiv => "arxiv",
            Domain::Legal => "legal",
            Domain::Finance => "finance",
            Domain::Code => "code",
            Domain::Dialogue => "dialogue",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Domain::Wikipedia => "General knowledge, encyclopedic facts, historical chronologies",
            Domain::Wikidata => "Structured knowledge graph triples, entity relations, disambiguation",
            Domain::PubMed => "Biomedical facts, clinical trials, pharmacology, dosage thresholds",
            Domain::ArXiv => "Physics, mathematics, computer science proofs, quantitative theorems",
            Domain::Legal => "Statutory citations, precedents, jurisdiction boundaries, contract clauses",
            Domain::Finance => "SEC filings, quarterly earnings, balance sheets, fiscal ratios",
            Domain::Code => "API contracts, invariant bounds, type signatures, algorithmic semantics",
            Domain::Dialogue => "Conversational state, multi-turn consistency, persona preservation",
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
