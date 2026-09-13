use serde::{Deserialize, Serialize};

/// Discrete binary metric state of an informational token (Light vs Heavy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricWeight {
    Light = 0, // Functional, syntactic connective, or concise token (<= 4 chars)
    Heavy = 1, // Substantive lexical entity, technical term, or compound token (> 4 chars)
}

/// A detected cadence fracture indicating an anomalous rhythmic shift
/// in the token transition lattice, often preceding a hallucination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceFracture {
    pub step_index: usize,
    pub prior_entropy: f64,
    pub post_entropy: f64,
    pub entropy_delta: f64,
    pub severity: f64,
}

/// Verification receipt returned by the Combinatorial Cadence Engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceReceipt {
    pub total_tokens: usize,
    pub light_fraction: f64,
    pub heavy_fraction: f64,
    pub transition_entropy: f64,
    pub cadence_fractures: Vec<CadenceFracture>,
    pub is_harmonious: bool,
    pub latency_nanos: u64,
}

/// High-throughput Combinatorial Cadence Engine.
/// Computes discrete binary state transition matrices and flags cadence fractures
/// across reasoning steps in sub-microseconds.
#[derive(Debug, Clone, Default)]
pub struct CombinatorialCadenceEngine;

impl CombinatorialCadenceEngine {
    pub fn new() -> Self {
        Self
    }

    /// Classifies a token into its discrete binary metric weight.
    #[inline(always)]
    pub fn classify_token(token: &str) -> MetricWeight {
        if token.trim().len() <= 4 {
            MetricWeight::Light
        } else {
            MetricWeight::Heavy
        }
    }

    /// Analyzes a text stream and computes its discrete transition lattice and cadence fractures.
    pub fn analyze(text: &str) -> CadenceReceipt {
        let t_start = std::time::Instant::now();
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.is_empty() {
            return CadenceReceipt {
                total_tokens: 0,
                light_fraction: 0.0,
                heavy_fraction: 0.0,
                transition_entropy: 0.0,
                cadence_fractures: Vec::new(),
                is_harmonious: true,
                latency_nanos: t_start.elapsed().as_nanos() as u64,
            };
        }

        let weights: Vec<MetricWeight> = words.iter().map(|w| Self::classify_token(w)).collect();
        let total = weights.len();
        let light_count = weights.iter().filter(|&&w| w == MetricWeight::Light).count();
        let heavy_count = total - light_count;

        let light_fraction = light_count as f64 / total as f64;
        let heavy_fraction = heavy_count as f64 / total as f64;

        // Compute 2x2 state transition matrix: T[from][to]
        let mut transitions = [[0usize; 2], [0usize; 2]];
        for window in weights.windows(2) {
            let from = window[0] as usize;
            let to = window[1] as usize;
            transitions[from][to] += 1;
        }

        // Compute transition entropy: H = - sum(pi_i * sum(P_ij * log2(P_ij)))
        let mut transition_entropy = 0.0;
        let pi = [light_fraction, heavy_fraction];

        for i in 0..2 {
            let row_sum: usize = transitions[i].iter().sum();
            if row_sum > 0 && pi[i] > 0.0 {
                let mut row_entropy = 0.0;
                for j in 0..2 {
                    if transitions[i][j] > 0 {
                        let p = transitions[i][j] as f64 / row_sum as f64;
                        row_entropy -= p * p.log2();
                    }
                }
                transition_entropy += pi[i] * row_entropy;
            }
        }

        // Detect Cadence Fractures across sentences / step windows
        let steps: Vec<&str> = text.split(|c| c == '.' || c == '\n')
            .filter(|s| !s.trim().is_empty())
            .collect();

        let mut fractures = Vec::new();
        if steps.len() >= 3 {
            let mut step_entropies = Vec::with_capacity(steps.len());
            for s in &steps {
                let s_words: Vec<&str> = s.split_whitespace().collect();
                if s_words.len() >= 2 {
                    let s_receipt = Self::analyze_slice(&s_words);
                    step_entropies.push(s_receipt);
                } else {
                    step_entropies.push(0.5);
                }
            }

            for i in 1..step_entropies.len() {
                let delta = (step_entropies[i] - step_entropies[i - 1]).abs();
                if delta > 0.45 {
                    fractures.push(CadenceFracture {
                        step_index: i + 1,
                        prior_entropy: step_entropies[i - 1],
                        post_entropy: step_entropies[i],
                        entropy_delta: delta,
                        severity: (delta - 0.45) / 0.55,
                    });
                }
            }
        }

        let is_harmonious = fractures.is_empty() && transition_entropy > 0.15;

        CadenceReceipt {
            total_tokens: total,
            light_fraction,
            heavy_fraction,
            transition_entropy,
            cadence_fractures: fractures,
            is_harmonious,
            latency_nanos: t_start.elapsed().as_nanos() as u64,
        }
    }

    fn analyze_slice(words: &[&str]) -> f64 {
        if words.len() < 2 {
            return 0.5;
        }
        let weights: Vec<MetricWeight> = words.iter().map(|w| Self::classify_token(w)).collect();
        let mut t = [[0usize; 2], [0usize; 2]];
        for w in weights.windows(2) {
            t[w[0] as usize][w[1] as usize] += 1;
        }
        let light = weights.iter().filter(|&&w| w == MetricWeight::Light).count() as f64 / weights.len() as f64;
        let heavy = 1.0 - light;
        let mut ent = 0.0;
        let pi = [light, heavy];

        for i in 0..2 {
            let row_sum: usize = t[i].iter().sum();
            if row_sum > 0 {
                for j in 0..2 {
                    if t[i][j] > 0 {
                        let p = t[i][j] as f64 / row_sum as f64;
                        ent -= pi[i] * p * p.log2();
                    }
                }
            }
        }
        ent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_classification() {
        assert_eq!(CombinatorialCadenceEngine::classify_token("the"), MetricWeight::Light);
        assert_eq!(CombinatorialCadenceEngine::classify_token("is"), MetricWeight::Light);
        assert_eq!(CombinatorialCadenceEngine::classify_token("thermodynamics"), MetricWeight::Heavy);
        assert_eq!(CombinatorialCadenceEngine::classify_token("acceleration"), MetricWeight::Heavy);
    }

    #[test]
    fn test_harmonious_rhythm() {
        let text = "The theory of quantum electrodynamics provides a rigorous mathematical framework for physics.";
        let receipt = CombinatorialCadenceEngine::analyze(text);
        assert!(receipt.total_tokens > 5);
        assert!(receipt.transition_entropy > 0.0);
        assert!(receipt.is_harmonious);
    }

    #[test]
    fn test_cadence_fracture_detection() {
        // Step 1 & 2 are balanced English prose. Step 3 is a wild structural anomaly.
        let text = "The rapid evolution of computer hardware enabled modern artificial intelligence.
Researchers developed sophisticated deep neural networks for visual recognition.
XYZ123456789 ABC987654321 QWERTYUIOPASDFGHJKLZXCVBNM 99999999999999999999999999.
Therefore the conclusion remains validated.";

        let receipt = CombinatorialCadenceEngine::analyze(text);
        assert!(!receipt.cadence_fractures.is_empty());
    }
}
