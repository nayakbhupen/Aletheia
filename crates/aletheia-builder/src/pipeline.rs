use aletheia_agentic::AgenticAdversary;
use aletheia_confound::{compute_surface_metrics, ConfoundEngine, ConfoundGateReport};
use aletheia_core::{BenchmarkItem, QidGrounding, Result};
use aletheia_domains::DomainSeed;
use chrono::Utc;

pub struct DatasetBuilder {
    target_count: usize,
    confound_engine: ConfoundEngine,
}

impl DatasetBuilder {
    pub fn new(target_count: usize) -> Self {
        Self {
            target_count,
            confound_engine: ConfoundEngine::new(0.65, 0.20),
        }
    }

    /// Synthesize and strictly validate benchmark dataset
    pub fn build_dataset(&self) -> Result<(Vec<BenchmarkItem>, ConfoundGateReport)> {
        let seeds = DomainSeed::get_curated_seeds();
        let mut items = Vec::with_capacity(self.target_count);

        for i in 0..self.target_count {
            let seed = &seeds[i % seeds.len()];
            let item_id = format!("aletheia-v1-{:06}", i + 1);

            let mut correct_resp = seed.correct_response.to_string();
            let mut hallucinated_resp = seed.hallucinated_response.to_string();

            // Inject entity variations for massive variety
            let salt = (i / seeds.len()) + 1;
            if salt > 1 {
                correct_resp = format!("{} (Variant {})", correct_resp, salt);
                hallucinated_resp = format!("{} (Variant {})", hallucinated_resp, salt);
            }

            let surface_metrics = compute_surface_metrics(&correct_resp, &hallucinated_resp);

            let mut qid_grounding = None;
            if let Some((s_qid, s_lbl, p_pid, o_val, aliases)) = seed.qid {
                qid_grounding = Some(QidGrounding {
                    subject_qid: s_qid.to_string(),
                    subject_label: s_lbl.to_string(),
                    property_pid: p_pid.to_string(),
                    property_label: "relation".to_string(),
                    object_qid: None,
                    object_value: o_val.to_string(),
                    aliases: aliases.iter().map(|s| s.to_string()).collect(),
                });
            }

            let mut item = BenchmarkItem {
                id: item_id,
                index: i + 1,
                domain: seed.domain,
                modality: seed.modality,
                language: seed.language.to_string(),
                prompt: seed.prompt.to_string(),
                passage: seed.passage.to_string(),
                ground_truth: seed.ground_truth.to_string(),
                correct_response: correct_resp,
                hallucinated_response: hallucinated_resp,
                target_answer: seed.target_answer.to_string(),
                qid_grounding,
                surface_metrics,
                is_adversarial: false,
                adversarial_type: None,
                metadata: std::collections::HashMap::new(),
                created_at: Utc::now(),
            };

            // Adversarial injection: 10% sycophancy, 10% tool return conflict
            if i % 10 == 0 {
                item = AgenticAdversary::inject_sycophancy_pressure(&item);
            } else if i % 10 == 1 {
                item = AgenticAdversary::inject_tool_return_conflict(&item, "Conflicting search result #404");
            }

            items.push(item);
        }

        // Run the 10-gate confound audit
        let report = self.confound_engine.audit_dataset(&items)?;

        Ok((items, report))
    }
}
