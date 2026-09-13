use regex::Regex;
use std::sync::LazyLock;

static NUMERIC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?").unwrap()
});

pub struct NumericNormalizer;

impl NumericNormalizer {
    /// Extract all numbers from a string as f64
    pub fn extract_numbers(text: &str) -> Vec<f64> {
        NUMERIC_REGEX
            .find_iter(text)
            .filter_map(|m| m.as_str().parse::<f64>().ok())
            .collect()
    }

    /// Check if two numbers are equivalent within a relative tolerance (default 1%)
    pub fn is_equivalent(a: f64, b: f64, rel_tolerance: f64) -> bool {
        if !a.is_finite() || !b.is_finite() {
            return false;
        }

        if (a - b).abs() < 1e-9 {
            return true;
        }

        let max_abs = a.abs().max(b.abs());
        if max_abs < 1e-9 {
            return true;
        }

        ((a - b).abs() / max_abs) <= rel_tolerance
    }

    /// Check if target numbers match candidate numbers
    pub fn matches_numeric_target(candidate: &str, ground_truth: &str, tolerance: f64) -> bool {
        let cand_nums = Self::extract_numbers(candidate);
        let truth_nums = Self::extract_numbers(ground_truth);

        if truth_nums.is_empty() {
            return false;
        }

        // If candidate contains all target numbers within tolerance
        truth_nums.iter().all(|&t| {
            cand_nums.iter().any(|&c| Self::is_equivalent(c, t, tolerance))
        })
    }
}
