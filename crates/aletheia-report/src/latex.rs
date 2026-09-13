use aletheia_metrics::BenchmarkSummary;

pub struct LatexReporter;

impl LatexReporter {
    /// Formats results into publication-ready LaTeX table
    pub fn render_latex_table(summaries: &[BenchmarkSummary]) -> String {
        let mut out = String::new();
        out.push_str("\\begin{table*}[t]\n\\centering\n\\small\n");
        out.push_str("\\begin{tabular}{lccccccc}\n\\toprule\n");
        out.push_str("\\textbf{Model} & \\textbf{Aletheia Score} & \\textbf{Factual (F)} & \\textbf{Epistemic (E)} & \\textbf{Abductive ($\\alpha$)} & \\textbf{E-AUROC} & \\textbf{ECE} & \\textbf{Collapse (\\%)} \\\\\n\\midrule\n");

        for s in summaries {
            let t = &s.truth_tensor;
            let d = &t.diagnostics;
            out.push_str(&format!(
                "{} & {:.3} & {:.3} & {:.3} & {:.3} & {:.3} & {:.3} & {:.1}\\% \\\\\n",
                s.model_id,
                t.aletheia_score,
                t.factual_fidelity,
                t.epistemic_calibration,
                d.mean_abductive_consistency,
                d.epistemic_auroc,
                d.expected_calibration_error,
                d.mode_collapse_rate * 100.0
            ));
        }

        out.push_str("\\bottomrule\n\\end{tabular}\n");
        out.push_str("\\caption{\\textbf{Aletheia Benchmark Leaderboard}: Evaluating factual fidelity, Phase Resonance $R_{sc}$ epistemic calibration, and mode collapse under adversarial pressure.}\n");
        out.push_str("\\label{tab:aletheia_leaderboard}\n\\end{table*}\n");
        out
    }

    /// Formats framework arena comparison into publication-ready LaTeX table
    pub fn render_arena_latex(comparisons: &[aletheia_metrics::FrameworkComparison]) -> String {
        let mut out = String::new();
        out.push_str("\\begin{table*}[t]\n\\centering\n\\small\n");
        out.push_str("\\begin{tabular}{llccccr}\n\\toprule\n");
        out.push_str("\\textbf{Framework/Benchmark} & \\textbf{Category} & \\textbf{Latency} & \\textbf{Cost/10k} & \\textbf{AUROC} & \\textbf{Epistemic} & \\textbf{False Safety} \\\\\n\\midrule\n");

        for c in comparisons {
            let lat = if c.latency_micros < 1000.0 {
                format!("{:.1} $\\mu$s", c.latency_micros)
            } else if c.latency_micros < 1_000_000.0 {
                format!("{:.0} ms", c.latency_micros / 1000.0)
            } else {
                format!("{:.1} s", c.latency_micros / 1_000_000.0)
            };

            let cost = if c.cost_per_10k_usd == 0.0 {
                "\\$0.00".to_string()
            } else {
                format!("\\${:.0}", c.cost_per_10k_usd)
            };

            let epistemic_str = if c.epistemic_quantification { "Yes" } else { "No" };

            let clean_name = c.name.replace('&', "\\&");
            let clean_category = c.category.replace('&', "\\&");

            out.push_str(&format!(
                "{} & {} & {} & {} & {:.3} & {} & {:.1}\\% \\\\\n",
                clean_name,
                clean_category,
                lat,
                cost,
                c.confound_auroc,
                epistemic_str,
                c.false_safety_rate * 100.0
            ));
        }

        out.push_str("\\bottomrule\n\\end{tabular}\n");
        out.push_str("\\caption{\\textbf{Cross-Industry Evaluation Comparison}: Aletheia v$\\infty$ vs. commercial application frameworks and academic benchmarks.}\n");
        out.push_str("\\label{tab:aletheia_arena}\n\\end{table*}\n");
        out
    }
}
