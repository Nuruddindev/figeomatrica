// figeometrica-rhetorica
// ─────────────────────────────────────────────────────────────────────────────
// The classical-rhetoric theory base as data.
//
// A theory base is a versioned JSON dataset: figures with their compiled
// geometric specs, plus category links. The loader is theory-agnostic —
// future bases (e.g. fallacies as apparent enthymemes) follow the same
// shape and load through the same API.
//
// NOTE on definitions: prose definitions are intentionally NOT shipped yet.
// Many current definitions derive from copyrighted secondary sources and are
// being rewritten as original text before publication. Structure (names,
// categories, geometry) is public-domain classical material.
// ─────────────────────────────────────────────────────────────────────────────

use figeometrica_core::FigurePattern;
use serde::{Deserialize, Serialize};

/// Example sentences attached to a figure. Each example is a sequence of
/// discourse units (most figures need one unit; cross-unit figures like
/// anaphora need several).
///
/// Positives MUST trigger the figure's geometry when machine-verifiable;
/// negatives MUST NOT. The validator (`src/bin/validate.rs`) enforces this.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Examples {
    #[serde(default, alias = "positif")]
    pub positive: Vec<Vec<String>>,
    #[serde(default, alias = "negatif")]
    pub negative: Vec<Vec<String>>,
}

/// CONTRACT.md §7 — posisi figur di tangga epistemik + catatan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epistemic {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One figure entry of the theory base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureEntry {
    pub id: u32,
    pub name: String,
    /// Compiled geometry; `None` = definition not yet geometrized.
    /// Field name is English "geometry"; Indonesian "geometri" accepted for backward compat.
    #[serde(default, alias = "geometri")]
    pub geometry: Option<FigurePattern>,
    /// CONTRACT §2 — geometric signature (blok kontrak; opsional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<figeometrica_core::FigureSignature>,
    /// CONTRACT §7/§12 — tangga epistemik (opsional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epistemic: Option<Epistemic>,
    #[serde(default)]
    pub categories: Vec<String>,
    /// Examples: English "examples" with "positive"/"negative"; Indonesian "contoh" with "positif"/"negatif" accepted.
    #[serde(default, alias = "contoh")]
    pub examples: Option<Examples>,
    /// GitHub usernames / attribution for this entry.
    #[serde(default)]
    pub atribusi: Option<serde_json::Value>,
}

/// The rhetoric theory base.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rhetorica {
    #[serde(default)]
    pub figures: Vec<FigureEntry>,
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "cannot read theory base: {e}"),
            LoadError::Json(e) => write!(f, "invalid theory base JSON: {e}"),
        }
    }
}

impl std::error::Error for LoadError {}

impl std::str::FromStr for Rhetorica {
    type Err = LoadError;

    /// Parse a theory base from a JSON string. Fills empty embedded pattern
    /// names from their parent figure entries.
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let mut base: Rhetorica = serde_json::from_str(json).map_err(LoadError::Json)?;
        for f in &mut base.figures {
            if let Some(g) = &mut f.geometry {
                if g.name.is_empty() {
                    g.name = f.name.clone();
                }
            }
        }
        Ok(base)
    }
}

impl Rhetorica {
    /// Embedded dataset: per-figure files from `data/figures/*.json` merged
    /// by build.rs at compile time.
    pub fn embedded_json() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/figures.json"))
    }

    /// Load the embedded dataset.
    pub fn embedded() -> Result<Rhetorica, LoadError> {
        use std::str::FromStr;
        Rhetorica::from_str(Self::embedded_json())
    }

    /// Load from a file path.
    pub fn from_path(path: &std::path::Path) -> Result<Rhetorica, LoadError> {
        use std::str::FromStr;
        Rhetorica::from_str(&std::fs::read_to_string(path).map_err(LoadError::Io)?)
    }

    /// Figures whose definitions have been compiled to geometry.
    pub fn geometrized(&self) -> impl Iterator<Item = &FigureEntry> {
        self.figures.iter().filter(|f| f.geometry.is_some())
    }

    /// Look up a figure by name (exact match).
    pub fn figure(&self, name: &str) -> Option<&FigureEntry> {
        self.figures.iter().find(|f| f.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use figeometrica_core::{Anchor, ElementClass};
    use std::str::FromStr;

    #[test]
    fn embedded_base_loads_with_full_catalog() {
        let r = Rhetorica::embedded().unwrap();
        assert!(r.figures.len() > 400, "expected the full figure catalog");
    }

    #[test]
    fn anaphora_has_compiled_geometry() {
        let r = Rhetorica::embedded().unwrap();
        let f = r.figure("anaphora").expect("anaphora present");
        let g = f.geometry.as_ref().expect("anaphora geometrized");
        assert_eq!(g.anchor, Anchor::Initial);
        assert_eq!(g.class, ElementClass::Lexical);
        assert_eq!(g.min_repeats, 2);
    }

    #[test]
    fn geometrized_dataset_grows_with_contributions() {
        let r = Rhetorica::embedded().unwrap();
        // The dataset is contributor-owned: it must always cover the core
        // catalog and may legitimately grow beyond it (abating, anesis, ...).
        assert!(
            r.geometrized().count() >= FigurePattern::catalog().len(),
            "dataset should cover at least the core catalog"
        );
    }

    #[test]
    fn indonesian_geometry_aliases_load() {
        // SARVA DB convention inside geometri must deserialize via aliases.
        let json = r#"{"figures":[{"id":1,"name":"tmesis","geometri":{"nama":"tmesis","jangkar":"Sisipan","kelas":"Leksikal","satuan":"grafem","operasi":"adjectio","minim_ulangan":1,"template":[],"catatan":"kata dipotong"}}]}"#;
        let r = Rhetorica::from_str(json).unwrap();
        let g = r.figure("tmesis").unwrap().geometry.as_ref().unwrap();
        assert_eq!(g.anchor, Anchor::Insertion);
        assert_eq!(g.unit_id, Some("grafem".to_string()));
        assert_eq!(g.operation, Some(figeometrica_core::Operation::Addition));
    }
}
