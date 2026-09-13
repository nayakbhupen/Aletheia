use aletheia_core::EvaluationReceipt;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Metadata stored in sidecar `<checkpoint>.meta.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMeta {
    pub benchmark_version: String,
    pub model_id: String,
    pub total_items: usize,
    pub completed_count: usize,
    pub k_samples: usize,
    pub max_tokens: usize,
    pub created_at: String,
    pub updated_at: String,
}

/// Manages crash-resilient streaming execution and idempotent resumption
pub struct CheckpointManager {
    path: PathBuf,
    meta_path: PathBuf,
    completed_ids: HashSet<String>,
    existing_receipts: Vec<EvaluationReceipt>,
}

impl CheckpointManager {
    /// Initialize manager, scanning existing checkpoint JSONL and sidecar metadata
    pub fn init(checkpoint_path: impl AsRef<Path>, model_id: &str, total_items: usize, k_samples: usize, max_tokens: usize) -> anyhow::Result<Self> {
        let path = checkpoint_path.as_ref().to_path_buf();
        let meta_path = PathBuf::from(format!("{}.meta.json", path.display()));

        let mut completed_ids = HashSet::new();
        let mut existing_receipts = Vec::new();

        if path.exists() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line_str = line?;
                let trimmed = line_str.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Ok(receipt) = serde_json::from_str::<EvaluationReceipt>(trimmed) {
                    completed_ids.insert(receipt.item_id.clone());
                    existing_receipts.push(receipt);
                }
            }

            // Verify sidecar metadata compatibility if present
            if meta_path.exists() {
                if let Ok(meta_file) = File::open(&meta_path) {
                    if let Ok(meta) = serde_json::from_reader::<_, CheckpointMeta>(BufReader::new(meta_file)) {
                        if meta.model_id != model_id {
                            anyhow::bail!(
                                "Checkpoint model mismatch: checkpoint was built for model '{}', but current model is '{}'",
                                meta.model_id,
                                model_id
                            );
                        }
                    }
                }
            }
        }

        let manager = Self {
            path,
            meta_path,
            completed_ids,
            existing_receipts,
        };

        // Write/update sidecar metadata
        manager.update_meta(model_id, total_items, k_samples, max_tokens)?;

        Ok(manager)
    }

    /// Check if item has already been evaluated in a previous run
    pub fn is_completed(&self, item_id: &str) -> bool {
        self.completed_ids.contains(item_id)
    }

    /// Count of already completed items
    pub fn completed_count(&self) -> usize {
        self.completed_ids.len()
    }

    /// Retrieve all receipts loaded from existing checkpoint
    pub fn existing_receipts(&self) -> &[EvaluationReceipt] {
        &self.existing_receipts
    }

    /// Append a single receipt atomically to the checkpoint JSONL file
    pub fn append_receipt(&mut self, receipt: &EvaluationReceipt) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let serialized = serde_json::to_string(receipt)?;
        writeln!(file, "{}", serialized)?;
        file.flush()?;

        self.completed_ids.insert(receipt.item_id.clone());
        self.existing_receipts.push(receipt.clone());

        Ok(())
    }

    /// Update sidecar metadata
    pub fn update_meta(&self, model_id: &str, total_items: usize, k_samples: usize, max_tokens: usize) -> anyhow::Result<()> {
        let meta = CheckpointMeta {
            benchmark_version: "0.1.0".to_string(),
            model_id: model_id.to_string(),
            total_items,
            completed_count: self.completed_ids.len(),
            k_samples,
            max_tokens,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let file = File::create(&self.meta_path)?;
        serde_json::to_writer_pretty(file, &meta)?;
        Ok(())
    }
}
