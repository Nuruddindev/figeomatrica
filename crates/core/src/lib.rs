// figeometrica-core
// ─────────────────────────────────────────────────────────────────────────────
// GEOMETRY OF FIGURES — form as a matchable template.
//
// Thesis: every rhetorical figure, if well defined, states an operational
// geometry of text. Canonical form:
//
//     figure = OPERATION x ANCHOR x GRAIN x REPETITION [+ ΔCOORDINATES]
//
//   - operation:  adjectio | detractio | immutatio | transmutatio | repetitio
//                 (the four classical operae + repetition)
//   - anchor:     initial | final | insertion | whole-unit | cross-unit
//   - grain:      grapheme | word | phrase | unit | discourse
//   - Δcoords:    signed shifts in rhetorical space — force↓, magnitude↑,
//                 explicitness↓, social-acceptability↑, ... (open vocabulary)
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
use serde_json;

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


/// Operation performed on elements (canonical-form axis; the four classical
/// operae plus repetition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(alias = "adjectio", alias = "addition")]
    Addition,
    #[serde(alias = "detractio", alias = "deletion")]
    Deletion,
    #[serde(alias = "immutatio", alias = "substitution")]
    Substitution,
    #[serde(alias = "transmutatio", alias = "permutation")]
    Permutation,
    #[serde(alias = "repetitio", alias = "repetition")]
    Repetition,
    /// Ordinal constraint: element i corresponds to position i of an
    /// external reference sequence (e.g. successive alphabet letters →
    /// abecedarian). Added when the dataset demanded it — canonical form
    /// grows with evidence, not speculation.
    #[serde(alias = "ordering")]
    Ordering,
}

/// Direction of a shift along a rhetorical-space coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(alias = "naik", alias = "up")]
    Up,
    #[serde(alias = "turun", alias = "down")]
    Down,
    /// Order-type shifts: the arrangement changes without a signed magnitude.
    #[serde(alias = "netral", alias = "neutral")]
    Neutral,
}

/// A signed shift along one geometric coordinate — the axes of rhetorical
/// space. Known vocabulary (open, versioned): magnitude, intensity, status,
/// importance, force, explicitness, social acceptability, order, ...
///
/// Figures BIND coordinates; only genuinely new phenomena add axes. This is
/// the anticipatory mapping layer: a new definition is classified by which
/// coordinate it moves and how, before anything new is invented.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transform {
    pub axis: String,
    pub direction: Direction,
}

