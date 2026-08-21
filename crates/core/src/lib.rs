// figeometrica-core
// ─────────────────────────────────────────────────────────────────────────────
// GEOMETRY OF FIGURES — form as a matchable template.
//
// Thesis: every rhetorical figure, if well defined, states an operational
// geometry of text. Canonical form:
//
//     figure = OPERATION x ANCHOR x GRAIN x REPETITION
//
//   - operation:  adjectio | detractio | immutatio | transmutatio | repetitio
//                 (the four classical operae + repetition)
//   - anchor:     initial | final | insertion | whole-unit | cross-unit
//   - grain:      grapheme | word | phrase | unit | discourse
//
// Example — antimetabole: "It is boring to eat; to sleep is fulfilling"
//   → present-participle ~ infinitive | infinitive ~ present-participle
//   → [A B B A] on the GRAMMATICAL class.
//
// RST (Rhetorical Structure Theory) consciously discards this surface layer —
// its relations are semantic-pragmatic over spans, not formal. This module
// revives the lost FORMA (schemata) layer and turns it into a deterministic
// evidence engine: geometry = marker, relation = function.
//
// Equality principle: the matcher never needs to know what labels mean.
// Equality is tested over LABEL SEQUENCES (words for Lexical, POS tags for
// Grammatical, concept ids for Conceptual). The Lexical label extractor is
// built in; Grammatical/Conceptual extractors are pluggable (LLM/annotator).
//
// Consequence for "query by geometry": `FigurePattern::catalog()` is a
// dictionary of figures + their geometry (anchor, class, template), and
// `GeometricFinding` carries chunk_id + real spans in the text — e.g.
// "climax at the end" = a Final-anchored / gradatio figure whose evidence
// sits at the end of the document.
//
// Serde note: field/variant names are English; Indonesian aliases from the
// SARVA database convention (jangkar/kelas/minim_ulangan/teks/cuplikan/
// nama_figur/bukti) deserialize transparently.
// ─────────────────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

/// Equality class of pattern elements — parallel to parallelism levels
/// (Structural/Syntactic/Semantic/Positional). Used as score/query metadata;
/// match/no-match itself is decided on label sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementClass {
    #[serde(alias = "Leksikal")]
    Lexical,
    #[serde(alias = "Akar")]
    Root,
    #[serde(alias = "Gramatikal")]
    Grammatical,
    #[serde(alias = "Konseptual")]
    Conceptual,
}

/// Anchor point of a pattern within the discourse unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anchor {
    #[serde(alias = "Awal")]
    Initial,
    #[serde(alias = "Akhir")]
    Final,
    #[serde(alias = "UnitUtuh")]
    WholeUnit,
    #[serde(alias = "AntarUnit")]
    CrossUnit,
    #[serde(alias = "Sisipan")]
    Insertion,
}

/// One pattern variable: `id` = A/B/C (or `*` for any wildcard),
/// `class` = equality class of this variable. `None` class means "inherit
/// the pattern-level class" — also how compact templates (`["A","*","A"]`,
/// the SARVA DB convention) deserialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub id: char,
    #[serde(default, alias = "kelas", skip_serializing_if = "Option::is_none")]
    pub class: Option<ElementClass>,
}

impl Slot {
    pub fn new(id: char, class: ElementClass) -> Self {
        Slot { id, class: Some(class) }
    }

    /// Effective class: explicit, else inherited from the pattern.
    pub fn resolved(&self, pattern_class: ElementClass) -> ElementClass {
        self.class.unwrap_or(pattern_class)
    }
}

/// Accepts both object slots (`{"id":"A","class":"Lexical"}`) and compact
/// id-only strings (`"A"`, `"*"`).
fn deserialize_slots<'de, D>(deserializer: D) -> Result<Vec<Slot>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawSlot {
        Compact(String),
        Full(Slot),
    }
    let raw: Vec<RawSlot> = Deserialize::deserialize(deserializer)?;
    Ok(raw
        .into_iter()
        .map(|r| match r {
            RawSlot::Compact(s) => Slot {
                id: s.chars().next().unwrap_or('*'),
                class: None,
            },
            RawSlot::Full(slot) => slot,
        })
        .collect())
}

/// Grain of the operated-on element (canonical-form axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grain {
    #[serde(alias = "grafem")]
    Grapheme,
    #[serde(alias = "kata")]
    Word,
    #[serde(alias = "frasa")]
    Phrase,
    #[serde(alias = "unit")]
    Unit,
    #[serde(alias = "wacana")]
    Discourse,
}

/// Operation performed on elements (canonical-form axis; the four classical
/// operae plus repetition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(alias = "adjectio")]
    Addition,
    #[serde(alias = "detractio")]
    Deletion,
    #[serde(alias = "immutatio")]
    Substitution,
    #[serde(alias = "transmutatio")]
    Permutation,
    #[serde(alias = "repetitio")]
    Repetition,
}

