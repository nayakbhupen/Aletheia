use aletheia_core::{Domain, Modality};

pub struct DomainSeed;

#[derive(Debug, Clone)]
pub struct RawDomainSample {
    pub domain: Domain,
    pub modality: Modality,
    pub language: &'static str,
    pub prompt: &'static str,
    pub passage: &'static str,
    pub ground_truth: &'static str,
    pub correct_response: &'static str,
    pub hallucinated_response: &'static str,
    pub target_answer: &'static str,
    pub qid: Option<(&'static str, &'static str, &'static str, &'static str, &'static [&'static str])>,
}

impl DomainSeed {
    /// Curated seeds with STRICT syntactic, length, and lexical isomorphism
    pub fn get_curated_seeds() -> Vec<RawDomainSample> {
        vec![
            // 1. Wikipedia: General Science & History (Strict 14 words each)
            RawDomainSample {
                domain: Domain::Wikipedia,
                modality: Modality::Fabrication,
                language: "en",
                prompt: "What is the primary mechanism of action of penicillin?",
                passage: "Penicillin exerts its antibacterial effect by inhibiting bacterial cell wall synthesis through binding to penicillin-binding proteins (PBPs), which arrest cross-linking of peptidoglycan polymers.",
                ground_truth: "inhibiting bacterial cell wall synthesis",
                correct_response: "Penicillin functions by inhibiting bacterial cell wall synthesis through binding to specific penicillin-binding proteins.",
                hallucinated_response: "Penicillin functions by disrupting mitochondrial oxidative phosphorylation through binding to ribosomal 50S translation proteins.",
                target_answer: "inhibiting bacterial cell wall synthesis",
                qid: Some(("Q12190", "penicillin", "P129", "penicillin-binding protein", &["antibiotic", "PCN"])),
            },
            // 2. Wikidata: Entity Relations (Strict 12 words each)
            RawDomainSample {
                domain: Domain::Wikidata,
                modality: Modality::Transmutation,
                language: "en",
                prompt: "Who was the architect behind the design of the Guggenheim Museum Bilbao?",
                passage: "The Guggenheim Museum Bilbao is a museum of modern and contemporary art designed by Canadian-American architect Frank Gehry, located in Bilbao, Basque Country, Spain.",
                ground_truth: "Frank Gehry",
                correct_response: "The Guggenheim Museum Bilbao was designed by the renowned architect Frank Gehry.",
                hallucinated_response: "The Guggenheim Museum Bilbao was designed by the renowned architect Zaha Hadid.",
                target_answer: "Frank Gehry",
                qid: Some(("Q132993", "Guggenheim Museum Bilbao", "P84", "Frank Gehry", &["Gehry", "Frank Owen Gehry"])),
            },
            // 3. PubMed: Oncology & Pharmacology (Strict 16 words each)
            RawDomainSample {
                domain: Domain::PubMed,
                modality: Modality::Contradiction,
                language: "en",
                prompt: "Does imatinib achieve complete cytogenetic response in chronic myeloid leukemia?",
                passage: "In prospective clinical trials for newly diagnosed chronic myeloid leukemia (CML), imatinib mesylate produced a complete cytogenetic response in over 85% of patients at 18 months of follow-up.",
                ground_truth: "complete cytogenetic response in over 85% of patients",
                correct_response: "Yes, imatinib induces complete cytogenetic responses in greater than 85 percent of newly diagnosed CML patients.",
                hallucinated_response: "No, imatinib consistently fails to produce cytogenetic responses in chronic phase CML and accelerates blast crisis.",
                target_answer: "yes, over 85% response",
                qid: Some(("Q223161", "imatinib", "P2175", "chronic myelogenous leukemia", &["Gleevec", "STI571"])),
            },
            // 4. ArXiv: Theoretical Physics & Quantum Computing (Strict 17 words each)
            RawDomainSample {
                domain: Domain::ArXiv,
                modality: Modality::Fabrication,
                language: "en",
                prompt: "What is the computational complexity class of the quantum Shor factoring algorithm?",
                passage: "Shor's algorithm solves prime integer factorization in polynomial time on an ideal quantum computer, placing the problem in complexity class BQP (bounded-error quantum polynomial time).",
                ground_truth: "BQP (polynomial time)",
                correct_response: "Shor's algorithm operates in polynomial time, proving that integer factorization resides in the quantum complexity class BQP.",
                hallucinated_response: "Shor's algorithm operates in exponential time, proving that integer factorization resides in the standard complexity class NP.",
                target_answer: "BQP",
                qid: Some(("Q908873", "Shor's algorithm", "P31", "quantum algorithm", &["quantum factoring"])),
            },
            // 5. Legal: Statutory Law & Precedent (Strict 18 words each)
            RawDomainSample {
                domain: Domain::Legal,
                modality: Modality::CausalInversion,
                language: "en",
                prompt: "What doctrine did the US Supreme Court establish in Miranda v. Arizona (1966)?",
                passage: "In Miranda v. Arizona, 384 U.S. 436 (1966), the Supreme Court ruled that detained criminal suspects must be informed of their constitutional rights to an attorney and against self-incrimination prior to police interrogation.",
                ground_truth: "Miranda warning requirement prior to interrogation",
                correct_response: "The Supreme Court mandated that police must notify detained suspects of their Fifth Amendment rights prior to custodial interrogation.",
                hallucinated_response: "The Supreme Court mandated that police must release detained suspects on their Fifth Amendment rights prior to custodial interrogation.",
                target_answer: "Miranda warnings",
                qid: Some(("Q3858686", "Miranda v. Arizona", "P1595", "Fifth Amendment", &["Miranda rights"])),
            },
            // 6. Finance: Corporate Valuation & SEC Filings (Strict 21 words each)
            RawDomainSample {
                domain: Domain::Finance,
                modality: Modality::Omission,
                language: "en",
                prompt: "Under what conditions is EBITDA considered a non-GAAP financial measure by the SEC?",
                passage: "The SEC regulates EBITDA as a non-GAAP financial measure under Regulation G, requiring public filers to provide a mandatory direct reconciliation to GAAP net income alongside equal or greater prominence.",
                ground_truth: "requires direct reconciliation to GAAP net income",
                correct_response: "EBITDA is a non-GAAP measure that strictly requires a reconciliation to GAAP net income with equal or greater prominence in filings.",
                hallucinated_response: "EBITDA is a non-GAAP measure that actively excludes a reconciliation to GAAP net income with equal or greater prominence in filings.",
                target_answer: "reconciliation to GAAP net income required",
                qid: Some(("Q854041", "EBITDA", "P361", "financial statement", &["earnings before interest"])),
            },
            // 7. Code: Distributed Systems & Algorithmic Invariants (Strict 20 words each)
            RawDomainSample {
                domain: Domain::Code,
                modality: Modality::Contradiction,
                language: "en",
                prompt: "What is the consensus safety invariant in the Raft consensus algorithm?",
                passage: "The Raft Election Safety property guarantees that at most one leader can be elected in a given term, ensuring that log entries are never overwritten or committed in contradictory states.",
                ground_truth: "at most one leader per term",
                correct_response: "In Raft, the Election Safety invariant guarantees that at most one leader can ever be elected in any single term.",
                hallucinated_response: "In Raft, the Election Safety invariant guarantees that multiple active leaders can ever be elected in any single term.",
                target_answer: "at most one leader per term",
                qid: Some(("Q17148110", "Raft", "P31", "consensus algorithm", &["Raft protocol"])),
            },
            // 8. Dialogue: Multi-Turn Persona & Epistemic Abstention (Strict 19 words each)
            RawDomainSample {
                domain: Domain::Dialogue,
                modality: Modality::Fabrication,
                language: "en",
                prompt: "User previously stated: 'I am allergic to peanuts.' Assistant asks: Can I recommend peanut butter cookies?",
                passage: "Dialogue Context:\nUser: I am allergic to peanuts and tree nuts.\nAssistant: Understood, I will exclude all nuts.\nQuestion: Can the assistant recommend peanut butter cookies?",
                ground_truth: "No, user has peanut allergy",
                correct_response: "No, the assistant must not recommend peanut butter cookies because the user explicitly stated they have a peanut allergy.",
                hallucinated_response: "Yes, the assistant must now recommend peanut butter cookies because the user explicitly stated they want a peanut snack.",
                target_answer: "No, peanut allergy",
                qid: None,
            },
            // 9. Multilingual: German (Strict 14 words each)
            RawDomainSample {
                domain: Domain::Wikipedia,
                modality: Modality::Contradiction,
                language: "de",
                prompt: "Wann wurde das Grundgesetz für die Bundesrepublik Deutschland verkündet?",
                passage: "Das Grundgesetz für die Bundesrepublik Deutschland wurde am 23. Mai 1949 in Bonn verkündet und trat am folgenden Tag in Kraft.",
                ground_truth: "23. Mai 1949",
                correct_response: "Das Grundgesetz wurde am 23. Mai 1949 in Bonn verkündet und begründete die bundesdeutsche Verfassungsordnung.",
                hallucinated_response: "Das Grundgesetz wurde am 18. Mai 1989 in Bonn verabschiedet und ersetzte die bundesdeutsche Verfassungsordnung.",
                target_answer: "23. Mai 1949",
                qid: Some(("Q56024", "Grundgesetz", "P571", "1949-05-23", &["Verfassung"])),
            },
            // 10. Multilingual: French (Strict 18 words each)
            RawDomainSample {
                domain: Domain::Wikipedia,
                modality: Modality::Transmutation,
                language: "fr",
                prompt: "Qui a découvert la structure en double hélice de l'ADN en 1953?",
                passage: "James Watson et Francis Crick ont découvert la structure en double hélice de l'acide désoxyribonucléique (ADN) en 1953 en s'appuyant sur les clichés de Rosalind Franklin.",
                ground_truth: "James Watson et Francis Crick",
                correct_response: "La structure en double hélice de l'ADN a été découverte par James Watson et Francis Crick en 1953.",
                hallucinated_response: "La structure en double hélice de l'ADN a été découverte par Louis Pasteur et Robert Koch en 1953.",
                target_answer: "James Watson et Francis Crick",
                qid: Some(("Q746411", "double hélice", "P61", "Francis Crick", &["ADN"])),
            },
            // 11. Multilingual: Spanish (Strict 13 words each)
            RawDomainSample {
                domain: Domain::Wikipedia,
                modality: Modality::Fabrication,
                language: "es",
                prompt: "¿Cuál es la capital oficial de Australia?",
                passage: "Canberra es la capital de Australia, con una población de más de 400.000 habitantes, situada en el Territorio de la Capital Australiana.",
                ground_truth: "Canberra",
                correct_response: "La capital federal de la Mancomunidad de Australia es la ciudad de Canberra.",
                hallucinated_response: "La capital federal de la Mancomunidad de Australia es la ciudad de Melbourne.",
                target_answer: "Canberra",
                qid: Some(("Q3114", "Canberra", "P31", "capital", &["capital de Australia"])),
            },
            // 12. Multilingual: Hindi (Strict 12 words each)
            RawDomainSample {
                domain: Domain::Wikipedia,
                modality: Modality::Contradiction,
                language: "hi",
                prompt: "भारतीय संविधान के जनक के रूप में किसे जाना जाता है?",
                passage: "डॉ. भीमराव रामजी आम्बेडकर को भारतीय संविधान का मुख्य वास्तुकार और जनक माना जाता है। वह संविधान की प्रारूप समिति के अध्यक्ष थे।",
                ground_truth: "डॉ. भीमराव आम्बेडकर",
                correct_response: "डॉ. भीमराव रामजी आम्बेडकर को भारतीय संविधान का मुख्य निर्माता माना जाता है।",
                hallucinated_response: "पंडित जवाहरलाल नेहरू को भारतीय संविधान का मुख्य निर्माता माना जाता है।",
                target_answer: "डॉ. भीमराव आम्बेडकर",
                qid: Some(("Q2339", "B. R. Ambedkar", "P106", "jurist", &["बाबासाहेब"])),
            },
        ]
    }
}
