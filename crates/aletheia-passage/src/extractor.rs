use regex::Regex;
use std::sync::LazyLock;

static ANSWER_TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)<answer>\s*(.*?)\s*</answer>").unwrap()
});

static UNCLOSED_ANSWER_TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)<answer>\s*([^<\n\r]+)").unwrap()
});

static BOXED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\boxed\{(.*?)\}").unwrap()
});

static UNCLOSED_BOXED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\boxed\{([^}\n\r]+)").unwrap()
});

static FINAL_ANSWER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:final answer is|therefore, the answer is|the answer is:?)\s*([^\n.]+)").unwrap()
});

pub struct AnswerExtractor;

impl AnswerExtractor {
    /// Extracts canonical answer and optional reasoning chain
    pub fn extract(raw_text: &str) -> (String, Option<String>) {
        let trimmed = raw_text.trim();

        // 1. Check <answer>...</answer> XML tags
        if let Some(caps) = ANSWER_TAG_REGEX.captures(trimmed) {
            let answer = caps[1].trim().to_string();
            let reasoning = ANSWER_TAG_REGEX.replace_all(trimmed, "").trim().to_string();
            return (answer, if reasoning.is_empty() { None } else { Some(reasoning) });
        }

        // 1b. Check unclosed <answer> XML tags (salvages generation truncated by token caps)
        if let Some(caps) = UNCLOSED_ANSWER_TAG_REGEX.captures(trimmed) {
            let answer = caps[1].trim().to_string();
            let reasoning = UNCLOSED_ANSWER_TAG_REGEX.replace_all(trimmed, "").trim().to_string();
            return (answer, if reasoning.is_empty() { None } else { Some(reasoning) });
        }

        // 2. Check LaTeX \boxed{...}
        if let Some(caps) = BOXED_REGEX.captures(trimmed) {
            let answer = caps[1].trim().to_string();
            return (answer, Some(trimmed.to_string()));
        }

        // 2b. Check unclosed LaTeX \boxed{... (salvages truncated formulas)
        if let Some(caps) = UNCLOSED_BOXED_REGEX.captures(trimmed) {
            let answer = caps[1].trim().to_string();
            return (answer, Some(trimmed.to_string()));
        }

        // 3. Check "the answer is ..." regex
        if let Some(caps) = FINAL_ANSWER_REGEX.captures(trimmed) {
            let answer = caps[1].trim().to_string();
            return (answer, Some(trimmed.to_string()));
        }

        // Fallback: If short (< 15 words), treat entire response as answer
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() <= 15 {
            (trimmed.to_string(), None)
        } else {
            // Take the last sentence as candidate answer
            let last_line = trimmed.lines().last().unwrap_or(trimmed).trim();
            (last_line.to_string(), Some(trimmed.to_string()))
        }
    }
}