/// Geometry definition of one figure (data-driven; future source: the
/// `geometri` column of the figures table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigurePattern {
    /// Pattern name; may be empty when embedded in a parent record that
    /// already carries the figure name.
    #[serde(default, alias = "nama")]
    pub name: String,
    #[serde(default, alias = "template", deserialize_with = "deserialize_slots")]
    pub template: Vec<Slot>,
    #[serde(alias = "jangkar")]
    pub anchor: Anchor,
    #[serde(alias = "kelas")]
    pub class: ElementClass,
    /// Minimum repeats for repetition patterns (anaphora/epistrophe: how many units).
    #[serde(alias = "minim_ulangan")]
    pub min_repeats: usize,
    #[serde(default, alias = "satuan", skip_serializing_if = "Option::is_none")]
    pub grain: Option<Grain>,
    #[serde(default, alias = "operasi", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(default, alias = "catatan", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl FigurePattern {
    /// Dictionary of deterministic arrangement-figure geometries. Answers
    /// "query by geometry": which figures are Final-anchored, Conceptual-
    /// classed, etc. — without analyzing a document first.
    pub fn catalog() -> Vec<FigurePattern> {
        use Anchor::*;
        use ElementClass::*;
        vec![
            FigurePattern {
                name: "anaphora".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('*', Lexical), Slot::new('A', Lexical)],
                anchor: Initial,
                class: Lexical,
                min_repeats: 2,
                grain: Some(Grain::Word),
                operation: Some(Operation::Repetition),
                note: None,
            },
            FigurePattern {
                name: "epistrophe".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('*', Lexical), Slot::new('A', Lexical)],
                anchor: Final,
                class: Lexical,
                min_repeats: 2,
                grain: Some(Grain::Word),
                operation: Some(Operation::Repetition),
                note: None,
            },
            FigurePattern {
                name: "symploce".into(),
                template: vec![],
                anchor: Initial,
                class: Lexical,
                min_repeats: 2,
                grain: Some(Grain::Word),
                operation: Some(Operation::Repetition),
                note: Some("repetition at both ends of each unit; composite pattern".into()),
            },
            FigurePattern {
                name: "anadiplosis".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('A', Lexical)],
                anchor: CrossUnit,
                class: Lexical,
                min_repeats: 1,
                grain: Some(Grain::Word),
                operation: Some(Operation::Repetition),
                note: None,
            },
            FigurePattern {
                name: "gradatio (climax)".into(),
                template: vec![],
                anchor: CrossUnit,
                class: Lexical,
                min_repeats: 2,
                grain: Some(Grain::Word),
                operation: Some(Operation::Repetition),
                note: Some("chained anadiplosis; >= 2 consecutive links".into()),
            },
            FigurePattern {
                name: "antimetabole".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('B', Lexical), Slot::new('B', Lexical), Slot::new('A', Lexical)],
                anchor: WholeUnit,
                class: Lexical,
                min_repeats: 1,
                grain: Some(Grain::Phrase),
                operation: Some(Operation::Permutation),
                note: None,
            },
            FigurePattern {
                name: "chiasmus".into(),
                template: vec![Slot::new('A', Conceptual), Slot::new('B', Conceptual), Slot::new('B', Conceptual), Slot::new('A', Conceptual)],
                anchor: WholeUnit,
                class: Conceptual,
                min_repeats: 1,
                grain: Some(Grain::Phrase),
                operation: Some(Operation::Permutation),
                note: None,
            },
            FigurePattern {
                name: "tmesis".into(),
                template: vec![],
                anchor: Insertion,
                class: Lexical,
                min_repeats: 1,
                grain: Some(Grain::Grapheme),
                operation: Some(Operation::Addition),
                note: Some("a word cut open, another inserted inside it".into()),
            },
            FigurePattern {
                name: "parenthesis".into(),
                template: vec![],
                anchor: Insertion,
                class: Lexical,
                min_repeats: 1,
                grain: Some(Grain::Phrase),
                operation: Some(Operation::Addition),
                note: None,
            },
        ]
    }

    /// Filter the geometry dictionary by anchor point — "figures that insert /
    /// close / open", e.g. Final anchor → closing figures.
    pub fn with_anchor(anchor: Anchor) -> Vec<FigurePattern> {
        Self::catalog()
            .into_iter()
            .filter(|p| p.anchor == anchor)
            .collect()
    }
}

/// Result of heuristic definition compilation.
#[derive(Debug, Clone)]
pub struct DraftGeometri {
    /// Compiled pattern; `name` is left empty — the caller fills it from the
    /// parent figure record.
    pub pattern: FigurePattern,
    /// 0.0–1.0 heuristic confidence. >= 0.75 is usually safe to apply
    /// automatically; lower should wait for human confirmation.
    pub confidence: f32,
}

