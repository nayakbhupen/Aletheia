use crate::abduction::AbductiveEpistemicEngine;
use serde::{Deserialize, Serialize};

/// The three epistemic streams governing formal truth convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfluenceStreamKind {
    Empirical,
    Deductive,
    Abductive,
}

/// A 3D directional vector representing a stream's magnitude and orientation
/// in the epistemic state space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EpistemicVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EpistemicVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag < 1e-12 {
            Self::new(0.0, 0.0, 0.0)
        } else {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

/// Qualitative status of the Triadic Confluence operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriadicStatus {
    TriadicConsensus,
    EmpiricalDeficit,
    AbductiveRupture,
    DeductiveSlip,
    EpistemicVoid,
}

/// Comprehensive receipt produced by the Triadic Confluence Operator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriadicConfluenceReceipt {
    pub empirical_score: f64,
    pub deductive_score: f64,
    pub abductive_score: f64,
    pub confluence_volume: f64,
    pub consensus_score: f64,
    pub asymmetry_index: f64,
    pub status: TriadicStatus,
    pub latency_nanos: u64,
}

/// High-performance 3-stream Triadic Confluence Operator.
/// Unites empirical grounding, deductive momentum, and abductive necessity
/// via differential determinant volume calculations.
#[derive(Debug, Clone, Default)]
pub struct TriadicConfluenceOperator;

impl TriadicConfluenceOperator {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates the 3-stream confluence for candidate reasoning against optional premise.
    pub fn evaluate(reasoning_text: &str, premise_text: Option<&str>) -> TriadicConfluenceReceipt {
        let t_start = std::time::Instant::now();

        // 1. Empirical Grounding Stream (Φ_empirical)
        let empirical_score = if let Some(premise) = premise_text {
            let p_words: std::collections::HashSet<&str> = premise.split_whitespace().collect();
            if p_words.is_empty() {
                0.5
            } else {
                let r_words: Vec<&str> = reasoning_text.split_whitespace().collect();
                if r_words.is_empty() {
                    0.0
                } else {
                    let overlap = r_words.iter().filter(|w| p_words.contains(*w)).count();
                    (overlap as f64 / r_words.len() as f64).min(1.0)
                }
            }
        } else {
            // Default baseline empirical presence from token diversity
            let words: Vec<&str> = reasoning_text.split_whitespace().collect();
            if words.len() < 3 { 0.2 } else { 0.85 }
        };

        // 2. Deductive Flow Stream (Φ_deductive)
        // Measures connective velocity and sequential forward progress
        let sentences: Vec<&str> = reasoning_text.split(|c| c == '.' || c == '\n').filter(|s| !s.trim().is_empty()).collect();
        let deductive_score = if sentences.len() <= 1 {
            0.6
        } else {
            let connectors = ["therefore", "because", "hence", "thus", "since", "consequently", "implies", "leads to"];
            let lower = reasoning_text.to_lowercase();
            let conn_count = connectors.iter().filter(|c| lower.contains(*c)).count();
            let flow = 0.5 + (conn_count as f64 * 0.15).min(0.5);
            flow.min(1.0)
        };

        // 3. Abductive Latent Grounding Stream (Φ_abductive)
        let sample_vec = [reasoning_text.to_string()];
        let abductive_receipt = AbductiveEpistemicEngine::verify_abduction(
            reasoning_text,
            &sample_vec,
            premise_text,
            0.1,
        );
        let abductive_score = abductive_receipt.abductive_consistency;

        // 4. Compute Triadic Vectors in 3D Epistemic Space
        let v_emp = EpistemicVector::new(empirical_score, 0.2, 0.1).normalize();
        let v_ded = EpistemicVector::new(0.1, deductive_score, 0.2).normalize();
        let v_abd = EpistemicVector::new(0.2, 0.1, abductive_score).normalize();

        // 5. Compute Confluence Volume via Scalar Triple Product: u_e · (u_d × u_a)
        let cross = v_ded.cross(&v_abd);
        let confluence_volume = (v_emp.dot(&cross)).abs();

        // 6. Compute Stream Asymmetry Index
        let mean = (empirical_score + deductive_score + abductive_score) / 3.0;
        let var = ((empirical_score - mean).powi(2) + (deductive_score - mean).powi(2) + (abductive_score - mean).powi(2)) / 3.0;
        let asymmetry_index = var.sqrt();

        // 7. Consensus Score
        let consensus_score = (mean * (1.0 - asymmetry_index)).max(0.0).min(1.0);

        // 8. State Classification
        let status = if empirical_score < 0.25 {
            TriadicStatus::EmpiricalDeficit
        } else if abductive_score < 0.25 {
            TriadicStatus::AbductiveRupture
        } else if deductive_score < 0.30 {
            TriadicStatus::DeductiveSlip
        } else if consensus_score >= 0.35 && confluence_volume > 0.05 {
            TriadicStatus::TriadicConsensus
        } else {
            TriadicStatus::EpistemicVoid
        };

        TriadicConfluenceReceipt {
            empirical_score,
            deductive_score,
            abductive_score,
            confluence_volume,
            consensus_score,
            asymmetry_index,
            status,
            latency_nanos: t_start.elapsed().as_nanos() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epistemic_vector_operations() {
        let v1 = EpistemicVector::new(1.0, 0.0, 0.0);
        let v2 = EpistemicVector::new(0.0, 1.0, 0.0);
        let v3 = EpistemicVector::new(0.0, 0.0, 1.0);

        assert_eq!(v1.dot(&v2), 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, v3);
        assert_eq!(v1.dot(&v2.cross(&v3)), 1.0);
    }

    #[test]
    fn test_triadic_consensus_on_sound_reasoning() {
        let premise = "The Apollo 11 spacecraft launched on July 16, 1969 from Kennedy Space Center.";
        let reasoning = "Apollo 11 launched in July 1969 from Kennedy Space Center. Therefore, the lunar mission was initiated in Florida.";

        let receipt = TriadicConfluenceOperator::evaluate(reasoning, Some(premise));
        assert!(receipt.empirical_score > 0.2);
        assert!(receipt.deductive_score >= 0.6);
        assert!(receipt.abductive_score > 0.3);
        assert!(receipt.consensus_score > 0.4);
    }

    #[test]
    fn test_empirical_deficit_detection() {
        let premise = "The water temperature in the lake is 15 degrees Celsius.";
        let reasoning = "Interstellar warp drives utilize antimatter plasma conduits. Therefore, galactic traversal occurs instantaneously.";

        let receipt = TriadicConfluenceOperator::evaluate(reasoning, Some(premise));
        assert_eq!(receipt.status, TriadicStatus::EmpiricalDeficit);
    }
}
