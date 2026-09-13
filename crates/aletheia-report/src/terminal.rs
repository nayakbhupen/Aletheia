use aletheia_metrics::BenchmarkSummary;
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Row, Table};

pub struct TerminalReporter;

impl TerminalReporter {
    /// Renders a world-class benchmark leaderboard in the terminal
    pub fn render_leaderboard(summaries: &[BenchmarkSummary]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Rank").fg(Color::Cyan),
                Cell::new("Model").fg(Color::Cyan),
                Cell::new("Aletheia Score (v∞)").fg(Color::Yellow),
                Cell::new("Factual Fidelity (F)").fg(Color::Green),
                Cell::new("Epistemic Cal (E)").fg(Color::Magenta),
                Cell::new("Abductive (α)").fg(Color::Cyan),
                Cell::new("Curvature (κ)").fg(Color::DarkCyan),
                Cell::new("E-AUROC").fg(Color::Blue),
                Cell::new("ECE").fg(Color::Red),
                Cell::new("Mode Collapse %").fg(Color::DarkRed),
                Cell::new("Latency (µs)").fg(Color::White),
            ]);

        let mut sorted = summaries.to_vec();
        sorted.sort_by(|a, b| {
            b.truth_tensor
                .aletheia_score
                .partial_cmp(&a.truth_tensor.aletheia_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (rank, summary) in sorted.iter().enumerate() {
            let t = &summary.truth_tensor;
            let d = &t.diagnostics;

            table.add_row(Row::from(vec![
                Cell::new(format!("#{}", rank + 1)),
                Cell::new(&summary.model_id),
                Cell::new(format!("{:.3}", t.aletheia_score)).fg(Color::Yellow),
                Cell::new(format!("{:.3}", t.factual_fidelity)).fg(Color::Green),
                Cell::new(format!("{:.3}", t.epistemic_calibration)).fg(Color::Magenta),
                Cell::new(format!("{:.3}", d.mean_abductive_consistency)).fg(Color::Cyan),
                Cell::new(format!("{:.3}", d.mean_attractor_curvature)).fg(Color::DarkCyan),
                Cell::new(format!("{:.3}", d.epistemic_auroc)),
                Cell::new(format!("{:.3}", d.expected_calibration_error)),
                Cell::new(format!("{:.1}%", d.mode_collapse_rate * 100.0)),
                Cell::new(format!("{:.1}", d.mean_latency_micros)),
            ]));
        }

        format!(
            "\n{}\n{}\n",
            "═════════════════════ ALETHEIA v∞ LEADERBOARD ═════════════════════".bold().cyan(),
            table
        )
    }

    /// Renders a comprehensive shootout arena table comparing Aletheia against top industry frameworks & benchmarks
    pub fn render_arena_leaderboard(comparisons: &[aletheia_metrics::FrameworkComparison]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Framework / Benchmark").fg(Color::Cyan),
                Cell::new("Category").fg(Color::White),
                Cell::new("Evaluation Method").fg(Color::Yellow),
                Cell::new("Latency").fg(Color::Green),
                Cell::new("Speedup").fg(Color::Green),
                Cell::new("Cost / 10K").fg(Color::Magenta),
                Cell::new("Confound AUROC").fg(Color::Blue),
                Cell::new("Epistemic").fg(Color::Cyan),
                Cell::new("Agentic").fg(Color::DarkCyan),
                Cell::new("Deterministic").fg(Color::White),
                Cell::new("False Safety %").fg(Color::Red),
            ]);

        for c in comparisons {
            let latency_str = if c.latency_micros < 1000.0 {
                format!("{:.2} µs", c.latency_micros)
            } else if c.latency_micros < 1_000_000.0 {
                format!("{:.1} ms", c.latency_micros / 1000.0)
            } else {
                format!("{:.2} s", c.latency_micros / 1_000_000.0)
            };

            let speedup_str = if c.speedup_vs_aletheia <= 1.0 {
                "1.0x (BASELINE)".bold().green().to_string()
            } else if c.speedup_vs_aletheia < 1000.0 {
                format!("{:.0}x slower", c.speedup_vs_aletheia)
            } else {
                format!("{:.0}k x slower", c.speedup_vs_aletheia / 1000.0)
            };

            let cost_str = if c.cost_per_10k_usd == 0.0 {
                "$0.00 (FREE)".bold().green().to_string()
            } else {
                format!("${:.2}", c.cost_per_10k_usd)
            };

            let auroc_color = if c.confound_auroc <= 0.52 {
                Color::Green
            } else if c.confound_auroc <= 0.65 {
                Color::Yellow
            } else {
                Color::Red
            };

            table.add_row(Row::from(vec![
                Cell::new(&c.name).fg(if c.name.contains("Aletheia") { Color::Cyan } else { Color::White }),
                Cell::new(&c.category),
                Cell::new(&c.evaluation_method),
                Cell::new(latency_str).fg(if c.latency_micros < 10.0 { Color::Green } else { Color::White }),
                Cell::new(speedup_str),
                Cell::new(cost_str),
                Cell::new(format!("{:.3}", c.confound_auroc)).fg(auroc_color),
                Cell::new(if c.epistemic_quantification { "✅ YES" } else { "❌ NO" }),
                Cell::new(if c.agentic_tool_aware { "✅ YES" } else { "❌ NO" }),
                Cell::new(if c.deterministic_reproducible { "✅ YES" } else { "❌ NO" }),
                Cell::new(format!("{:.1}%", c.false_safety_rate * 100.0)).fg(if c.false_safety_rate < 0.05 { Color::Green } else { Color::Red }),
            ]));
        }

        format!(
            "\n{}\n{}\n",
            "══════════════════ ALETHEIA v∞ vs. INDUSTRY BENCHMARKS & EVAL FRAMEWORKS ══════════════════".bold().cyan(),
            table
        )
    }
}
