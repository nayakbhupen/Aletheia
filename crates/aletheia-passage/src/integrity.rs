use regex::Regex;
use std::sync::LazyLock;

static ABSTENTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(insufficient information|cannot be determined|not provided|not mentioned|unknown|unanswerable|not specified|does not state|cannot answer)\b").unwrap()
});

static TRUNCATION_ENDINGS: &[&str] = &[
    "...", "…", "and", "or", "the", "a", "an", "with", "of", "to", "in", "that", "which",
];

pub struct PassageIntegrityChecker;

impl PassageIntegrityChecker {
    /// Detects if a passage or model completion has been prematurely truncated
    pub fn is_truncated(passage: &str) -> bool {
        let trimmed = passage.trim();
        if trimmed.is_empty() {
            return true;
        }

        // 1. Explicit unclosed tags/braces indicating token cap cutoff
        if trimmed.contains("<answer>") && !trimmed.contains("</answer>") {
            return true;
        }
        let open_braces = trimmed.chars().filter(|&c| c == '{').count();
        let close_braces = trimmed.chars().filter(|&c| c == '}').count();
        if trimmed.contains(r"\boxed{") && open_braces > close_braces {
            return true;
        }

        // 2. Check dangling words
        for &ending in TRUNCATION_ENDINGS {
            if trimmed.ends_with(ending) {
                return true;
            }
        }

        // 3. Check unmatched brackets
        let open_parens = trimmed.chars().filter(|&c| c == '(').count();
        let close_parens = trimmed.chars().filter(|&c| c == ')').count();
        if open_parens != close_parens {
            return true;
        }

        // 4. Concise short entity completion (e.g. "Paris", "Frank Gehry", "42")
        // Short direct answers without punctuation are not truncated unless ending in dangling words
        if trimmed.split_whitespace().count() <= 3 && !trimmed.contains('\n') {
            return false;
        }

        // 5. Check trailing punctuation and valid closing delimiters
        let last_char = trimmed.chars().last().unwrap();
        if !['.', '!', '?', '"', '\'', '»', ')', ']', '}', '>'].contains(&last_char) {
            return true;
        }

        false
    }

    /// Check if response represents an abstention
    pub fn is_abstention(response: &str) -> bool {
        ABSTENTION_REGEX.is_match(response)
    }

    /// Verifies that passage contains key entities needed to derive answer
    pub fn contains_grounding_premise(passage: &str, ground_truth: &str) -> bool {
        let p_lower = passage.to_lowercase();
        let g_lower = ground_truth.to_lowercase();
        p_lower.contains(&g_lower)
    }
}
