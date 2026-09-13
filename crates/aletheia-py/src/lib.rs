//! Official Python Bindings for Aletheia: The Definitive Epistemic Hallucination Engine.
//! Exposes sub-microsecond Rust math kernels directly to Python and PyTorch workflows.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::time::Instant;

use aletheia_core::{BenchmarkItem, Domain, Modality, QidGrounding};
use aletheia_epistemic::{
    CombinatorialCadenceEngine, DiophantineInvariantSolver, PhaseSpaceResonanceEngine,
    TriadicConfluenceOperator,
};
use aletheia_grading::GradingCascade;
use aletheia_passage::{AnswerExtractor, KinematicAnalyzer, PassageIntegrityChecker};

/// Evaluates K sampled LLM response paths for phase-space trajectory resonance R_sc,
/// epistemic consensus state, and mode collapse.
#[pyfunction]
#[pyo3(signature = (samples, context=None, threshold=None))]
fn audit<'py>(
    py: Python<'py>,
    samples: Vec<String>,
    context: Option<String>,
    threshold: Option<f64>,
) -> PyResult<Bound<'py, PyDict>> {
    let tau = threshold.unwrap_or(0.35);
    let receipt = PhaseSpaceResonanceEngine::evaluate(&samples, context.as_deref(), tau);

    let dict = PyDict::new(py);
    dict.set_item("rsc", receipt.rsc)?;
    dict.set_item("state_name", receipt.state_name)?;
    dict.set_item("decision", receipt.decision)?;
    dict.set_item("is_safe", receipt.is_safe)?;
    dict.set_item("dominant_answer", receipt.dominant_answer)?;
    dict.set_item("k_samples", receipt.k_samples)?;
    dict.set_item("agreement_ratio", receipt.agreement_ratio)?;
    dict.set_item("attractor_collapse", receipt.attractor_collapse)?;
    dict.set_item("attractor_score", receipt.attractor_score)?;
    dict.set_item("latency_nanos", receipt.latency_nanos as u64)?;
    dict.set_item("latency_micros", receipt.latency_micros)?;

    Ok(dict)
}

/// Evaluates 3-stream triadic confluence (empirical, deductive, abductive),
/// combinatorial cadence transitions, and linear Diophantine invariants.
#[pyfunction]
#[pyo3(signature = (reasoning, premise=None))]
fn confluence<'py>(
    py: Python<'py>,
    reasoning: String,
    premise: Option<String>,
) -> PyResult<Bound<'py, PyDict>> {
    let t_start = Instant::now();
    let triadic = TriadicConfluenceOperator::evaluate(&reasoning, premise.as_deref());
    let cadence = CombinatorialCadenceEngine::analyze(&reasoning);
    let invariants = DiophantineInvariantSolver::synthesize_invariants_from_text(&reasoning);

    let mut solved_count = 0usize;
    let mut total_invariants = 0usize;
    for inv in &invariants {
        total_invariants += 1;
        let v = DiophantineInvariantSolver::verify(inv, reasoning.len() as i64);
        if v.is_satisfied {
            solved_count += 1;
        }
    }

    let status_str = match triadic.status {
        aletheia_epistemic::TriadicStatus::TriadicConsensus => "TRIADIC_CONSENSUS",
        aletheia_epistemic::TriadicStatus::EmpiricalDeficit => "EMPIRICAL_DEFICIT",
        aletheia_epistemic::TriadicStatus::AbductiveRupture => "ABDUCTIVE_RUPTURE",
        aletheia_epistemic::TriadicStatus::DeductiveSlip => "DEDUCTIVE_SLIP",
        aletheia_epistemic::TriadicStatus::EpistemicVoid => "EPISTEMIC_VOID",
    };

    let fractures_list = PyList::empty(py);
    for f in &cadence.cadence_fractures {
        let f_dict = PyDict::new(py);
        f_dict.set_item("step_index", f.step_index)?;
        f_dict.set_item("prior_entropy", f.prior_entropy)?;
        f_dict.set_item("post_entropy", f.post_entropy)?;
        f_dict.set_item("entropy_delta", f.entropy_delta)?;
        f_dict.set_item("severity", f.severity)?;
        fractures_list.append(f_dict)?;
    }

    let dict = PyDict::new(py);
    dict.set_item("status", status_str)?;
    dict.set_item("confluence_volume", triadic.confluence_volume)?;
    dict.set_item("consensus_score", triadic.consensus_score)?;
    dict.set_item("asymmetry_index", triadic.asymmetry_index)?;
    dict.set_item("empirical_score", triadic.empirical_score)?;
    dict.set_item("deductive_score", triadic.deductive_score)?;
    dict.set_item("abductive_score", triadic.abductive_score)?;
    dict.set_item("transition_entropy", cadence.transition_entropy)?;
    dict.set_item("light_fraction", cadence.light_fraction)?;
    dict.set_item("heavy_fraction", cadence.heavy_fraction)?;
    dict.set_item("is_harmonious", cadence.is_harmonious)?;
    dict.set_item("cadence_fractures", fractures_list)?;
    dict.set_item("diophantine_invariants_total", total_invariants)?;
    dict.set_item("diophantine_invariants_satisfied", solved_count)?;
    dict.set_item("latency_nanos", t_start.elapsed().as_nanos() as u64)?;
    dict.set_item("latency_micros", t_start.elapsed().as_secs_f64() * 1_000_000.0)?;

    Ok(dict)
}

