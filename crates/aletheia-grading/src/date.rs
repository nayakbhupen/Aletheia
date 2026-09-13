use chrono::NaiveDate;
use regex::Regex;
use std::sync::LazyLock;

static ISO_DATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d{4})-(\d{2})-(\d{2})\b").unwrap()
});

static YEAR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d{1,4})\s*(BC|BCE|AD|CE)?\b").unwrap()
});

pub struct DateNormalizer;

impl DateNormalizer {
    /// Extract canonical year or ISO date string
    pub fn extract_canonical_date(text: &str) -> Option<String> {
        if let Some(caps) = ISO_DATE_REGEX.captures(text) {
            let y: i32 = caps[1].parse().ok()?;
            let m: u32 = caps[2].parse().ok()?;
            let d: u32 = caps[3].parse().ok()?;
            if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
                return Some(date.to_string());
            }
        }

        if let Some(caps) = YEAR_REGEX.captures(text) {
            let year = caps.get(1)?.as_str();
            let era = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            return Some(format!("{} {}", year, era).trim().to_string());
        }

        None
    }

    /// Compare candidate and truth dates for historical and temporal equivalence
    pub fn matches_date(candidate: &str, ground_truth: &str) -> bool {
        let cand_date = Self::extract_canonical_date(candidate);
        let truth_date = Self::extract_canonical_date(ground_truth);

        match (cand_date, truth_date) {
            (Some(c), Some(t)) => {
                if c == t {
                    true
                } else if t.len() <= 4 {
                    candidate.split(|ch: char| !ch.is_alphanumeric()).any(|w| w == t)
                } else {
                    candidate.contains(&t)
                }
            }
            _ => false,
        }
    }
}
