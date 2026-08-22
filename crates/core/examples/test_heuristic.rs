fn main() {
    use figeometrica_core::compile_definition;
    
    let defs = vec![
        ("Aphaeresis", "Aphaeresis is detractio applied to an initial segment of a word, removing one or more letters, sounds, or syllables from its left boundary while preserving the remaining sequence."),
        ("Prothesis", "Prothesis is adjectio applied at the initial boundary of a word, inserting one or more letters, sounds, or syllables before the existing sequence."),
        ("Epenthesis", "Epenthesis is adjectio applied within the interior of a word, inserting one or more letters, sounds, or syllables between existing segments while preserving the original boundary segments."),
        ("Syncope", "Syncope is detractio applied to one or more letters, sounds, or syllables from the interior of a single word, removing a medial unit while preserving the word initial and final boundaries."),
        ("Synaloepha", "Synaloepha is detractio across a word boundary, collapsing two adjacent vowel units belonging to neighboring words by omitting one of them."),
        ("Apocope", "Cutting off final letter/syllable; abcisio."),
    ];
    
    for (name, defn) in defs {
        let result = compile_definition(defn);
        println!("{}: {:?}", name, result.map(|d| (d.pattern.operation, d.pattern.anchor, d.pattern.locus_id, d.confidence)));
    }
}
