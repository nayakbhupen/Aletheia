use serde::{Deserialize, Serialize};

/// Global configuration for Aletheia benchmark suite and evaluation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub target_item_count: usize,
    pub confound_max_auroc: f64,
    pub confound_max_jsd: f64,
    #[serde(alias = "spectral_threshold")]
    pub resonance_threshold: f64,
    pub k_sampling_paths: usize,
    pub sqlite_cache_path: String,
    pub onnx_model_path: Option<String>,
    pub default_temperature: f64,
    #[serde(default = "default_max_new_tokens")]
    pub max_new_tokens: usize,
}

fn default_max_new_tokens() -> usize {
    256
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            target_item_count: 25_000,
            confound_max_auroc: 0.505,
            confound_max_jsd: 0.001,
            resonance_threshold: 0.35,
            k_sampling_paths: 5,
            sqlite_cache_path: "data/wikidata_cache.db".to_string(),
            onnx_model_path: None,
            default_temperature: 0.7,
            max_new_tokens: 256,
        }
    }
}
