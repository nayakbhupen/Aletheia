use aletheia_core::{AletheiaError, BenchmarkItem, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[async_trait::async_trait]
pub trait ModelAdapter: Send + Sync {
    fn model_id(&self) -> &str;
    async fn generate(&self, prompt: &str, k_samples: usize, temperature: f64) -> Result<Vec<String>>;

    async fn generate_for_item(&self, item: &BenchmarkItem, k_samples: usize, temperature: f64) -> Result<Vec<String>> {
        self.generate(&item.prompt, k_samples, temperature).await
    }
}

// 1. Ollama Adapter (Local Free Models)
pub struct OllamaAdapter {
    client: Client,
    base_url: String,
    model_name: String,
    max_new_tokens: usize,
}

impl OllamaAdapter {
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(120)).build().unwrap(),
            base_url: "http://localhost:11434".to_string(),
            model_name: model_name.into(),
            max_new_tokens: 256,
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_new_tokens = max_tokens;
        self
    }
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f64,
    num_predict: usize,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait::async_trait]
impl ModelAdapter for OllamaAdapter {
    fn model_id(&self) -> &str {
        &self.model_name
    }

    async fn generate(&self, prompt: &str, k_samples: usize, temperature: f64) -> Result<Vec<String>> {
        let mut results = Vec::with_capacity(k_samples);
        let endpoint = format!("{}/api/generate", self.base_url);

        for _ in 0..k_samples {
            let req_body = OllamaRequest {
                model: &self.model_name,
                prompt,
                stream: false,
                options: OllamaOptions {
                    temperature,
                    num_predict: self.max_new_tokens,
                },
            };

            let res = self.client.post(&endpoint)
                .json(&req_body)
                .send()
                .await
                .map_err(|e| AletheiaError::ModelInference(format!("Ollama connection error: {}", e)))?;

            if !res.status().is_success() {
                let err_text = res.text().await.unwrap_or_default();
                return Err(AletheiaError::ModelInference(format!("Ollama error: {}", err_text)));
            }

            let resp: OllamaResponse = res.json()
                .await
                .map_err(|e| AletheiaError::ModelInference(format!("Ollama JSON error: {}", e)))?;

            results.push(resp.response);
        }

        Ok(results)
    }
}

// 2. Mock Adapter (Deterministic offline testing & benchmarking)
pub struct MockAdapter {
    model_name: String,
    simulated_accuracy: f64,
}

impl MockAdapter {
    pub fn new(model_name: impl Into<String>, simulated_accuracy: f64) -> Self {
        Self {
            model_name: model_name.into(),
            simulated_accuracy,
        }
    }

    pub fn simulated_accuracy(&self) -> f64 {
        self.simulated_accuracy
    }
}

#[async_trait::async_trait]
impl ModelAdapter for MockAdapter {
    fn model_id(&self) -> &str {
        &self.model_name
    }

    async fn generate(&self, _prompt: &str, k_samples: usize, _temp: f64) -> Result<Vec<String>> {
        let mut results = Vec::with_capacity(k_samples);
        for i in 0..k_samples {
            results.push(format!("<answer>Verified answer for sample {}</answer>", i));
        }
        Ok(results)
    }

    async fn generate_for_item(&self, item: &BenchmarkItem, k_samples: usize, _temp: f64) -> Result<Vec<String>> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut results = Vec::with_capacity(k_samples);
        let roll: f64 = rng.gen_range(0.0..1.0);
        let is_correct = roll < self.simulated_accuracy;

        for _ in 0..k_samples {
            if is_correct {
                results.push(format!("<answer>{}</answer>", item.target_answer));
            } else {
                results.push(format!("<answer>{}</answer>", item.hallucinated_response));
            }
        }
        Ok(results)
    }
}

/// Heuristic classifier that attempts to exploit word-count length shortcuts (like in HaluEval)
pub struct WordCountHeuristicAdapter;

#[async_trait::async_trait]
impl ModelAdapter for WordCountHeuristicAdapter {
    fn model_id(&self) -> &str {
        "heuristic:length-cheat (HaluEval Exploit)"
    }

    async fn generate(&self, _prompt: &str, k_samples: usize, _temp: f64) -> Result<Vec<String>> {
        Ok(vec!["".to_string(); k_samples])
    }

    async fn generate_for_item(&self, item: &BenchmarkItem, k_samples: usize, _temp: f64) -> Result<Vec<String>> {
        // HaluEval cheat: always picks the longer text
        let chosen = if item.correct_response.len() > item.hallucinated_response.len() {
            &item.correct_response
        } else {
            &item.hallucinated_response
        };
        Ok(vec![chosen.clone(); k_samples])
    }
}

