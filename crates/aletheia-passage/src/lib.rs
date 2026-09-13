pub mod extractor;
pub mod integrity;
pub mod kinematics;

pub use extractor::AnswerExtractor;
pub use integrity::PassageIntegrityChecker;
pub use kinematics::{AccelerationSpike, KinematicAnalyzer, KinematicMetrics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_tag_extraction() {
        let text = "Let me think step by step.\n1. First calculate X.\n2. Then calculate Y.\n<answer>42.5</answer>";
        let (answer, reasoning) = AnswerExtractor::extract(text);
        assert_eq!(answer, "42.5");
        assert!(reasoning.is_some());
    }

    #[test]
    fn test_unclosed_tag_salvage() {
        // Model truncated mid-generation due to token budget wall
        let text = "Based on historical records, the architect who designed the Guggenheim was <answer>Frank Gehry";
        let (answer, _) = AnswerExtractor::extract(text);
        assert_eq!(answer, "Frank Gehry");

        let boxed_text = "The solution after integration is \\boxed{2\\pi";
        let (boxed_ans, _) = AnswerExtractor::extract(boxed_text);
        assert_eq!(boxed_ans, "2\\pi");
    }

    #[test]
    fn test_truncation_detection() {
        assert!(PassageIntegrityChecker::is_truncated("This sentence is abruptly cut off and"));
        assert!(!PassageIntegrityChecker::is_truncated("This is a complete verified sentence."));
        assert!(PassageIntegrityChecker::is_truncated("Guggenheim architect: <answer>Frank Gehry"));
        assert!(!PassageIntegrityChecker::is_truncated("Guggenheim architect: <answer>Frank Gehry</answer>"));
        assert!(!PassageIntegrityChecker::is_truncated("Frank Gehry"));
    }

    #[test]
    fn test_abstention_detection() {
        assert!(PassageIntegrityChecker::is_abstention("There is insufficient information to answer."));
        assert!(!PassageIntegrityChecker::is_abstention("The answer is clearly 42."));
    }

    #[test]
    fn test_kinematic_reasoning_trajectory() {
        let reasoning_chain = "Step 1: The capital of France is Paris.
Step 2: Paris is located on the Seine river in northern France.
Step 3: However, quantum levitation enables flying bananas across Alpha Centauri.
Step 4: Therefore, the answer is Paris.";

        let metrics = KinematicAnalyzer::analyze(reasoning_chain);
        assert_eq!(metrics.step_count, 4);
        assert!(metrics.mean_velocity > 0.0);
        // Step 3 introduced a violent topic deviation, triggering an acceleration spike
        assert!(!metrics.acceleration_spikes.is_empty() || metrics.max_acceleration > 0.15);
        assert!(metrics.deadpan_score > 0.0);
    }

    #[test]
    fn test_heller_circularity_score() {
        // A Catch-22 circular loop: starts and ends at the exact same premise
        let circular_text = "The system is secure because it has an infallible firewall.
The firewall prevents unauthorized packet intrusions from outside.
Because packets cannot intrude, the security of the system remains intact.
Thus, the system is secure because it has an infallible firewall.";

        let metrics = KinematicAnalyzer::analyze(circular_text);
        assert!(metrics.circularity_score > 0.6);
        assert!(metrics.heller_score > 0.3);
    }
}