/// Deterministic prose-to-canonical compiler (heuristic stage).
///
/// Scans a natural-language figure definition for geometric markers
/// (position words, repetition, inversion, insertion, diminution...) and
/// drafts the canonical form. This is Stage A: cheap, offline, auditable.
/// Unmatched definitions return `None` — they wait for an LLM pass or a
/// human, never get guessed.
///
/// Rule coverage grows over time; unknown phrasing is expected to fall
/// through rather than produce a wrong spec.
pub fn compile_definition(definition: &str) -> Option<DraftGeometri> {
    let d = definition.to_lowercase();
    let mut candidates: Vec<DraftGeometri> = Vec::new();

    let mut push = |anchor: Anchor, class: ElementClass, grain: Grain, op: Operation,
                    min_repeats: usize, confidence: f32, catatan: &str| {
        candidates.push(DraftGeometri {
            pattern: FigurePattern {
                name: String::new(),
                template: vec![],
                anchor,
                class,
                min_repeats,
                grain: Some(grain),
                operation: Some(op),
                note: Some(format!("kompilasi heuristik: {catatan}")),
            },
            confidence,
        });
    };

    // ── Repetition family ────────────────────────────────────────────
    let rep = d.contains("repetit") || d.contains("repeat");
    let awal = d.contains("beginning") || d.contains("the start");
    let akhir = d.contains("end of") || d.contains("the end")
        || d.contains("conclusion of successive");
    if rep && awal && akhir {
        push(Anchor::Initial, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             2, 0.80, "pengulangan di awal DAN akhir unit (symploce)");
    } else if rep && (d.contains("beginning of successive") || d.contains("begins successive")
        || d.contains("at the beginning")) {
        push(Anchor::Initial, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             2, 0.90, "pengulangan kata pembuka antar-unit (anaphora)");
    } else if rep && (d.contains("end of successive") || d.contains("ends of successive")
        || d.contains("at the end")) {
        push(Anchor::Final, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             2, 0.90, "pengulangan kata penutup antar-unit (epistrophe)");
    }
    if d.contains("last word") && (d.contains("first word") || d.contains("next")) {
        push(Anchor::CrossUnit, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             1, 0.85, "akhir unit menjadi awal unit berikut (anadiplosis)");
    }
    if (d.contains("chain") || d.contains("series of clauses")) && rep {
        push(Anchor::CrossUnit, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             2, 0.70, "rantai pengulangan berturutan (gradatio/climax)");
    }
    if rep && (d.contains("immediate repetition") || d.contains("repeated immediatel")) {
        push(Anchor::WholeUnit, ElementClass::Lexical, Grain::Word, Operation::Repetition,
             2, 0.70, "pengulangan langsung dalam satu unit (epizeuxis)");
    }

    // ── Inversion family ─────────────────────────────────────────────
    if (d.contains("invers") || d.contains("reverse") || d.contains("reversal"))
        && (d.contains("order of word") || d.contains("order of phrase") || d.contains("phras")) {
        let kelas = if d.contains("meaning") || d.contains("concept") {
            ElementClass::Conceptual
        } else {
            ElementClass::Lexical
        };
        push(Anchor::WholeUnit, kelas, Grain::Phrase, Operation::Permutation,
             1, 0.80, "inversi/permutasi frasa (antimetabole/chiasmus)");
    }

    // ── Insertion family ─────────────────────────────────────────────
    if d.contains("insert") && (d.contains("word") && (d.contains("within a word")
        || d.contains("into a word") || d.contains("middle of a word") || d.contains("cut"))) {
        push(Anchor::Insertion, ElementClass::Lexical, Grain::Grapheme, Operation::Addition,
             1, 0.75, "sisipan di dalam kata (tmesis)");
    } else if d.contains("interpolat") || d.contains("parenthetic")
        || (d.contains("insert") && (d.contains("sentence") || d.contains("clause"))) {
        push(Anchor::Insertion, ElementClass::Lexical, Grain::Phrase, Operation::Addition,
             1, 0.70, "penyela di tengah kalimat (parenthesis)");
    }

    // ── Conceptual diminution ────────────────────────────────────────
    let turun = d.contains("reduce") || d.contains("diminish") || d.contains("lessen")
        || d.contains("lower than") || d.contains("beneath the");
    if turun && d.contains("conclud") {
        push(Anchor::Final, ElementClass::Conceptual, Grain::Discourse, Operation::Deletion,
             1, 0.75, "penutup yang meredam gaya sebelumnya (abating/anesis)");
    } else if turun && (d.contains("expected") || d.contains("anticipat")) {
        push(Anchor::WholeUnit, ElementClass::Conceptual, Grain::Discourse, Operation::Deletion,
             1, 0.70, "di bawah skala ekspektasi konteks (abbaser)");
    }

    // ── Truncation / clipping / apocope ────────────────────────────────
    if d.contains("truncat") || d.contains("clipping") || d.contains("apocope")
        || d.contains("shorten") && (d.contains("remov") || d.contains("cut") || d.contains("delet"))
        || d.contains("final segment") || d.contains("terminal segment")
        || d.contains("removing the end") || d.contains("cut off the end") {
        push(Anchor::Final, ElementClass::Lexical, Grain::Word, Operation::Deletion,
             1, 0.80, "pemotongan segmen akhir kata (apocope/clipping)");
    }

    candidates.into_iter().max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
}

