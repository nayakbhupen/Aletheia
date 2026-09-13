use crate::auroc_gate::AurocGate;
use crate::feature::{compute_surface_metrics, FeatureVector};
use crate::jsd::JsdGate;
use aletheia_core::{BenchmarkItem, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfoundGateReport {
    pub passed: bool,
    pub logistic_auroc: f64,
    pub max_allowed_auroc: f64,
    pub word_count_jsd: f64,
    pub char_count_jsd: f64,
    pub hedge_density_jsd: f64,
    pub punct_density_jsd: f64,
    pub max_allowed_jsd: f64,
    pub total_pairs_evaluated: usize,
    pub mean_isomorphism_score: f64,
}

pub struct ConfoundEngine {
    pub max_auroc: f64,
    pub max_jsd: f64,
}

impl Default for ConfoundEngine {
    fn default() -> Self {
        Self {
            max_auroc: 0.505,
            max_jsd: 0.001,
        }
    }
}

impl ConfoundEngine {
    pub fn new(max_auroc: f64, max_jsd: f64) -> Self {
        Self { max_auroc, max_jsd }
    }

    /// Audit a collection of benchmark items to verify zero surface confounds
    pub fn audit_dataset(&self, items: &[BenchmarkItem]) -> Result<ConfoundGateReport> {
        if items.is_empty() {
            return Ok(ConfoundGateReport {
                passed: true,
                logistic_auroc: 0.5,
                max_allowed_auroc: self.max_auroc,
                word_count_jsd: 0.0,
                char_count_jsd: 0.0,
                hedge_density_jsd: 0.0,
                punct_density_jsd: 0.0,
                max_allowed_jsd: self.max_jsd,
                total_pairs_evaluated: 0,
                mean_isomorphism_score: 1.0,
            });
        }

        let mut c_words = Vec::with_capacity(items.len());
        let mut h_words = Vec::with_capacity(items.len());
        let mut c_chars = Vec::with_capacity(items.len());
        let mut h_chars = Vec::with_capacity(items.len());
        let mut c_hedges = Vec::with_capacity(items.len());
        let mut h_hedges = Vec::with_capacity(items.len());
        let mut c_puncts = Vec::with_capacity(items.len());
        let mut h_puncts = Vec::with_capacity(items.len());

        let mut features = Vec::with_capacity(items.len() * 2);
        let mut labels = Vec::with_capacity(items.len() * 2);
        let mut iso_sum = 0.0;

        for item in items {
            let fc = FeatureVector::extract(&item.correct_response);
            let fh = FeatureVector::extract(&item.hallucinated_response);

            let m = compute_surface_metrics(&item.correct_response, &item.hallucinated_response);
            iso_sum += m.syntactic_isomorphism_score;

            c_words.push(fc.word_count as f64);
            h_words.push(fh.word_count as f64);

            c_chars.push(fc.char_count as f64);
            h_chars.push(fh.char_count as f64);

            c_hedges.push(fc.hedge_density);
            h_hedges.push(fh.hedge_density);

            c_puncts.push(fc.punct_density);
            h_puncts.push(fh.punct_density);

            // Label 0: correct, Label 1: hallucinated
            features.push(fc.to_array());
            labels.push(0.0);

            features.push(fh.to_array());
            labels.push(1.0);
        }

        let wc_jsd = JsdGate::compute_jsd(&c_words, &h_words, 20);
        let cc_jsd = JsdGate::compute_jsd(&c_chars, &h_chars, 20);
        let hd_jsd = JsdGate::compute_jsd(&c_hedges, &h_hedges, 20);
        let pd_jsd = JsdGate::compute_jsd(&c_puncts, &h_puncts, 20);

        let logistic_auroc = AurocGate::compute_auroc(&features, &labels);
        let mean_isomorphism = iso_sum / items.len() as f64;

        let auroc_ok = (logistic_auroc - 0.5).abs() <= (self.max_auroc - 0.5);
        let jsd_ok = wc_jsd <= self.max_jsd
            && cc_jsd <= self.max_jsd
            && hd_jsd <= self.max_jsd
            && pd_jsd <= self.max_jsd;

        let passed = auroc_ok && jsd_ok;

        let report = ConfoundGateReport {
            passed,
            logistic_auroc,
            max_allowed_auroc: self.max_auroc,
            word_count_jsd: wc_jsd,
            char_count_jsd: cc_jsd,
            hedge_density_jsd: hd_jsd,
            punct_density_jsd: pd_jsd,
            max_allowed_jsd: self.max_jsd,
            total_pairs_evaluated: items.len(),
            mean_isomorphism_score: mean_isomorphism,
        };

        if !passed {
            tracing::warn!(
                "Confound gate violation: AUROC={:.4} (max {:.4}), JSD=[wc={:.5}, cc={:.5}, hedge={:.5}, punct={:.5}]",
                logistic_auroc, self.max_auroc, wc_jsd, cc_jsd, hd_jsd, pd_jsd
            );
        }

        Ok(report)
    }
}