impl Transform {
    pub fn new(axis: &str, direction: Direction) -> Self {
        Transform { axis: axis.to_string(), direction }
    }
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
    #[serde(default, alias = "satuan", alias = "grain", skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, alias = "operasi", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// Signed coordinate shifts in rhetorical space (empty = purely
    /// structural pattern).
    #[serde(default, alias = "transformasi", skip_serializing_if = "Vec::is_empty")]
    pub transforms: Vec<Transform>,
    /// Occurrence index in the series (None = single occurrence, collapses
    /// to the anchor).
    #[serde(default, alias = "locus", skip_serializing_if = "Option::is_none")]
    pub locus_id: Option<String>,
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
                unit_id: Some("word".into()),
                operation: Some(Operation::Repetition),
                locus_id: None,
                transforms: vec![],
                note: None,
            },
            FigurePattern {
                name: "epistrophe".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('*', Lexical), Slot::new('A', Lexical)],
                anchor: Final,
                class: Lexical,
                min_repeats: 2,
                unit_id: Some("word".into()),
                operation: Some(Operation::Repetition),
                locus_id: None,
                transforms: vec![],
                note: None,
            },
            FigurePattern {
                name: "symploce".into(),
                template: vec![],
                anchor: Initial,
                class: Lexical,
                min_repeats: 2,
                unit_id: Some("word".into()),
                operation: Some(Operation::Repetition),
                locus_id: None,
                transforms: vec![],
                note: Some("repetition at both ends of each unit; composite pattern".into()),
            },
            FigurePattern {
                name: "anadiplosis".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('A', Lexical)],
                anchor: CrossUnit,
                class: Lexical,
                min_repeats: 1,
                unit_id: Some("word".into()),
                operation: Some(Operation::Repetition),
                locus_id: None,
                transforms: vec![],
                note: None,
            },
            FigurePattern {
                name: "gradatio (climax)".into(),
                template: vec![],
                anchor: CrossUnit,
                class: Lexical,
                min_repeats: 2,
                unit_id: Some("word".into()),
                operation: Some(Operation::Repetition),
                locus_id: None,
                transforms: vec![],
                note: Some("chained anadiplosis; >= 2 consecutive links".into()),
            },
            FigurePattern {
                name: "antimetabole".into(),
                template: vec![Slot::new('A', Lexical), Slot::new('B', Lexical), Slot::new('B', Lexical), Slot::new('A', Lexical)],
                anchor: WholeUnit,
                class: Lexical,
                min_repeats: 1,
                unit_id: Some("phrase".into()),
                operation: Some(Operation::Permutation),
                locus_id: None,
                transforms: vec![],
                note: None,
            },
            FigurePattern {
                name: "chiasmus".into(),
                template: vec![Slot::new('A', Conceptual), Slot::new('B', Conceptual), Slot::new('B', Conceptual), Slot::new('A', Conceptual)],
                anchor: WholeUnit,
                class: Conceptual,
                min_repeats: 1,
                unit_id: Some("phrase".into()),
                operation: Some(Operation::Permutation),
                locus_id: None,
                transforms: vec![],
                note: None,
            },
            FigurePattern {
                name: "tmesis".into(),
                template: vec![],
                anchor: Insertion,
                class: Lexical,
                min_repeats: 1,
                unit_id: Some("grapheme".into()),
                operation: Some(Operation::Addition),
                locus_id: None,
                transforms: vec![],
                note: Some("a word cut open, another inserted inside it".into()),
            },
            FigurePattern {
                name: "parenthesis".into(),
                template: vec![],
                anchor: Insertion,
                class: Lexical,
                min_repeats: 1,
                unit_id: Some("phrase".into()),
                operation: Some(Operation::Addition),
                locus_id: None,
                transforms: vec![],
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
    /// Which known family received the binding — "oh, ini di sini". Only
    /// when NO family fits is a definition genuinely new and may warrant
    /// extending the vocabulary (as `Ordering` did for abecedarian).
    pub family: &'static str,
}

fn draft(anchor: Anchor, class: ElementClass, grain: Option<&str>, op: Operation,
         min_repeats: usize, confidence: f32, family: &'static str,
         locus_id: Option<&str>, transforms: &[(&str, Direction)],
         catatan: &str) -> DraftGeometri {
    DraftGeometri {
        pattern: FigurePattern {
            name: String::new(),
            template: vec![],
            anchor,
            class,
            min_repeats,
            unit_id: grain.map(|s| s.to_string()),
            operation: Some(op),
            locus_id: locus_id.map(|s| s.to_string()),
            transforms: transforms
                .iter()
                .map(|(axis, dir)| Transform::new(axis, *dir))
                .collect(),
            note: Some(format!("kompilasi heuristik: {catatan}")),
        },
        confidence,
        family,
    }
}