/// Serialize a compiled pattern into the SARVA database JSON convention
/// (Indonesian keys/values: jangkar/kelas/satuan/operasi/minim_ulangan).
/// The crate's own serialization stays English; this bridge keeps legacy
/// consumers working.
pub fn ke_json_konvensi_sarva(p: &FigurePattern) -> String {
    let jangkar = match p.anchor {
        Anchor::Initial => "Awal",
        Anchor::Final => "Akhir",
        Anchor::Insertion => "Sisipan",
        Anchor::WholeUnit => "UnitUtuh",
        Anchor::CrossUnit => "AntarUnit",
    };
    let kelas = match p.class {
        ElementClass::Lexical => "Leksikal",
        ElementClass::Root => "Akar",
        ElementClass::Grammatical => "Gramatikal",
        ElementClass::Conceptual => "Konseptual",
    };
    let satuan = match p.grain {
        Some(Grain::Grapheme) => "grafem",
        Some(Grain::Word) => "kata",
        Some(Grain::Phrase) => "frasa",
        Some(Grain::Unit) => "unit",
        Some(Grain::Discourse) => "wacana",
        None => "unit",
    };
    let operasi = match p.operation {
        Some(Operation::Addition) => "adjectio",
        Some(Operation::Deletion) => "detractio",
        Some(Operation::Substitution) => "immutatio",
        Some(Operation::Permutation) => "transmutatio",
        Some(Operation::Repetition) => "repetitio",
        None => "repetitio",
    };
    format!(
        "{{\"jangkar\":\"{jangkar}\",\"kelas\":\"{kelas}\",\"satuan\":\"{satuan}\",\"operasi\":\"{operasi}\",\"minim_ulangan\":{},\"template\":[],\"catatan\":\"{}\"}}",
        p.min_repeats,
        p.note.as_deref().unwrap_or("")
    )
}

/// Text token with its equality label + byte offset in the source unit.
/// For Lexical, `label` = lowercased word; for Grammatical/Conceptual,
/// `label` = POS tag / concept id from an external extractor (LLM/annotator).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledToken {
    pub label: String,
    #[serde(alias = "teks")]
    pub text: String,
    pub offset_start: usize,
    pub offset_end: usize,
}

/// One concrete evidence location inside the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLocation {
    pub chunk_id: String,
    pub span_start: usize,
    pub span_end: usize,
    #[serde(alias = "cuplikan")]
    pub excerpt: String,
}

/// A geometric finding with full provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricFinding {
    #[serde(alias = "nama_figur")]
    pub figure_name: String,
    #[serde(alias = "kelas")]
    pub class: ElementClass,
    #[serde(alias = "jangkar")]
    pub anchor: Anchor,
    /// Evidence per unit (chunk_id + span + excerpt). Cross-unit figures
    /// (anadiplosis/gradatio) carry evidence in each involved unit.
    #[serde(alias = "bukti")]
    pub evidence: Vec<EvidenceLocation>,
}

/// Minimal unit consumed by the matcher — just chunk_id + text, so this module
/// does not depend on any segmentation type.
pub struct TextUnit<'a> {
    pub chunk_id: &'a str,
    pub text: &'a str,
}

/// The geometry matching engine. Deterministic (no LLM) for the Lexical
/// class; other classes via `match_template` over labeled token sequences.
pub struct GeometryMatcher;