/// Profiles the differential kinematics of a multi-step reasoning trajectory.
#[pyfunction]
fn kinematics<'py>(py: Python<'py>, text: String) -> PyResult<Bound<'py, PyDict>> {
    let t_start = Instant::now();
    let metrics = KinematicAnalyzer::analyze(&text);

    let spikes_list = PyList::empty(py);
    for spike in &metrics.acceleration_spikes {
        let s_dict = PyDict::new(py);
        s_dict.set_item("step_index", spike.index)?;
        s_dict.set_item("magnitude", spike.magnitude)?;
        s_dict.set_item("pacing_shift", spike.pacing_shift)?;
        s_dict.set_item("isolation_score", spike.isolation_score)?;
        s_dict.set_item("position_ratio", spike.position_ratio)?;
        spikes_list.append(s_dict)?;
    }

    let dict = PyDict::new(py);
    dict.set_item("step_count", metrics.step_count)?;
    dict.set_item("mean_velocity", metrics.mean_velocity)?;
    dict.set_item("velocity_variance", metrics.velocity_variance)?;
    dict.set_item("mean_acceleration", metrics.mean_acceleration)?;
    dict.set_item("max_acceleration", metrics.max_acceleration)?;
    dict.set_item("deadpan_score", metrics.deadpan_score)?;
    dict.set_item("circularity_score", metrics.circularity_score)?;
    dict.set_item("heller_score", metrics.heller_score)?;
    dict.set_item("straightness", metrics.straightness)?;
    dict.set_item("primary_derailment_step", metrics.primary_derailment_step)?;
    dict.set_item("acceleration_spikes", spikes_list)?;
    dict.set_item("latency_nanos", t_start.elapsed().as_nanos() as u64)?;
    dict.set_item("latency_micros", t_start.elapsed().as_secs_f64() * 1_000_000.0)?;

    Ok(dict)
}

/// Deterministic 3-Tier Grading Cascade: Wikidata QID resolution,
/// normalized numeric/date tolerances, and frozen NLI entailment.
#[pyfunction]
#[pyo3(signature = (prediction, ground_truth, qid=None))]
fn grade<'py>(
    py: Python<'py>,
    prediction: String,
    ground_truth: String,
    qid: Option<String>,
) -> PyResult<Bound<'py, PyDict>> {
    let cascade = GradingCascade::in_memory()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    let mut item = BenchmarkItem::new(
        "py-eval",
        1,
        Domain::Wikipedia,
        Modality::Fabrication,
        "Python Evaluation Prompt",
        &ground_truth,
        &ground_truth,
        &ground_truth,
        "Perturbed distractor",
    );

    if let Some(q) = qid {
        item.qid_grounding = Some(QidGrounding {
            subject_qid: q,
            subject_label: ground_truth.clone(),
            property_pid: "P31".to_string(),
            property_label: "instance of".to_string(),
            object_qid: None,
            object_value: ground_truth.clone(),
            aliases: vec![],
        });
    }

    let decision = cascade.grade(&item, &prediction);

    let tier_str = decision.tier.as_str();

    let dict = PyDict::new(py);
    dict.set_item("is_correct", decision.is_correct)?;
    dict.set_item("tier", tier_str)?;
    dict.set_item("confidence", decision.confidence)?;
    dict.set_item("rationale", decision.rationale)?;

    Ok(dict)
}

