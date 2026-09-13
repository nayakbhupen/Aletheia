use serde::{Deserialize, Serialize};

/// High-dimensional semantic projection dimension for feature hashing
const HASH_PROJECTION_DIM: usize = 256;

/// Details of an anomalous acceleration spike in the reasoning trajectory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccelerationSpike {
    /// Step index where the acceleration spike occurred
    pub index: usize,
    /// Absolute magnitude of acceleration
    pub magnitude: f64,
    /// Signed pacing shift (positive = sudden leap, negative = sudden slowdown)
    pub pacing_shift: f64,
    /// Outlier isolation score relative to background variance
    pub isolation_score: f64,
    /// Relative position in trajectory in [0.0, 1.0]
    pub position_ratio: f64,
}

/// Comprehensive kinematic trajectory profile of a reasoning chain or text passage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KinematicMetrics {
    /// Number of discrete reasoning steps / sentences analyzed
    pub step_count: usize,
    /// Mean semantic step velocity (average pacing distance)
    pub mean_velocity: f64,
    /// Variance in step velocity (pacing stability)
    pub velocity_variance: f64,
    /// Mean acceleration across steps
    pub mean_acceleration: f64,
    /// Variance in acceleration
    pub acceleration_variance: f64,
    /// Maximum absolute acceleration observed
    pub max_acceleration: f64,
    /// Step index of maximum acceleration
    pub max_acceleration_index: usize,
    /// Detected anomalous acceleration spikes (potential hallucination / drift deflection points)
    pub acceleration_spikes: Vec<AccelerationSpike>,
    /// Deadpan score: ratio of peak spike magnitude to steady background variance
    /// (high = sudden ungrounded assertion buried in calm, formal prose)
    pub deadpan_score: f64,
    /// Circularity score: semantic similarity between origin and conclusion
    pub circularity_score: f64,
    /// Heller score (Catch-22 tautology metric): circular looping combined with semantic deceleration
    pub heller_score: f64,
    /// Straightness ratio: net displacement divided by total cumulative trajectory length
    pub straightness: f64,
    /// Step index identified as the primary derailment or logical leap point
    pub primary_derailment_step: Option<usize>,
    /// Per-step velocity profile
    pub velocity_profile: Vec<f64>,
    /// Per-step acceleration profile
    pub acceleration_profile: Vec<f64>,
}

/// Pure-Rust Kinematic Trajectory Analyzer for reasoning chains and text passages
pub struct KinematicAnalyzer;