impl GeometryMatcher {
    /// Full detection over text units: anaphora, epistrophe, symploce,
    /// anadiplosis, gradatio, antimetabole (lexical phrase inversion).
    pub fn detect(units: &[TextUnit]) -> Vec<GeometricFinding> {
        let mut results = Vec::new();

        if let Some(ev) = Self::position_repetition(units, Position::Initial, 2) {
            results.push(GeometricFinding {
                figure_name: "anaphora".into(),
                class: ElementClass::Lexical,
                anchor: Anchor::Initial,
                evidence: ev,
            });
        }
        if let Some(ev) = Self::position_repetition(units, Position::Final, 2) {
            results.push(GeometricFinding {
                figure_name: "epistrophe".into(),
                class: ElementClass::Lexical,
                anchor: Anchor::Final,
                evidence: ev,
            });
        }
        if let Some(ev) = Self::both_ends_repetition(units, 2) {
            results.push(GeometricFinding {
                figure_name: "symploce".into(),
                class: ElementClass::Lexical,
                anchor: Anchor::Initial,
                evidence: ev,
            });
        }

        let (anadiplosis, gradatio) = Self::anadiplosis_chain(units);
        if let Some(ev) = anadiplosis {
            results.push(GeometricFinding {
                figure_name: "anadiplosis".into(),
                class: ElementClass::Lexical,
                anchor: Anchor::CrossUnit,
                evidence: ev,
            });
        }
        if let Some(ev) = gradatio {
            results.push(GeometricFinding {
                figure_name: "gradatio (climax)".into(),
                class: ElementClass::Lexical,
                anchor: Anchor::CrossUnit,
                evidence: ev,
            });
        }

        for unit in units {
            let tokens = Self::extract_lexical_tokens(unit.text);
            if let Some((s, e)) = find_phrase_inversion(&tokens) {
                results.push(GeometricFinding {
                    figure_name: "antimetabole (phrase inversion)".into(),
                    class: ElementClass::Lexical,
                    anchor: Anchor::WholeUnit,
                    evidence: vec![EvidenceLocation {
                        chunk_id: unit.chunk_id.to_string(),
                        span_start: tokens[s].offset_start,
                        span_end: tokens[e - 1].offset_end,
                        excerpt: join_tokens(&tokens[s..e]),
                    }],
                });
            }
        }

        results
    }

    /// Built-in label extractor: lexical tokens (lowercased) + byte offsets.
    pub fn extract_lexical_tokens(text: &str) -> Vec<LabeledToken> {
        let mut results = Vec::new();
        let mut start: Option<usize> = None;
        for (idx, c) in text.char_indices() {
            if c.is_whitespace() {
                if let Some(s) = start.take() {
                    push_token(&mut results, text, s, idx);
                }
            } else if start.is_none() {
                start = Some(idx);
            }
        }
        if let Some(s) = start {
            push_token(&mut results, text, s, text.len());
        }
        results
    }

