/// Computes Jensen-Shannon Divergence (JSD) between two scalar sample sets
/// D_JS(P || Q) = 0.5 * D_KL(P || M) + 0.5 * D_KL(Q || M), where M = 0.5 * (P + Q)
pub struct JsdGate;

impl JsdGate {
    /// Compute JSD between two distributions using stack-allocated histogram bins
    pub fn compute_jsd(p_samples: &[f64], q_samples: &[f64], n_bins: usize) -> f64 {
        if p_samples.is_empty() || q_samples.is_empty() {
            return 0.0;
        }

        let bins = n_bins.clamp(2, 32);

        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for &v in p_samples.iter().chain(q_samples.iter()) {
            if v.is_finite() {
                if v < min_val {
                    min_val = v;
                }
                if v > max_val {
                    max_val = v;
                }
            }
        }

        if !min_val.is_finite() || !max_val.is_finite() || (max_val - min_val).abs() < 1e-9 {
            return 0.0;
        }

        let bin_width = (max_val - min_val) / bins as f64;
        let mut p_counts = [0.0f64; 32];
        let mut q_counts = [0.0f64; 32];

        let eps = 1e-6;

        for &v in p_samples {
            if v.is_finite() {
                let mut bin = ((v - min_val) / bin_width).floor() as usize;
                if bin >= bins {
                    bin = bins - 1;
                }
                p_counts[bin] += 1.0;
            }
        }

        for &v in q_samples {
            if v.is_finite() {
                let mut bin = ((v - min_val) / bin_width).floor() as usize;
                if bin >= bins {
                    bin = bins - 1;
                }
                q_counts[bin] += 1.0;
            }
        }

        let p_total = p_samples.len() as f64 + (bins as f64 * eps);
        let q_total = q_samples.len() as f64 + (bins as f64 * eps);

        let mut kl_pm = 0.0;
        let mut kl_qm = 0.0;

        for i in 0..bins {
            let p_prob = (p_counts[i] + eps) / p_total;
            let q_prob = (q_counts[i] + eps) / q_total;
            let m = 0.5 * (p_prob + q_prob);

            if p_prob > 0.0 && m > 0.0 {
                kl_pm += p_prob * (p_prob / m).ln();
            }
            if q_prob > 0.0 && m > 0.0 {
                kl_qm += q_prob * (q_prob / m).ln();
            }
        }

        (0.5 * kl_pm + 0.5 * kl_qm).max(0.0)
    }

    /// Verifies if JSD is under the strict threshold
    pub fn verify_threshold(p_samples: &[f64], q_samples: &[f64], threshold: f64) -> (bool, f64) {
        let jsd = Self::compute_jsd(p_samples, q_samples, 20);
        (jsd <= threshold, jsd)
    }
}
