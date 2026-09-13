/// Numerically stable sigmoid function preventing exp overflow/underflow
#[inline(always)]
fn stable_sigmoid(x: f64) -> f64 {
    if x >= 0.0 {
        let z = (-x).exp();
        1.0 / (1.0 + z)
    } else {
        let z = x.exp();
        z / (1.0 + z)
    }
}

/// Logistic regression classifier on surface features to detect predictive confounds
pub struct AurocGate;

impl AurocGate {
    /// Fit logistic regression and compute Mann-Whitney AUROC with exact tied-rank handling
    pub fn compute_auroc(features: &[[f64; 5]], labels: &[f64]) -> f64 {
        let n = features.len();
        if n == 0 || n != labels.len() {
            return 0.5;
        }

        // Standardize features safely
        let mut means = [0.0; 5];
        let mut stds = [1.0; 5];

        for d in 0..5 {
            let sum: f64 = features.iter().map(|f| f[d]).sum();
            means[d] = sum / n as f64;
            let var: f64 = features.iter().map(|f| (f[d] - means[d]).powi(2)).sum::<f64>() / n as f64;
            stds[d] = (var.sqrt()).max(1e-6);
        }

        let mut norm_features = Vec::with_capacity(n);
        for f in features {
            let mut nf = [0.0; 5];
            for d in 0..5 {
                nf[d] = (f[d] - means[d]) / stds[d];
            }
            norm_features.push(nf);
        }

        // Gradient descent with L2 regularization
        let mut weights = [0.0; 5];
        let mut bias = 0.0;
        let lr = 0.05;
        let l2 = 0.01;
        let epochs = 80;

        for _ in 0..epochs {
            let mut grad_w = [0.0; 5];
            let mut grad_b = 0.0;

            for i in 0..n {
                let mut logit = bias;
                for d in 0..5 {
                    logit += weights[d] * norm_features[i][d];
                }
                let prob = stable_sigmoid(logit);
                let err = prob - labels[i];

                grad_b += err;
                for d in 0..5 {
                    grad_w[d] += err * norm_features[i][d] + l2 * weights[d];
                }
            }

            bias -= lr * (grad_b / n as f64);
            for d in 0..5 {
                weights[d] -= lr * (grad_w[d] / n as f64);
            }
        }

        // Compute predicted probabilities
        let mut preds: Vec<(f64, f64)> = Vec::with_capacity(n);
        for i in 0..n {
            let mut logit = bias;
            for d in 0..5 {
                logit += weights[d] * norm_features[i][d];
            }
            let prob = stable_sigmoid(logit);
            preds.push((prob, labels[i]));
        }

        Self::calculate_mann_whitney_auroc(&preds)
    }

    /// Computes AUROC via Mann-Whitney U test with exact fractional tied-rank handling
    pub fn calculate_mann_whitney_auroc(predictions: &[(f64, f64)]) -> f64 {
        let n = predictions.len();
        if n == 0 {
            return 0.5;
        }

        let mut sorted = predictions.to_vec();
        // Sort by predicted probability ascending
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut n_pos = 0.0;
        let mut n_neg = 0.0;
        for &(_, label) in &sorted {
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
            while j < n && (sorted[j].0 - sorted[i].0).abs() < 1e-9 {
                j += 1;
            }
            // Average rank for tied group (1-indexed ranks from i+1 to j)
            let avg_rank = (i + 1 + j) as f64 / 2.0;
            for k in i..j {
                if sorted[k].1 > 0.5 {
                    rank_sum_pos += avg_rank;
                }
            }
            i = j;
        }

        let u_stat = rank_sum_pos - (n_pos * (n_pos + 1.0)) / 2.0;
        (u_stat / (n_pos * n_neg)).clamp(0.0, 1.0)
    }
}