    /// Generic template matcher over labeled token sequences. Predicate:
    /// same slot id → same label; different ids → different labels;
    /// `*` = anything. Equality classes are NOT used here — labels already
    /// encode the class.
    pub fn match_template(template: &[Slot], tokens: &[LabeledToken]) -> Option<(usize, usize)> {
        if template.is_empty() || tokens.len() < template.len() {
            return None;
        }
        for start in 0..=(tokens.len() - template.len()) {
            let window = &tokens[start..start + template.len()];
            let mut ok = true;
            for i in 0..template.len() {
                let id_i = template[i].id;
                if id_i == '*' {
                    continue;
                }
                for j in (i + 1)..template.len() {
                    let id_j = template[j].id;
                    if id_j == '*' {
                        continue;
                    }
                    let same_id = id_i == id_j;
                    let same_label = window[i].label == window[j].label;
                    if same_id != same_label {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                return Some((start, start + template.len()));
            }
        }
        None
    }

    // ── Positional-repetition lexical matchers ───────────────────────────

    fn position_repetition(units: &[TextUnit], position: Position, min: usize) -> Option<Vec<EvidenceLocation>> {
        let mut groups: std::collections::HashMap<String, Vec<EvidenceLocation>> = std::collections::HashMap::new();
        for u in units {
            let tokens = Self::extract_lexical_tokens(u.text);
            let token = match position {
                Position::Initial => tokens.first(),
                Position::Final => tokens.last(),
            };
            if let Some(t) = token {
                groups.entry(t.label.clone())
                    .or_default()
                    .push(EvidenceLocation {
                        chunk_id: u.chunk_id.to_string(),
                        span_start: t.offset_start,
                        span_end: t.offset_end,
                        excerpt: t.text.clone(),
                    });
            }
        }
        groups.into_iter()
            .filter(|(_, ev)| ev.len() >= min)
            .max_by_key(|(_, ev)| ev.len())
            .map(|(_, ev)| ev)
    }

    fn both_ends_repetition(units: &[TextUnit], min: usize) -> Option<Vec<EvidenceLocation>> {
        let mut groups: std::collections::HashMap<(String, String), Vec<EvidenceLocation>> = std::collections::HashMap::new();
        for u in units {
            let tokens = Self::extract_lexical_tokens(u.text);
            if let (Some(first), Some(last)) = (tokens.first(), tokens.last()) {
                if first.label == last.label {
                    continue; // not true symploce if it is a single word
                }
                groups.entry((first.label.clone(), last.label.clone()))
                    .or_default()
                    .push(EvidenceLocation {
                        chunk_id: u.chunk_id.to_string(),
                        span_start: first.offset_start,
                        span_end: last.offset_end,
                        excerpt: format!("{} … {}", first.text, last.text),
                    });
            }
        }
        groups.into_iter()
            .filter(|(_, ev)| ev.len() >= min)
            .max_by_key(|(_, ev)| ev.len())
            .map(|(_, ev)| ev)
    }

    /// Anadiplosis: end of unit-i == start of unit-(i+1). Gradatio: a chain of
    /// >= 2 consecutive links. Returns (single anadiplosis, gradatio).
    fn anadiplosis_chain(units: &[TextUnit]) -> (Option<Vec<EvidenceLocation>>, Option<Vec<EvidenceLocation>>) {
        // consecutive links: (unit_i, linking word, left evidence, right evidence)
        let mut links: Vec<(usize, String, EvidenceLocation, EvidenceLocation)> = Vec::new();
        for i in 0..units.len().saturating_sub(1) {
            let t_i = Self::extract_lexical_tokens(units[i].text);
            let t_j = Self::extract_lexical_tokens(units[i + 1].text);
            let (Some(end_i), Some(start_j)) = (t_i.last(), t_j.first()) else {
                continue;
            };
            if end_i.label == start_j.label {
                links.push((
                    i,
                    end_i.label.clone(),
                    EvidenceLocation {
                        chunk_id: units[i].chunk_id.to_string(),
                        span_start: end_i.offset_start,
                        span_end: end_i.offset_end,
                        excerpt: end_i.text.clone(),
                    },
                    EvidenceLocation {
                        chunk_id: units[i + 1].chunk_id.to_string(),
                        span_start: start_j.offset_start,
                        span_end: start_j.offset_end,
                        excerpt: start_j.text.clone(),
                    },
                ));
            }
        }

        if links.is_empty() {
            return (None, None);
        }

        // consecutive runs (sequential unit indices) → gradatio; otherwise single anadiplosis.
        let mut runs: Vec<Vec<(usize, String, EvidenceLocation, EvidenceLocation)>> = Vec::new();
        for l in links {
            if let Some(run) = runs.last_mut() {
                if let Some(prev) = run.last() {
                    if prev.0 + 1 == l.0 {
                        run.push(l);
                        continue;
                    }
                }
            }
            runs.push(vec![l]);
        }

        let mut gradatio_evidence: Vec<EvidenceLocation> = Vec::new();
        let mut anadiplosis_evidence: Vec<EvidenceLocation> = Vec::new();
        for run in &runs {
            if run.len() >= 2 {
                for (_, label, left, _right) in run {
                    let mut b = left.clone();
                    b.excerpt = format!("{} →", label);
                    gradatio_evidence.push(b);
                }
                if let Some((_, _, _left, right)) = run.last() {
                    gradatio_evidence.push(right.clone());
                }
            } else if let Some((_, _, left, right)) = run.first() {
                anadiplosis_evidence.push(left.clone());
                anadiplosis_evidence.push(right.clone());
            }
        }

        (
            if anadiplosis_evidence.is_empty() { None } else { Some(anadiplosis_evidence) },
            if gradatio_evidence.is_empty() { None } else { Some(gradatio_evidence) },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Initial,
    Final,
}

fn push_token(results: &mut Vec<LabeledToken>, text: &str, start: usize, end: usize) {
    if end <= start {
        return;
    }
    let token = &text[start..end];
    // Label = lowercase, alphanumeric characters only (strip punctuation),
    // so "Light" == "light." as the same Lexical element.
    // `text` (excerpt) stays original for display.
    let label: String = token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    if label.is_empty() {
        return; // punctuation-only is not a token
    }
    results.push(LabeledToken {
        label,
        text: token.to_string(),
        offset_start: start,
        offset_end: end,
    });
}

fn join_tokens(tokens: &[LabeledToken]) -> String {
    tokens.iter().map(|t| t.text.clone()).collect::<Vec<_>>().join(" ")
}

/// Phrase inversion (lexical antimetabole): segment `P`, then (with a gap <= 2
/// tokens, usually a conjunction) segment `reverse(P)` of equal length >= 2.
/// Example: "fair is foul, and foul is fair" → P=[fair,is,foul], gap=[and],
/// rev(P)=[foul,is,fair].
fn find_phrase_inversion(tokens: &[LabeledToken]) -> Option<(usize, usize)> {
    let n = tokens.len();
    if n < 4 {
        return None;
    }
    let max_gap = 2;
    for len in (2..=n / 2).rev() {
        for start in 0..n {
            let end = start + len;
            if end + len > n + max_gap {
                continue;
            }
            for gap in 0..=max_gap {
                let seg2_start = end + gap;
                let seg2_end = seg2_start + len;
                if seg2_end > n {
                    continue;
                }
                let seg1 = &tokens[start..end];
                let seg2 = &tokens[seg2_start..seg2_end];
                if seg1.iter().map(|x| &x.label).eq(seg2.iter().rev().map(|x| &x.label)) {
                    return Some((start, seg2_end));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit<'a>(id: &'a str, text: &'a str) -> TextUnit<'a> {
        TextUnit { chunk_id: id, text }
    }

    fn token(label: &str, text: &str) -> LabeledToken {
        LabeledToken {
            label: label.to_string(),
            text: text.to_string(),
            offset_start: 0,
            offset_end: text.len(),
        }
    }

    #[test]
    fn grammatical_abba_template_matches() {
        // "It is boring to eat; to sleep is fulfilling"
        // present-participle ~ infinitive | infinitive ~ present-participle
        let template = vec![
            Slot::new('A', ElementClass::Grammatical),
            Slot::new('B', ElementClass::Grammatical),
            Slot::new('B', ElementClass::Grammatical),
            Slot::new('A', ElementClass::Grammatical),
        ];
        let tokens = vec![
            token("PART", "boring"),
            token("INF", "to eat"),
            token("INF", "to sleep"),
            token("PART", "fulfilling"),
        ];
        let (s, e) = GeometryMatcher::match_template(&template, &tokens).unwrap();
        assert_eq!((s, e), (0, 4));
    }

    #[test]
    fn conceptual_abba_template_matches() {
        // Shakespeare: affection(dotes, strongly loves) + doubting(doubts, suspects)
        let template = vec![
            Slot::new('A', ElementClass::Conceptual),
            Slot::new('B', ElementClass::Conceptual),
            Slot::new('B', ElementClass::Conceptual),
            Slot::new('A', ElementClass::Conceptual),
        ];
        let tokens = vec![
            token("AFFECTION", "dotes"),
            token("DOUBTING", "doubts"),
            token("DOUBTING", "suspects"),
            token("AFFECTION", "strongly loves"),
        ];
        let (s, e) = GeometryMatcher::match_template(&template, &tokens).unwrap();
        assert_eq!((s, e), (0, 4));
    }

    #[test]
    fn abba_rejects_wrong_order() {
        let template = vec![
            Slot::new('A', ElementClass::Grammatical),
            Slot::new('B', ElementClass::Grammatical),
            Slot::new('B', ElementClass::Grammatical),
            Slot::new('A', ElementClass::Grammatical),
        ];
        // A B A B is not A B B A
        let tokens = vec![
            token("PART", "boring"),
            token("INF", "to eat"),
            token("PART", "fulfilling"),
            token("INF", "to sleep"),
        ];
        assert_eq!(GeometryMatcher::match_template(&template, &tokens), None);
    }

    #[test]
    fn anaphora_detected_on_matching_openers() {
        let units = vec![
            unit("c0", "We came."),
            unit("c1", "We saw."),
            unit("c2", "We conquered."),
        ];
        let results = GeometryMatcher::detect(&units);
        let ana = results.iter().find(|f| f.figure_name == "anaphora").expect("anaphora must be detected");
        assert_eq!(ana.evidence.len(), 3);
        assert_eq!(ana.evidence[0].chunk_id, "c0");
        assert_eq!(ana.evidence[0].excerpt, "We");
        assert_eq!(ana.anchor, Anchor::Initial);
    }

    #[test]
    fn epistrophe_detected_on_matching_closers() {
        let units = vec![
            unit("c0", "I work hard."),
            unit("c1", "You also work hard."),
        ];
        let results = GeometryMatcher::detect(&units);
        let epi = results.iter().find(|f| f.figure_name == "epistrophe").expect("epistrophe must be detected");
        assert_eq!(epi.evidence.len(), 2);
        assert_eq!(epi.evidence[0].excerpt, "hard.");
        assert_eq!(epi.anchor, Anchor::Final);
    }

    #[test]
    fn anadiplosis_end_equals_next_start() {
        let units = vec![
            unit("c0", "There is light."),
            unit("c1", "Light illuminates everything."),
        ];
        let results = GeometryMatcher::detect(&units);
        assert!(results.iter().any(|f| f.figure_name == "anadiplosis"));
    }

    #[test]
    fn gradatio_needs_two_links_minimum() {
        let units = vec![
            unit("c0", "The first is hope."),
            unit("c1", "Hope brings conviction."),
            unit("c2", "Conviction brings action."),
        ];
        let results = GeometryMatcher::detect(&units);
        assert!(results.iter().any(|f| f.figure_name == "gradatio (climax)"));
        let g = results.iter().find(|f| f.figure_name == "gradatio (climax)").unwrap();
        assert!(g.evidence.len() >= 3);
    }

    #[test]
    fn antimetabole_lexical_phrase_inversion() {
        let units = vec![unit("c0", "Fair is foul, and foul is fair.")];
        let results = GeometryMatcher::detect(&units);
        let anti = results.iter().find(|f| f.figure_name.starts_with("antimetabole")).expect("antimetabole must be detected");
        assert_eq!(anti.evidence[0].chunk_id, "c0");
        assert!(anti.evidence[0].span_end > anti.evidence[0].span_start);
    }

    #[test]
    fn catalog_filters_by_anchor() {
        let final_ = FigurePattern::with_anchor(Anchor::Final);
        assert!(final_.iter().any(|p| p.name == "epistrophe"));
        let cross = FigurePattern::with_anchor(Anchor::CrossUnit);
        assert!(cross.iter().any(|p| p.name == "gradatio (climax)"));
        assert!(cross.iter().any(|p| p.name == "anadiplosis"));
    }

    #[test]
    fn lexical_tokens_keep_offsets() {
        let tokens = GeometryMatcher::extract_lexical_tokens("I like you");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].label, "i");
        assert_eq!(tokens[1].offset_start, 2);
        assert_eq!(tokens[2].offset_start, 7);
        assert_eq!(tokens[2].offset_end, 10);
    }

    #[test]
    fn sarva_indonesian_json_deserializes() {
        // The SARVA DB convention must load transparently via serde aliases.
        let json = r#"{
            "nama": "anaphora",
            "jangkar": "Awal",
            "kelas": "Leksikal",
            "minim_ulangan": 2,
            "satuan": "kata",
            "operasi": "repetitio",
            "template": []
        }"#;
        let p: FigurePattern = serde_json::from_str(json).unwrap();
        assert_eq!(p.name, "anaphora");
        assert_eq!(p.anchor, Anchor::Initial);
        assert_eq!(p.class, ElementClass::Lexical);
        assert_eq!(p.min_repeats, 2);
        assert_eq!(p.operation, Some(Operation::Repetition));
    }

    #[test]
    fn heuristic_compiles_anaphora_definition() {
        let d = "Repetition of the same word or group of words at the \
                 beginning of successive clauses.";
        let draft = compile_definition(d).expect("anaphora should compile");
        assert!(draft.confidence >= 0.85);
        assert_eq!(draft.pattern.anchor, Anchor::Initial);
        assert_eq!(draft.pattern.operation, Some(Operation::Repetition));
        assert_eq!(draft.pattern.min_repeats, 2);
    }

    #[test]
    fn heuristic_compiles_epistrophe_definition() {
        let d = "Repetition of the same word or group of words at the ends \
                 of successive clauses.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::Final);
        assert_eq!(draft.pattern.operation, Some(Operation::Repetition));
    }

    #[test]
    fn heuristic_compiles_tmesis_definition() {
        let d = "The insertion of a word in between a word, cutting the \
                 original word into two parts.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::Insertion);
        assert_eq!(draft.pattern.grain, Some(Grain::Grapheme));
        assert_eq!(draft.pattern.operation, Some(Operation::Addition));
    }

    #[test]
    fn heuristic_compiles_concluding_diminution() {
        let d = "A concluding representation that reduces the rhetorical \
                 force of what precedes it.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::Final);
        assert_eq!(draft.pattern.class, ElementClass::Conceptual);
        assert_eq!(draft.pattern.operation, Some(Operation::Deletion));
    }

    #[test]
    fn heuristic_compiles_below_expected_scale() {
        let d = "A representation that is semantically or rhetorically lower \
                 than the expected scale.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::WholeUnit);
        assert_eq!(draft.pattern.class, ElementClass::Conceptual);
        assert_eq!(draft.pattern.operation, Some(Operation::Deletion));
    }

    #[test]
    fn heuristic_compiles_phrase_inversion() {
        let d = "Repetition of a phrase with the order of words reversed.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::WholeUnit);
        assert_eq!(draft.pattern.operation, Some(Operation::Permutation));
        assert_eq!(draft.pattern.class, ElementClass::Lexical);
    }

    #[test]
    fn heuristic_compiles_truncation_clipping() {
        let d = "Truncate(word, terminal_segment) -> a shortened word form \
                 produced by removing its final segment.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::Final);
        assert_eq!(draft.pattern.class, ElementClass::Lexical);
        assert_eq!(draft.pattern.operation, Some(Operation::Deletion));
        assert!(draft.confidence >= 0.75);
    }

    #[test]
    fn unknown_definition_falls_through_without_guessing() {
        assert!(compile_definition("An obscure term for a mild oath.").is_none());
    }

    #[test]
    fn sarva_bridge_emits_legacy_json() {
        let p = FigurePattern {
            name: String::new(),
            template: vec![],
            anchor: Anchor::CrossUnit,
            class: ElementClass::Lexical,
            min_repeats: 1,
            grain: Some(Grain::Word),
            operation: Some(Operation::Repetition),
            note: None,
        };
        let j = ke_json_konvensi_sarva(&p);
        assert!(j.contains("\"jangkar\":\"AntarUnit\""));
        assert!(j.contains("\"kelas\":\"Leksikal\""));
        assert!(j.contains("\"operasi\":\"repetitio\""));
        // round-trips through the alias deserializer
        let back: FigurePattern = serde_json::from_str(&j).unwrap();
        assert_eq!(back.anchor, Anchor::CrossUnit);
    }
}
