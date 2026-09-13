use aletheia_epistemic::{
    CombinatorialCadenceEngine, DiophantineInvariantSolver, EpistemicAuditor,
    TriadicConfluenceOperator,
};
use aletheia_grading::GradingCascade;
use aletheia_passage::{AnswerExtractor, KinematicAnalyzer, PassageIntegrityChecker};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::time::Instant;

/// JSON-RPC 2.0 Request wrapper
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

/// Aletheia Model Context Protocol (MCP) Server running over stdio
pub struct McpServer {
    epistemic_auditor: EpistemicAuditor,
    grading_cascade: GradingCascade,
}

impl McpServer {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            epistemic_auditor: EpistemicAuditor::default_threshold(),
            grading_cascade: GradingCascade::in_memory()?,
        })
    }

    /// Main loop: reads newline-delimited JSON-RPC from stdin, writes responses to stdout
    pub fn run_stdio(&mut self) -> anyhow::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line_res in stdin.lock().lines() {
            let line = match line_res {
                Ok(l) => l,
                Err(_) => break,
            };

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
                if let Some(resp) = self.handle_request(&req) {
                    let serialized = serde_json::to_string(&resp)?;
                    writeln!(stdout, "{}", serialized)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, req: &JsonRpcRequest) -> Option<Value> {
        let id = req.id.clone().unwrap_or(Value::Null);

        match req.method.as_str() {
            "initialize" => {
                Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "aletheia",
                            "version": "0.1.0"
                        }
                    }
                }))
            }

            "notifications/initialized" => {
                // Client initialization acknowledgment; no response needed
                None
            }

            "ping" => {
                Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {}
                }))
            }

            "tools/list" => {
                Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "aletheia_audit",
                                "description": "Sub-microsecond epistemic verification of model completion samples using native Phase-Space Resonance (R_sc), Abductive Grounding (alpha), Attractor Basin Curvature (kappa), and mode collapse detection.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "samples": {
                                            "type": "array",
                                            "items": { "type": "string" },
                                            "description": "K stochastic completion samples from the model for the same prompt"
                                        },
                                        "passage": {
                                            "type": "string",
                                            "description": "Optional reference grounding passage or prompt context"
                                        }
                                    },
                                    "required": ["samples"]
                                }
                            },
                            {
                                "name": "aletheia_confluence",
                                "description": "Audits a reasoning trace or passage using 3-Stream Triadic Confluence (Empirical, Deductive, Abductive streams), algebraic Diophantine invariant solver, and Combinatorial Cadence anomaly detection.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {
                                            "type": "string",
                                            "description": "Chain-of-thought reasoning text or multi-step passage"
                                        },
                                        "premise": {
                                            "type": "string",
                                            "description": "Optional reference grounding premise or factual context"
                                        }
                                    },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "aletheia_kinematics",
                                "description": "Analyzes reasoning chain trajectory dynamics: step velocity, acceleration spikes (pinpointing the exact step where hallucination/deflection occurred), deadpan score, and circularity/tautology (Heller score).",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {
                                            "type": "string",
                                            "description": "Chain-of-thought reasoning text or multi-step passage"
                                        }
                                    },
                                    "required": ["text"]
                                }
                            },
                            {
                                "name": "aletheia_grade",
                                "description": "Evaluates candidate answer against target ground truth using Aletheia's 3-Tier deterministic grading cascade (Exact match, normalized numeric/date, and Wikidata QID resolution).",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "candidate_answer": { "type": "string", "description": "Candidate answer extracted from model response" },
                                        "target_answer": { "type": "string", "description": "Ground truth verified answer" }
                                    },
                                    "required": ["candidate_answer", "target_answer"]
                                }
                            },
                            {
                                "name": "aletheia_integrity",
                                "description": "Validates passage integrity, detects token limit truncation cutoffs, unclosed tags, and extracts canonical answers.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "raw_response": { "type": "string", "description": "Raw model completion text" }
                                    },
                                    "required": ["raw_response"]
                                }
                            },
                            {
                                "name": "aletheia_doctor",
                                "description": "Runs sub-microsecond latency diagnostics and system verification for Aletheia's mathematical engine.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {}
                                }
                            }
                        ]
                    }
                }))
            }

            "tools/call" => {
                let tool_name = req.params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(Value::Null);

                let result = self.execute_tool(tool_name, &args);
                match result {
                    Ok(val) => {
                        Some(json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&val).unwrap_or_else(|_| val.to_string())
                                    }
                                ],
                                "isError": false
                            }
                        }))
                    }
                    Err(err_msg) => {
                        Some(json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": format!("Error: {}", err_msg)
                                    }
                                ],
                                "isError": true
                            }
                        }))
                    }
                }
            }

            _ => {
                Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method '{}' not found", req.method)
                    }
                }))
            }
        }
    }

    fn execute_tool(&mut self, name: &str, args: &Value) -> std::result::Result<Value, String> {
        match name {
            "aletheia_audit" => {
                let samples_val = args.get("samples").ok_or("Missing 'samples' argument")?;
                let samples: Vec<String> = samples_val
                    .as_array()
                    .ok_or("'samples' must be an array of strings")?
                    .iter()
                    .filter_map(|s| s.as_str().map(|str_val| str_val.to_string()))
                    .collect();

                if samples.is_empty() {
                    return Err("At least one completion sample is required".to_string());
                }

                let passage = args.get("passage").and_then(|p| p.as_str());
                let t_start = Instant::now();
                let audit_res = self.epistemic_auditor.audit_samples(&samples, passage);
                let latency_nanos = t_start.elapsed().as_nanos();

                Ok(json!({
                    "resonance_rsc": audit_res.rsc,
                    "epistemic_state": audit_res.epistemic_state,
                    "abductive_consistency": audit_res.abductive_consistency,
                    "attractor_curvature": audit_res.attractor_curvature,
                    "mode_collapse_detected": audit_res.attractor_collapse,
                    "dominant_answer": audit_res.dominant_answer,
                    "latency_nanos": latency_nanos,
                    "latency_micros": latency_nanos as f64 / 1000.0
                }))
            }

            "aletheia_confluence" => {
                let text = args.get("text").and_then(|t| t.as_str()).ok_or("Missing 'text' argument")?;
                let premise = args.get("premise").and_then(|p| p.as_str());
                let t_start = Instant::now();

                let triadic = TriadicConfluenceOperator::evaluate(text, premise);
                let synthesized_invariants = DiophantineInvariantSolver::synthesize_invariants_from_text(text);
                let word_count = text.split_whitespace().count() as i64;
                let invariant_results: Vec<_> = synthesized_invariants
                    .iter()
                    .map(|inv| DiophantineInvariantSolver::verify(inv, word_count))
                    .collect();
                let cadence = CombinatorialCadenceEngine::analyze(text);
                let latency_nanos = t_start.elapsed().as_nanos();

                Ok(json!({
                    "status": format!("{:?}", triadic.status),
                    "confluence_volume": triadic.confluence_volume,
                    "consensus_score": triadic.consensus_score,
                    "asymmetry_index": triadic.asymmetry_index,
                    "empirical_score": triadic.empirical_score,
                    "deductive_score": triadic.deductive_score,
                    "abductive_score": triadic.abductive_score,
                    "combinatorial_transition_entropy": cadence.transition_entropy,
                    "is_cadence_harmonious": cadence.is_harmonious,
                    "cadence_fractures": cadence.cadence_fractures,
                    "invariants_satisfied": invariant_results.iter().filter(|r| r.is_satisfied).count(),
                    "invariants_total": invariant_results.len(),
                    "latency_nanos": latency_nanos,
                    "latency_micros": latency_nanos as f64 / 1000.0
                }))
            }

            "aletheia_kinematics" => {
                let text = args.get("text").and_then(|t| t.as_str()).ok_or("Missing 'text' argument")?;
                let t_start = Instant::now();
                let metrics = KinematicAnalyzer::analyze(text);
                let latency_nanos = t_start.elapsed().as_nanos();

                Ok(json!({
                    "step_count": metrics.step_count,
                    "mean_velocity": metrics.mean_velocity,
                    "velocity_variance": metrics.velocity_variance,
                    "mean_acceleration": metrics.mean_acceleration,
                    "max_acceleration": metrics.max_acceleration,
                    "max_acceleration_step": metrics.max_acceleration_index + 1,
                    "acceleration_spikes": metrics.acceleration_spikes,
                    "primary_derailment_step": metrics.primary_derailment_step,
                    "deadpan_score": metrics.deadpan_score,
                    "circularity_score": metrics.circularity_score,
                    "heller_score": metrics.heller_score,
                    "straightness": metrics.straightness,
                    "latency_nanos": latency_nanos,
                    "latency_micros": latency_nanos as f64 / 1000.0
                }))
            }

            "aletheia_grade" => {
                let cand = args.get("candidate_answer").and_then(|c| c.as_str()).ok_or("Missing 'candidate_answer'")?;
                let target = args.get("target_answer").and_then(|t| t.as_str()).ok_or("Missing 'target_answer'")?;

                let dummy_item = aletheia_core::BenchmarkItem::new(
                    "mcp_grade",
                    0,
                    aletheia_core::Domain::Wikipedia,
                    aletheia_core::Modality::Fabrication,
                    "",
                    "",
                    target,
                    target,
                    "",
                );

                let grade = self.grading_cascade.grade(&dummy_item, cand);
                Ok(json!({
                    "is_correct": grade.is_correct,
                    "tier": grade.tier.as_str(),
                    "confidence": grade.confidence
                }))
            }

            "aletheia_integrity" => {
                let raw = args.get("raw_response").and_then(|r| r.as_str()).ok_or("Missing 'raw_response'")?;
                let (clean_answer, reasoning) = AnswerExtractor::extract(raw);
                let is_truncated = PassageIntegrityChecker::is_truncated(raw);
                let is_abstention = PassageIntegrityChecker::is_abstention(raw);

                Ok(json!({
                    "extracted_answer": clean_answer,
                    "reasoning_chain": reasoning,
                    "is_truncated": is_truncated,
                    "is_abstention": is_abstention
                }))
            }

            "aletheia_doctor" => {
                let t_start = Instant::now();
                let samples = vec![
                    "The capital of France is Paris.".to_string(),
                    "Paris is the capital of France.".to_string(),
                    "The capital of France is Paris.".to_string(),
                ];
                let audit_res = self.epistemic_auditor.audit_samples(&samples, None);
                let latency_nanos = t_start.elapsed().as_nanos();

                Ok(json!({
                    "status": "OPERATIONAL",
                    "engine": "Aletheia Phase-Space Resonance & Abductive Grounding",
                    "initial_state": audit_res.epistemic_state,
                    "audit_latency_micros": latency_nanos as f64 / 1000.0,
                    "version": "0.1.0"
                }))
            }

            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}