/// Deterministic prose-to-canonical compiler (heuristic stage).
///
/// Anticipatory mapping over RHETORICAL SPACE:
///
/// ```text
/// GENERAL STRATEGIES  = addition | subtraction | substitution |
///                       transposition | repetition (+ ordering)
/// GEOMETRIC COORDS    = magnitude | intensity | status | importance |
///                       force | explicitness | social acceptability |
///                       order | ...   (open vocabulary)
/// FIGURE              = strategy x Δcoords x anchor x unit x locus x
///                       repetition x constraints
/// ```
///
/// A new definition is mapped INTO an existing family whenever possible;
/// only when no family fits is it genuinely new — then a new binding or a
/// new axis joins the vocabulary. Unmatched definitions return `None`:
/// they wait for an LLM pass or a human, never get guessed.
pub fn compile_definition(definition: &str) -> Option<DraftGeometri> {
    let d = definition.to_lowercase();
    let mut c: Vec<DraftGeometri> = Vec::new();

    // ── REPETITION (positional recurrence) ───────────────────────────
    let rep = d.contains("repetit") || d.contains("repeat");
    let awal = d.contains("beginning") || d.contains("the start");
    let akhir = d.contains("end of") || d.contains("the end")
        || d.contains("conclusion of successive");
    if rep && awal && akhir {
        c.push(draft(Anchor::Initial, ElementClass::Lexical, Some("word"), Operation::Repetition, 2, 0.80, "repetition", Some("every"), &[],
            "pengulangan di awal DAN akhir unit (symploce)"));
    } else if rep && (d.contains("beginning of successive") || d.contains("begins successive")
        || d.contains("at the beginning")) {
        c.push(draft(Anchor::Initial, ElementClass::Lexical, Some("word"), Operation::Repetition, 2, 0.90, "repetition", Some("every"), &[],
            "pengulangan kata pembuka antar-unit (anaphora)"));
    } else if rep && (d.contains("end of successive") || d.contains("ends of successive")
        || d.contains("at the end")) {
        c.push(draft(Anchor::Final, ElementClass::Lexical, Some("word"), Operation::Repetition, 2, 0.90, "repetition", Some("every"), &[],
            "pengulangan kata penutup antar-unit (epistrophe)"));
    }
    if d.contains("repetit")
        && (d.contains("intervening") || d.contains("non-contiguous")
            || d.contains("across multiple")) {
        c.push(draft(Anchor::CrossUnit, ElementClass::Conceptual, Some("unit"), Operation::Repetition, 2, 0.75, "repetition",
            Some("distributed"), &[],
            "kembalian berulang ke anchor argumen sama, materi selingan diizinkan (commoratio)"));
    }
    if d.contains("reiterate")
        || d.contains("dwelling on")
        || (d.contains("persisten") && d.contains("repetit")) {
        c.push(draft(Anchor::CrossUnit, ElementClass::Conceptual, Some("unit"), Operation::Repetition, 2, 0.70, "repetition",
            Some("clustered"), &[],
            "plea hampir identik diulang persisten secara berumpun (epimone)"));
    }
    if d.contains("last word") && (d.contains("first word") || d.contains("next")) {
        c.push(draft(Anchor::CrossUnit, ElementClass::Lexical, Some("word"), Operation::Repetition, 1, 0.85, "repetition", None, &[],
            "akhir unit menjadi awal unit berikut (anadiplosis)"));
    }
    if (d.contains("chain") || d.contains("series of clauses")) && rep {
        c.push(draft(Anchor::CrossUnit, ElementClass::Lexical, Some("word"), Operation::Repetition, 2, 0.70, "repetition", None, &[],
            "rantai pengulangan berturutan (gradatio/climax)"));
    }
    if rep && (d.contains("immediate repetition") || d.contains("repeated immediatel")) {
        c.push(draft(Anchor::WholeUnit, ElementClass::Lexical, Some("word"), Operation::Repetition, 2, 0.70, "repetition", None, &[],
            "pengulangan langsung dalam satu unit (epizeuxis)"));
    }

    // ── TRANSPOSITION (order rearrangement) ──────────────────────────
    if (d.contains("invers") || d.contains("reverse") || d.contains("reversal"))
        && (d.contains("order of word") || d.contains("order of phrase") || d.contains("phras")) {
        let kelas = if d.contains("meaning") || d.contains("concept") {
            ElementClass::Conceptual
        } else {
            ElementClass::Lexical
        };
        c.push(draft(Anchor::WholeUnit, kelas, Some("phrase"), Operation::Permutation, 1, 0.80, "transposition", None,
            &[("order", Direction::Neutral)],
            "inversi/permutasi frasa (antimetabole/chiasmus)"));
    }

    // ── ADDITION (interpolation) ─────────────────────────────────────
    if d.contains("insert") && (d.contains("word") && (d.contains("within a word")
        || d.contains("into a word") || d.contains("middle of a word") || d.contains("cut"))) {
        c.push(draft(Anchor::Insertion, ElementClass::Lexical, Some("grapheme"), Operation::Addition, 1, 0.75, "addition", None, &[],
            "sisipan di dalam kata (tmesis)"));
    } else if d.contains("interpolat") || d.contains("parenthetic")
        || (d.contains("insert") && (d.contains("sentence") || d.contains("clause"))) {
        c.push(draft(Anchor::Insertion, ElementClass::Lexical, Some("phrase"), Operation::Addition, 1, 0.70, "addition", None, &[],
            "penyela di tengah kalimat (parenthesis)"));
    }
    if d.contains("exaggerat") || d.contains("hyperbole") || d.contains("overstat") {
        c.push(draft(Anchor::WholeUnit, ElementClass::Conceptual, Some("phrase"), Operation::Addition, 1, 0.65, "amplification", None,
            &[("magnitude", Direction::Up)],
            "melampaui baseline skala (hyperbole)"));
    }

    // ── SUBTRACTION / SUBSTITUTION (understatement: transformasi turun) ──
    let turun = d.contains("reduce") || d.contains("diminish") || d.contains("lessen")
        || d.contains("lower than") || d.contains("beneath the");
    if turun && d.contains("conclud") {
        c.push(draft(Anchor::Final, ElementClass::Conceptual, Some("discourse"), Operation::Deletion, 1, 0.75, "understatement", Some("terminal"),
            &[("force", Direction::Down)],
            "penutup yang meredam gaya sebelumnya (abating/anesis)"));
    } else if turun && (d.contains("expected") || d.contains("anticipat")) {
        c.push(draft(Anchor::WholeUnit, ElementClass::Conceptual, Some("discourse"), Operation::Deletion, 1, 0.70, "understatement", None,
            &[("status", Direction::Down)],
            "di bawah skala ekspektasi konteks (abbaser)"));
    }
    if d.contains("mockery") || d.contains("conciliator")
        || (d.contains("soften") && d.contains("harsh")) {
        c.push(draft(Anchor::CrossUnit, ElementClass::Conceptual, Some("unit"), Operation::Substitution, 1, 0.75, "understatement", Some("response"),
            &[("intensity", Direction::Down), ("social acceptability", Direction::Up)],
            "respons konsiliatoris meredam pertukaran kasar lewat canda (charientismus)"));
    }
    if d.contains("litotes") || ((d.contains("negat") || d.contains("deni"))
        && (d.contains("opposite") || d.contains("extreme"))) {
        c.push(draft(Anchor::WholeUnit, ElementClass::Conceptual, Some("word"), Operation::Substitution, 1, 0.70, "understatement", None,
            &[("explicitness", Direction::Down), ("intensity", Direction::Down)],
            "menegaskan lewat menyangkal lawan ekstrem (litotes)"));
    }
    if d.contains("belittle") || d.contains("meiosis") || d.contains("understat") {
        c.push(draft(Anchor::WholeUnit, ElementClass::Conceptual, Some("phrase"), Operation::Deletion, 1, 0.65, "understatement", None,
            &[("importance", Direction::Down)],
            "mengecilkan makna dibanding baseline (meiosis)"));
    }

    // ── SUBTRACTION (structural truncation / apocope) ────────────────
    if d.contains("synaloepha") || d.contains("synaloeph") {
        c.push(draft(Anchor::CrossUnit, ElementClass::Lexical, Some("grapheme"), Operation::Deletion, 1, 0.85, "truncation", Some("cross_unit"), &[],
            "penghilangan vokal di batas kata (synaloepha)"));
    }
        // ── SYNCOPE (medial deletion) ───────────────────────────────
    if d.contains("syncope") || (d.contains("mid") && (d.contains("cut") || d.contains("omit") || d.contains("remov"))) {
        c.push(draft(Anchor::Final, ElementClass::Lexical, Some("word"), Operation::Deletion, 1, 0.86, "truncation", Some("medial"), &[],
            "pemotongan segmen tengah kata (syncope)"));
    }

    // ── PROTHESIS / EPENTHESIS (addition at boundaries/within word) ──
    if d.contains("prothesis") || (d.contains("adjectio") && d.contains("initial") && d.contains("boundary") && d.contains("insert"))
        || d.contains("prothesis") {
        c.push(draft(Anchor::Initial, ElementClass::Lexical, Some("word"), Operation::Addition,
             1, 0.80, "addition", Some("every_slot"), &[], "penambahan di awal kata (prothesis)"));
    }
    if d.contains("epenthesis") || (d.contains("adjectio") && d.contains("interior") && d.contains("insert"))
        || d.contains("epenthesis") {
        c.push(draft(Anchor::CrossUnit, ElementClass::Lexical, Some("word"), Operation::Addition,
             1, 0.80, "addition", Some("medial"), &[],
             "penambahan di tengah kata (epenthesis)"));
    }

    // ── APHAERESIS (initial boundary deletion) ─────────────────────────
    if d.contains("aphaeresis") || d.contains("aphaeresis")
        || (d.contains("detractio") || d.contains("remov") || d.contains("omission") || d.contains("cut") || d.contains("omit"))
        && (d.contains("initial") || d.contains("beginning") || d.contains("left boundary")) {
        c.push(draft(Anchor::Initial, ElementClass::Lexical, Some("word"), Operation::Deletion, 1, 0.85, "truncation", None, &[],
            "penghilangan segmen awal kata (aphaeresis)"));
    }
    if d.contains("prothesis") || (d.contains("adjectio") && d.contains("initial") && d.contains("boundary") && d.contains("insert"))
        || d.contains("prothesis") {
        c.push(draft(Anchor::Initial, ElementClass::Lexical, Some("word"), Operation::Addition,
             1, 0.80, "addition", Some("initial"), &[],
             "penambahan di awal kata (prothesis)"));
    }

    if d.contains("apocope") || d.contains("apocope")
        || d.contains("omission of") && (d.contains("final") || d.contains("cutting off"))
        || d.contains("cutting off") && (d.contains("final") || d.contains("end"))
        || (d.contains("omission of") || d.contains("cutting off")) && (d.contains("final") || d.contains("end")) {
        c.push(draft(Anchor::Final, ElementClass::Lexical, Some("word"), Operation::Deletion, 1, 0.80, "truncation", Some("medial"), &[],
            "pemotongan segmen akhir kata (apocope)"));
    }
    if d.contains("truncat") || d.contains("clipping") || d.contains("apocope") || d.contains("aphaeresis")
        || d.contains("shorten") && (d.contains("remov") || d.contains("cut") || d.contains("delet"))
        || d.contains("final segment") || d.contains("terminal segment")
        || d.contains("removing the end") || d.contains("cut off the end") || d.contains("cut off") && d.contains("final")
        || d.contains("omission of") && (d.contains("final") || d.contains("initial") || d.contains("letter") || d.contains("syllable"))
        || d.contains("omission of final") || d.contains("omission of initial")
        || d.contains("cutting") && (d.contains("beginning") || d.contains("initial") || d.contains("end") || d.contains("middle") || d.contains("mid"))
        || d.contains("cut off") && (d.contains("beginning") || d.contains("initial") || d.contains("end") || d.contains("final"))
        || d.contains("cutting off") && (d.contains("beginning") || d.contains("initial") || d.contains("end") || d.contains("final"))
        || d.contains("syncope") || d.contains("mid") && (d.contains("cut") || d.contains("omit") || d.contains("remov")) {
        c.push(draft(Anchor::Final, ElementClass::Lexical, Some("word"), Operation::Deletion, 1, 0.80, "truncation", None, &[],
            "pemotongan segmen awal/akhir/tengah kata (apocope/aphaeresis/syncope/clipping)"));
    }

    // ── ORDER CORRESPONDENCE (abecedarian family) ────────────────────
    if d.contains("alphabet") || d.contains("successive letters")
        || d.contains("initial letters in order")
        || d.contains("letters follow") {
        c.push(draft(Anchor::CrossUnit, ElementClass::Lexical, Some("grapheme"), Operation::Ordering, 1, 0.80, "ordering", None,
            &[("order", Direction::Neutral)],
            "huruf awal tiap unit mengikuti urutan referensi eksternal (abecedarian)"));
    }

    c.into_iter().max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
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
    let satuan = p.unit_id.as_deref().unwrap_or("unit");
    let operasi = match p.operation {
        Some(Operation::Addition) => "adjectio",
        Some(Operation::Deletion) => "detractio",
        Some(Operation::Substitution) => "immutatio",
        Some(Operation::Permutation) => "transmutatio",
        Some(Operation::Repetition) => "repetitio",
        Some(Operation::Ordering) => "ordering",
        None => "repetitio",
    };
    let arah = |d: Direction| match d {
        Direction::Up => "naik",
        Direction::Down => "turun",
        Direction::Neutral => "netral",
    };
    let mut obj = serde_json::json!({
        "jangkar": jangkar,
        "kelas": kelas,
        "satuan": satuan,
        "operasi": operasi,
        "minim_ulangan": p.min_repeats,
        "template": serde_json::Value::Array(vec![]),
    });
    if !p.transforms.is_empty() {
        obj["transformasi"] = serde_json::Value::Array(p.transforms.iter().map(|t| serde_json::json!({"sumbu": t.axis, "arah": arah(t.direction)})).collect());
    }
    if let Some(l) = &p.locus_id {
        // Canonical latin id everywhere — matches the knowledge tables
        // (loci.id); no per-consumer translation.
        obj["locus"] = serde_json::json!(l);
    }
    if let Some(n) = p.note.as_deref() {
        obj["catatan"] = serde_json::json!(n);
    }
    serde_json::to_string(&obj).unwrap()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Initial,
    Final,
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
        };

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
        assert_eq!(draft.pattern.unit_id, Some("grapheme".into()));
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
    fn heuristic_compiles_abecedarian_ordering() {
        let d = "A series of units whose initial letters follow the order of \
                 the alphabet.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.pattern.anchor, Anchor::CrossUnit);
        assert_eq!(draft.pattern.operation, Some(Operation::Ordering));
        assert_eq!(draft.pattern.unit_id, Some("grapheme".into()));
    }

    #[test]
    fn heuristic_compiles_charientismus_understatement() {
        let d = "A conciliatory response that transforms a harsh exchange \
                 into a softened one through mockery.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.family, "understatement");
        assert_eq!(draft.pattern.anchor, Anchor::CrossUnit);
        assert_eq!(draft.pattern.operation, Some(Operation::Substitution));
        // dua koordinat bergerak sekaligus: intensitas turun, penerimaan sosial naik
        assert!(draft.pattern.transforms.contains(&Transform::new("intensity", Direction::Down)));
        assert!(draft.pattern.transforms.contains(&Transform::new("social acceptability", Direction::Up)));
    }

    #[test]
    fn heuristic_compiles_litotes() {
        let d = "An assertion by way of negating the opposite extreme.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.family, "understatement");
        assert!(draft.pattern.transforms.contains(&Transform::new("explicitness", Direction::Down)));
    }

    #[test]
    fn heuristic_compiles_hyperbole_amplification() {
        let d = "An exaggerated statement that overstates the magnitude of \
                 its subject beyond the baseline.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.family, "amplification");
        assert!(draft.pattern.transforms.contains(&Transform::new("magnitude", Direction::Up)));
    }

    #[test]
    fn repetition_bindings_carry_family_tag() {
        let d = "Repetition of the same word at the beginning of successive clauses.";
        let draft = compile_definition(d).unwrap();
        assert_eq!(draft.family, "repetition");
        assert!(draft.pattern.transforms.is_empty(), "structural pattern has no coordinate shift");
    }

    #[test]
    fn locus_binds_where_evidence_demands() {
        // response-slot: charientismus menjawab provokasi
        let c = compile_definition("A conciliatory response that transforms a \
            harsh exchange into a softened one through mockery.").unwrap();
        assert_eq!(c.pattern.locus_id, Some("response".into()));
        // every-slot: epistrophe menyebar ke tiap slot deret
        let e = compile_definition("Repetition of the same word or group of \
            words at the ends of successive clauses.").unwrap();
        assert_eq!(e.pattern.locus_id, Some("every".into()));
        // terminal-slot: abating hanya pada okurensi penutup
        let a = compile_definition("A concluding representation that reduces \
            the rhetorical force of what precedes it.").unwrap();
        assert_eq!(a.pattern.locus_id, Some("terminal".into()));
        // okurensi tunggal: locus runtuh ke anchor (None)
        let t = compile_definition("Truncate(word, terminal_segment) -> a \
            shortened word form produced by removing its final segment.").unwrap();
        assert_eq!(t.pattern.locus_id, None);
    }

    #[test]
    fn locus_vocabulary_grew_from_evidence() {
        // commoratio: distributed recurrence — varian lahir dari figur ini
        let cm = compile_definition("Repetition of an argumentative anchor \
            across multiple discourse positions, with intervening material \
            permitted between occurrences.").unwrap();
        assert_eq!(cm.pattern.locus_id, Some("distributed".into()));
        assert_eq!(cm.pattern.operation, Some(Operation::Repetition));
        // epimone: clustered — definisi baru MAUPUN ringkas terdeteksi
        let ep1 = compile_definition("reiterate(anchor, near_identical_unit) -> \
            persistent repetition of the same argumentative plea in \
            substantially the same verbal form.").unwrap();
        assert_eq!(ep1.pattern.locus_id, Some("clustered".into()));
        let ep2 = compile_definition("Persistent dwelling on point; refrain; \
            commoratio.").unwrap();
        assert_eq!(ep2.pattern.locus_id, Some("clustered".into()));
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
            unit_id: Some("word".into()),
            operation: Some(Operation::Repetition),
            locus_id: None,
            transforms: vec![],
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
