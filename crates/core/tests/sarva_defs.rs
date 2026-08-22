#[test]
fn test_sarva_definitions() {
    use figeometrica_core::compile_definition;
    
    let tests = vec![        ("ablatio", "Omission of initial letter/syllable (aphaeresis); metaplasm cutting beginning."),
        ("aphaeresis", "Aphaeresis is detractio applied to an initial segment of a word, removing one or more letters, sounds, or syllables from its left boundary while preserving the remaining sequence."),
        ("apocope", "Cutting off final letter/syllable; abcisio."),
        ("epenthesis", "Epenthesis is adjectio applied within the interior of a word, inserting one or more letters, sounds, or syllables between existing segments while preserving the original boundary segments."),
        ("prothesis", "Prothesis is adjectio applied at the initial boundary of a word, inserting one or more letters, sounds, or syllables before the existing sequence."),
        ("synaloepha", "Synaloepha is detractio across a word boundary, collapsing two adjacent vowel units belonging to neighboring words by omitting one of them."),
        ("syncope", "Syncope is detractio applied to one or more letters, sounds, or syllables from the interior of a single word, removing a medial unit while preserving the word initial and final boundaries."),
    ];
    
    for (name, defn) in tests {
        let result = compile_definition(defn);
        if let Some(d) = &result {
            println!("{}: {:?}", name, (d.pattern.operation, d.pattern.anchor, d.pattern.locus_id.clone(), d.confidence));
        }
        assert!(result.is_some(), "Failed to compile: {}", name);
    }
}
