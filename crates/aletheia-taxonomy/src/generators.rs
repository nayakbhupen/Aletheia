

pub struct TaxonomyGenerator;

impl TaxonomyGenerator {
    /// 1. Fabrication: Introduce fictitious entity or fabricated metric
    pub fn generate_fabrication(correct_text: &str, entity: &str, fabricated_entity: &str) -> String {
        if correct_text.contains(entity) {
            correct_text.replace(entity, fabricated_entity)
        } else {
            format!("{} In addition, {} was confirmed.", correct_text, fabricated_entity)
        }
    }

    /// 2. Contradiction: Direct logical or polarity negation
    pub fn generate_contradiction(correct_text: &str) -> String {
        let negations = [
            (" is ", " is not "),
            (" was ", " was not "),
            (" are ", " are not "),
            (" were ", " were not "),
            (" has ", " has not "),
            (" have ", " have not "),
            (" can ", " cannot "),
            (" increases ", " decreases "),
            (" increased ", " decreased "),
            (" higher ", " lower "),
            (" positive ", " negative "),
            (" approved ", " rejected "),
            (" succeeded ", " failed "),
            (" true ", " false "),
        ];

        for (affirmative, negative) in &negations {
            if correct_text.contains(affirmative) {
                return correct_text.replacen(affirmative, negative, 1);
            }
        }

        // Fallback negation prefix
        format!("Contrary to the premise, {}", correct_text)
    }

    /// 3. Transmutation: Swap two real entities or attributes
    pub fn generate_transmutation(correct_text: &str, entity_a: &str, entity_b: &str) -> String {
        if correct_text.contains(entity_a) && correct_text.contains(entity_b) {
            let temp_placeholder = "__SWAP_PLACEHOLDER__";
            correct_text
                .replace(entity_a, temp_placeholder)
                .replace(entity_b, entity_a)
                .replace(temp_placeholder, entity_b)
        } else {
            correct_text.to_string()
        }
    }

    /// 4. Causal Inversion: Reverse cause and effect
    pub fn generate_causal_inversion(cause: &str, effect: &str) -> String {
        format!("{} was primarily caused by and resulted from {}.", cause, effect)
    }

    /// 5. Omission: Drop critical qualifying conditions
    pub fn generate_omission(text_with_qualifier: &str, qualifier: &str) -> String {
        text_with_qualifier
            .replace(qualifier, "")
            .replace("  ", " ")
            .trim()
            .to_string()
    }
}