impl KinematicAnalyzer {
    /// Analyze the kinematic trajectory of a reasoning chain or passage
    pub fn analyze(text: &str) -> KinematicMetrics {
        let steps = Self::segment_reasoning_steps(text);
        if steps.len() < 2 {
            return Self::empty_metrics(steps.len());
        }

        // 1. Embed each step into high-dimensional unit-norm feature space
        let embeddings: Vec<[f32; HASH_PROJECTION_DIM]> = steps
            .iter()
            .map(|s| Self::hash_embed_step(s))
            .collect();

        // 2. Compute step velocities (cosine distance between consecutive steps)
        let n_steps = steps.len();
        let mut velocities = Vec::with_capacity(n_steps.saturating_sub(1));
        for i in 0..n_steps - 1 {
            let sim = Self::cosine_similarity(&embeddings[i], &embeddings[i + 1]);
            let vel = (1.0 - sim as f64).clamp(0.0, 2.0);
            velocities.push(vel);
        }

        let mean_vel = if velocities.is_empty() {
            0.0
        } else {
            velocities.iter().sum::<f64>() / velocities.len() as f64
        };

        let vel_var = if velocities.len() > 1 {
            velocities.iter().map(|v| (v - mean_vel).powi(2)).sum::<f64>() / velocities.len() as f64
        } else {
            0.0
        };

        // 3. Compute accelerations (differences in velocity)
        let mut accelerations = Vec::with_capacity(velocities.len().saturating_sub(1));
        for i in 0..velocities.len().saturating_sub(1) {
            let acc = velocities[i + 1] - velocities[i];
            accelerations.push(acc);
        }

        let mean_acc = if accelerations.is_empty() {
            0.0
        } else {
            accelerations.iter().sum::<f64>() / accelerations.len() as f64
        };

        let acc_var = if accelerations.len() > 1 {
            accelerations.iter().map(|a| (a - mean_acc).powi(2)).sum::<f64>() / accelerations.len() as f64
        } else {
            0.0
        };
        let acc_std = acc_var.sqrt();

        // 4. Detect acceleration spikes & derailment points
        let mut spikes = Vec::new();
        let mut max_acc_mag = 0.0;
        let mut max_acc_idx = 0;

        for (idx, &acc) in accelerations.iter().enumerate() {
            let mag = acc.abs();
            if mag > max_acc_mag {
                max_acc_mag = mag;
                max_acc_idx = idx;
            }

            let isolation = if acc_std > 1e-6 {
                (acc - mean_acc).abs() / acc_std
            } else {
                mag
            };

            // Spike criteria: absolute magnitude > 0.25 or isolation score > 2.0
            if mag > 0.25 || isolation > 2.0 {
                spikes.push(AccelerationSpike {
                    index: idx + 1, // Step where the deflection materialized
                    magnitude: mag,
                    pacing_shift: acc,
                    isolation_score: isolation,
                    position_ratio: (idx + 1) as f64 / n_steps as f64,
                });
            }
        }

        // 5. Deadpan Score: isolated acceleration peak against quiet background
        let deadpan_score = if !accelerations.is_empty() {
            let mean_abs = accelerations.iter().map(|a| a.abs()).sum::<f64>() / accelerations.len() as f64;
            let ratio = max_acc_mag / (mean_abs + acc_std + 0.05);
            (ratio / 4.0).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // 6. Circularity & Heller Score
        let origin_sim = Self::cosine_similarity(&embeddings[0], &embeddings[n_steps - 1]) as f64;
        let circularity = origin_sim.clamp(0.0, 1.0);

        // Deceleration between first and final steps
        let first_vel = velocities.first().copied().unwrap_or(0.0);
        let last_vel = velocities.last().copied().unwrap_or(0.0);
        let deceleration = (first_vel - last_vel).max(0.0);

        // Path metrics
        let total_path_length: f64 = velocities.iter().sum();
        let net_displacement = (1.0 - origin_sim).max(0.0).sqrt();
        let straightness = if total_path_length > 1e-6 {
            (net_displacement / total_path_length).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Heller score: tautological circularity combined with high path length vs net displacement
        let tautology_density = if net_displacement < 0.2 && total_path_length > 0.5 {
            1.0 - (net_displacement / (total_path_length + 0.1)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let heller_score = (circularity * 0.5 + tautology_density * 0.3 + deceleration * 0.2).clamp(0.0, 1.0);

        let primary_derailment_step = if !spikes.is_empty() {
            // Sort by isolation score descending to find the most violent deviation
            let mut sorted_spikes = spikes.clone();
            sorted_spikes.sort_by(|a, b| b.isolation_score.partial_cmp(&a.isolation_score).unwrap());
            Some(sorted_spikes[0].index)
        } else if max_acc_mag > 0.20 {
            Some(max_acc_idx + 1)
        } else {
            None
        };

        KinematicMetrics {
            step_count: n_steps,
            mean_velocity: mean_vel,
            velocity_variance: vel_var,
            mean_acceleration: mean_acc,
            acceleration_variance: acc_var,
            max_acceleration: max_acc_mag,
            max_acceleration_index: max_acc_idx,
            acceleration_spikes: spikes,
            deadpan_score,
            circularity_score: circularity,
            heller_score,
            straightness,
            primary_derailment_step,
            velocity_profile: velocities,
            acceleration_profile: accelerations,
        }
    }

    /// Segments text into reasoning steps or sentences
    pub fn segment_reasoning_steps(text: &str) -> Vec<String> {
        let mut steps = Vec::new();

        // Check if explicitly numbered steps exist (e.g., "1. ", "Step 1:", "- ")
        let lines: Vec<&str> = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        let has_numbered_steps = lines.len() >= 2 && lines.iter().any(|l| {
            l.starts_with("1.") || l.starts_with("Step 1") || l.starts_with("- ") || l.starts_with("* ")
        });

        if has_numbered_steps && lines.len() >= 2 {
            for line in lines {
                let cleaned = line
                    .trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == '*' || c == ' ')
                    .trim();
                if !cleaned.is_empty() {
                    steps.push(cleaned.to_string());
                }
            }
        } else {
            // Standard sentence segmentation on punctuation delimiters
            let mut current = String::new();
            let chars: Vec<char> = text.chars().collect();
            let len = chars.len();

            let mut i = 0;
            while i < len {
                let c = chars[i];
                current.push(c);

                if (c == '.' || c == '!' || c == '?' || c == '\n') && i + 1 < len {
                    let next = chars[i + 1];
                    if next.is_whitespace() || c == '\n' {
                        let trimmed = current.trim();
                        // Ignore abbreviations or tiny tokens like "e.g." or "Dr."
                        if trimmed.len() > 3 {
                            steps.push(trimmed.to_string());
                            current.clear();
                        }
                    }
                }
                i += 1;
            }

            let last_trimmed = current.trim();
            if !last_trimmed.is_empty() {
                steps.push(last_trimmed.to_string());
            }
        }

        steps
    }

    /// Fast, deterministic, zero-allocation feature hashing into high-dimensional unit vector
    #[inline]
    fn hash_embed_step(text: &str) -> [f32; HASH_PROJECTION_DIM] {
        let mut vec = [0.0f32; HASH_PROJECTION_DIM];
        let lower = text.to_lowercase();
        let tokens: Vec<&str> = lower
            .split(|c: char| !c.is_alphanumeric())
            .filter(|t| !t.is_empty())
            .collect();

        if tokens.is_empty() {
            return vec;
        }

        // 1. Unigrams
        for token in &tokens {
            let h = Self::fnv1a_hash(token.as_bytes());
            let idx = (h as usize) % HASH_PROJECTION_DIM;
            let sign = if (h >> 31) & 1 == 1 { 1.0 } else { -1.0 };
            vec[idx] += sign;
        }

        // 2. Bigrams for sequential semantic context
        for window in tokens.windows(2) {
            let mut bigram_bytes = Vec::with_capacity(window[0].len() + window[1].len() + 1);
            bigram_bytes.extend_from_slice(window[0].as_bytes());
            bigram_bytes.push(b'_');
            bigram_bytes.extend_from_slice(window[1].as_bytes());

            let h = Self::fnv1a_hash(&bigram_bytes);
            let idx = (h as usize) % HASH_PROJECTION_DIM;
            let sign = if (h >> 31) & 1 == 1 { 1.5 } else { -1.5 };
            vec[idx] += sign;
        }

        // 3. L2 Normalization
        let norm_sq: f32 = vec.iter().map(|&x| x * x).sum();
        if norm_sq > 1e-12 {
            let inv_norm = 1.0 / norm_sq.sqrt();
            for x in vec.iter_mut() {
                *x *= inv_norm;
            }
        }

        vec
    }

    #[inline(always)]
    fn fnv1a_hash(bytes: &[u8]) -> u32 {
        let mut hash = 0x811c9dc5u32;
        for &byte in bytes {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(0x01000193u32);
        }
        hash
    }

    #[inline(always)]
    fn cosine_similarity(a: &[f32; HASH_PROJECTION_DIM], b: &[f32; HASH_PROJECTION_DIM]) -> f32 {
        let mut dot = 0.0f32;
        for i in 0..HASH_PROJECTION_DIM {
            dot += a[i] * b[i];
        }
        dot.clamp(-1.0, 1.0)
    }

    fn empty_metrics(step_count: usize) -> KinematicMetrics {
        KinematicMetrics {
            step_count,
            mean_velocity: 0.0,
            velocity_variance: 0.0,
            mean_acceleration: 0.0,
            acceleration_variance: 0.0,
            max_acceleration: 0.0,
            max_acceleration_index: 0,
            acceleration_spikes: Vec::new(),
            deadpan_score: 0.0,
            circularity_score: 1.0,
            heller_score: 0.0,
            straightness: 1.0,
            primary_derailment_step: None,
            velocity_profile: Vec::new(),
            acceleration_profile: Vec::new(),
        }
    }
}
