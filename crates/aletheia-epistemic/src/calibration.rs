pub struct EpistemicCalibration;

impl EpistemicCalibration {
    /// Compute E-AUROC: Area Under ROC for confidence (1 - R_sc) predicting correctness with exact tied-rank handling
    pub fn compute_e_auroc(confidences: &[f64], correctness: &[bool]) -> f64 {
        let n = confidences.len();
        if n == 0 || n != correctness.len() {
            return 0.5;
        }

        let mut pairs: Vec<(f64, f64)> = confidences
            .iter()
            .zip(correctness.iter())
            .map(|(&c, &y)| (c, if y { 1.0 } else { 0.0 }))
            .collect();

        // Sort by confidence ascending
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut n_pos = 0.0;
        let mut n_neg = 0.0;
        for &(_, label) in &pairs {
            if label > 0.5 {
                n_pos += 1.0;
            } else {
                n_neg += 1.0;
            }
        }

        if n_pos == 0.0 || n_neg == 0.0 {
            return 0.5;
        }

        // Exact tied-rank assignment
        let mut rank_sum_pos = 0.0;
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && (pairs[j].0 - pairs[i].0).abs() < 1e-9 {
                j += 1;
            }
            // Average rank for tied values in [i+1, j]
            let avg_rank = (i + 1 + j) as f64 / 2.0;
            for k in i..j {
                if pairs[k].1 > 0.5 {
                    rank_sum_pos += avg_rank;
                }
            }
            i = j;
        }

        let u_stat = rank_sum_pos - (n_pos * (n_pos + 1.0)) / 2.0;
        (u_stat / (n_pos * n_neg)).clamp(0.0, 1.0)
    }

    /// Compute Expected Calibration Error (ECE) with n_bins
    pub fn compute_ece(confidences: &[f64], correctness: &[bool], n_bins: usize) -> f64 {
        let n = confidences.len();
        if n == 0 || n != correctness.len() {
            return 0.0;
        }

        let bins = n_bins.clamp(2, 50);
        let mut bin_counts = vec![0.0; bins];
        let mut bin_conf_sums = vec![0.0; bins];
        let mut bin_acc_sums = vec![0.0; bins];

        for (&conf, &correct) in confidences.iter().zip(correctness.iter()) {
            let clamped_conf = conf.clamp(0.0, 0.9999);
            let bin_idx = (clamped_conf * bins as f64).floor() as usize;

            bin_counts[bin_idx] += 1.0;
            bin_conf_sums[bin_idx] += clamped_conf;
            if correct {
                bin_acc_sums[bin_idx] += 1.0;
            }
        }

        let mut ece = 0.0;
        for i in 0..bins {
            if bin_counts[i] > 0.0 {
                let bin_acc = bin_acc_sums[i] / bin_counts[i];
                let bin_conf = bin_conf_sums[i] / bin_counts[i];
                let weight = bin_counts[i] / n as f64;
                ece += weight * (bin_acc - bin_conf).abs();
            }
        }

        ece
    }

    /// Compute Brier score: Mean squared error between confidence and binary outcome
    pub fn compute_brier_score(confidences: &[f64], correctness: &[bool]) -> f64 {
        if confidences.is_empty() || confidences.len() != correctness.len() {
            return 0.25;
        }

        let sum: f64 = confidences
            .iter()
            .zip(correctness.iter())
            .map(|(&c, &y)| {
                let target = if y { 1.0 } else { 0.0 };
                (c - target).powi(2)
            })
            .sum();

        sum / confidences.len() as f64
    }
}
