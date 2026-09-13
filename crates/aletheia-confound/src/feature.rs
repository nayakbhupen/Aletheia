use aletheia_core::SurfaceMetrics;
use regex::Regex;
use std::sync::LazyLock;

static HEDGE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(possibly|might|may|could|perhaps|allegedly|reportedly|presumably|supposedly|it is believed|it seems|likely|unlikely|appears to be|suggests that|arguably|tends to|indicates|potential|conceivably|hypothetically)\b").unwrap()
});

#[inline(always)]
fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | ';' | ':' | '!' | '?' | '\'' | '"' | '-' | '—' | '–' | '(' | ')' | '[' | ']' | '{' | '}'
    )
}

/// Extracted surface features of a text sample (Zero-heap allocation hot path)
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureVector {
    pub word_count: usize,
    pub char_count: usize,
    pub hedge_count: usize,
    pub hedge_density: f64,
    pub punct_count: usize,
    pub punct_density: f64,
    pub avg_word_length: f64,
}

impl FeatureVector {
    #[inline]
    pub fn extract(text: &str) -> Self {
        let mut char_count = 0usize;
        let mut punct_count = 0usize;

        for c in text.chars() {
            char_count += 1;
            if is_punctuation(c) {
                punct_count += 1;
            }
        }

        // Single-pass word count without allocating Vec
        let word_count = text.split_whitespace().count().max(1);
        let hedge_count = HEDGE_REGEX.find_iter(text).count();

        let hedge_density = hedge_count as f64 / word_count as f64;
        let punct_density = punct_count as f64 / char_count.max(1) as f64;
        let avg_word_length = char_count as f64 / word_count as f64;

        Self {
            word_count,
            char_count,
            hedge_count,
            hedge_density,
            punct_count,
            punct_density,
            avg_word_length,
        }
    }

    #[inline(always)]
    pub fn to_array(&self) -> [f64; 5] {
        [
            self.word_count as f64,
            self.char_count as f64,
            self.hedge_density,
            self.punct_density,
            self.avg_word_length,
        ]
    }
}

/// Compute surface metrics and isomorphism score between correct and hallucinated responses
#[inline]
pub fn compute_surface_metrics(correct: &str, hallucinated: &str) -> SurfaceMetrics {
    let f_c = FeatureVector::extract(correct);
    let f_h = FeatureVector::extract(hallucinated);

    let wc_ratio = if f_c.word_count > 0 {
        f_h.word_count as f64 / f_c.word_count as f64
    } else {
        1.0
    };

    let cc_ratio = if f_c.char_count > 0 {
        f_h.char_count as f64 / f_c.char_count as f64
    } else {
        1.0
    };

    let wc_diff = (f_c.word_count as f64 - f_h.word_count as f64).abs();
    let wc_max = f_c.word_count.max(f_h.word_count).max(1) as f64;
    let wc_sim = (1.0 - (wc_diff / wc_max)).max(0.0);

    let cc_diff = (f_c.char_count as f64 - f_h.char_count as f64).abs();
    let cc_max = f_c.char_count.max(f_h.char_count).max(1) as f64;
    let cc_sim = (1.0 - (cc_diff / cc_max)).max(0.0);

    let iso_score = (wc_sim * 0.6 + cc_sim * 0.4).clamp(0.0, 1.0);

    SurfaceMetrics {
        word_count_correct: f_c.word_count,
        word_count_hallucinated: f_h.word_count,
        word_count_ratio: wc_ratio,
        char_count_correct: f_c.char_count,
        char_count_hallucinated: f_h.char_count,
        char_count_ratio: cc_ratio,
        hedge_count_correct: f_c.hedge_count,
        hedge_count_hallucinated: f_h.hedge_count,
        punct_count_correct: f_c.punct_count,
        punct_count_hallucinated: f_h.punct_count,
        syntactic_isomorphism_score: iso_score,
    }
}
