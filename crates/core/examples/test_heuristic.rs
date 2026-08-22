fn main() {
    use figeometrica_core::compile_definition;
    
    let defs = vec![
        ("Aphaeresis", "Aphaeresis is detractio applied to an initial segment of a word, removing one or more letters, sounds, or syllables from its left boundary while preserving the remaining sequence."),
        ("Prothesis", "Prothesis is adjectio applied at the initial boundary of a word, inserting one or more letters, sounds, or syllables before the existing sequence."),
        ("Epenthesis", "Epenthesis is adjectio applied within the interior of a word, inserting one or more letters, sounds, or syllables between existing segments while preserving the original boundary segments."),
        ("Syncope", "Syncope is detractio applied to one or more letters, sounds, or syllables from the interior of a single word, removing a medial unit while preserving the word initial and final boundaries."),
        ("Synaloepha", "Synaloepha is detractio across a word boundary, collapsing two adjacent vowel units belonging to neighboring words by omitting one of them."),
        ("Apocope", "Apocope is the omission of a final letter or syllable from a word."),
        ("Syncope", "Syncope is detractio applied to one or more letters, sounds, or syllables from the interior of a single word, removing a medial unit while preserving the word initial and final boundaries."),
        ("Prothesis", "Prothesis is adjectio applied at the initial boundary of a word, inserting one or more letters, sounds, or syllables before the existing sequence."),
        ("Epenthesis", "Epenthesis is adjectio applied within the interior of a word, inserting one or more letters, sounds, or syllables between existing segments while preserving the original boundary segments."),
        ("Apocope", "Apocope is the omission of a final letter or syllable from a word."),
        ("Synaloepha", "Synaloepha is detractio across a word boundary, collapsing two adjacent vowel units belonging to neighboring words by omitting one of them."),
        ("Contractio", "Contractio is the omission of a medial part of a word, also known as syncope."),
        ("Ecthlipsis", "Ecthlipsis is the elision of a vowel at the end of a word before another vowel, also known as synaloepha."),
        ("Ablatio", "Ablatio is the omission of initial letter or syllable, also known as aphaeresis."),
        ("Abcisi", "Abcisi is the cutting off of the end of a word, also known as apocope."),
    ];
    
    for (name, defn) in defs {
        let result = compile_definition(defn);
        println!("{}: {:?}", name, result.map(|d| (d.pattern.operation, d.pattern.anchor, d.pattern.locus_id, d.confidence)));
    }
}
