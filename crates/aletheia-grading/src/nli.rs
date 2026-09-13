use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

static NEGATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(not|never|no|none|neither|nor|cannot|isn't|aren't|wasn't|weren't|doesn't|don't|didn't|won't|wouldn't|shouldn't|hardly|scarcely)\b").unwrap()
});

static STOPWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
        "in", "on", "at", "to", "for", "with", "by", "about", "against",
        "between", "into", "through", "during", "before", "after", "above",
        "below", "from", "up", "down", "of", "off", "over", "under", "again",
        "further", "then", "once", "here", "there", "when", "where", "why",
        "how", "all", "any", "both", "each", "few", "more", "most", "other",
        "some", "such", "than", "too", "very", "s", "t", "can", "will", "just",
    ]
    .iter()
    .cloned()
    .collect()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NliVerdict {
    Entailment,
    Contradiction,
    Neutral,
}

pub struct DeterministicNli;

impl DeterministicNli {
    /// Tokenize and clean text into lowercase non-stopword tokens (Zero redundant hashset allocation)
    pub fn tokenize(text: &str) -> HashSet<String> {
        let mut tokens = HashSet::with_capacity(32);
        for word in text.split(|c: char| !c.is_alphanumeric()) {
            if !word.is_empty() {
                let lower = word.to_lowercase();
                if !STOPWORDS.contains(lower.as_str()) {
                    tokens.insert(lower);
                }
            }
        }
        tokens
    }

    /// Check if negation polarity matches between premise and hypothesis
    #[inline(always)]
    pub fn has_negation_clash(premise: &str, hypothesis: &str) -> bool {
        let p_neg = NEGATION_REGEX.is_match(premise);
        let h_neg = NEGATION_REGEX.is_match(hypothesis);
        p_neg != h_neg
    }

    /// Evaluate entailment between reference passage and candidate response
    pub fn evaluate_entailment(premise: &str, hypothesis: &str) -> (NliVerdict, f64) {
        let p_tokens = Self::tokenize(premise);
        let h_tokens = Self::tokenize(hypothesis);

        if h_tokens.is_empty() {
            return (NliVerdict::Neutral, 0.0);
        }

        let intersection = h_tokens.intersection(&p_tokens).count();
        let recall = intersection as f64 / h_tokens.len() as f64;

        if Self::has_negation_clash(premise, hypothesis) && recall > 0.4 {
            return (NliVerdict::Contradiction, 0.95);
        }

        if recall >= 0.70 {
            (NliVerdict::Entailment, recall)
        } else if recall <= 0.20 {
            (NliVerdict::Contradiction, 1.0 - recall)
        } else {
            (NliVerdict::Neutral, recall)
        }
    }
}