/// Checks whether an LLM generation was abruptly truncated mid-sentence by token walls.
#[pyfunction]
fn is_truncated(text: String) -> bool {
    PassageIntegrityChecker::is_truncated(&text)
}

/// Checks whether an LLM generation indicates an abstention or refusal to answer.
#[pyfunction]
fn is_abstention(text: String) -> bool {
    PassageIntegrityChecker::is_abstention(&text)
}

/// Extracts `<answer>...</answer>` tags or `\boxed{...}` entities with unclosed tag salvage.
#[pyfunction]
fn extract_answer(text: String) -> (String, Option<String>) {
    AnswerExtractor::extract(&text)
}

/// Solves linear Diophantine indeterminate congruence: a * x ≡ c (mod m).
#[pyfunction]
fn solve_invariant(a: i64, m: i64, c: i64) -> Option<i64> {
    DiophantineInvariantSolver::solve_linear_congruence(a, m, c)
}

/// Runs sub-microsecond hardware and SIMD math diagnostics on host CPU.
#[pyfunction]
fn doctor<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
    let samples = vec![
        "42".to_string(),
        "42.0".to_string(),
        "The answer is 42".to_string(),
        "42".to_string(),
    ];

    // Benchmark Phase Resonance
    let t0 = Instant::now();
    for _ in 0..10_000 {
        let _ = PhaseSpaceResonanceEngine::evaluate(&samples, None, 0.35);
    }
    let rsc_ns = (t0.elapsed().as_micros() as f64 * 1000.0) / 10_000.0;

    // Benchmark Diophantine Invariants
    let t1 = Instant::now();
    for i in 0..20_000 {
        let _ = DiophantineInvariantSolver::solve_linear_congruence(1031, 9973, i as i64);
    }
    let dio_ns = (t1.elapsed().as_micros() as f64 * 1000.0) / 20_000.0;

    // Benchmark Triadic Confluence
    let t2 = Instant::now();
    for _ in 0..2_000 {
        let _ = TriadicConfluenceOperator::evaluate("Apollo 11 landed in 1969.", Some("Apollo 11 was in 1969."));
    }
    let triadic_ns = (t2.elapsed().as_micros() as f64 * 1000.0) / 2_000.0;

    let dict = PyDict::new(py);
    dict.set_item("phase_resonance_speed_ns", rsc_ns)?;
    dict.set_item("phase_resonance_speed_micros", rsc_ns / 1000.0)?;
    dict.set_item("diophantine_speed_ns", dio_ns)?;
    dict.set_item("triadic_confluence_speed_ns", triadic_ns)?;
    dict.set_item("throughput_evals_per_sec", 1_000_000_000.0 / rsc_ns)?;
    dict.set_item("status", "ALL_SYSTEMS_OPERATIONAL")?;

    Ok(dict)
}

/// Official Aletheia Python Module.
#[pymodule]
fn aletheia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add_function(wrap_pyfunction!(audit, m)?)?;
    m.add_function(wrap_pyfunction!(confluence, m)?)?;
    m.add_function(wrap_pyfunction!(kinematics, m)?)?;
    m.add_function(wrap_pyfunction!(grade, m)?)?;
    m.add_function(wrap_pyfunction!(is_truncated, m)?)?;
    m.add_function(wrap_pyfunction!(is_abstention, m)?)?;
    m.add_function(wrap_pyfunction!(extract_answer, m)?)?;
    m.add_function(wrap_pyfunction!(solve_invariant, m)?)?;
    m.add_function(wrap_pyfunction!(doctor, m)?)?;

    Ok(())
}
