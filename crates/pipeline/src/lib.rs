// figeometrica-pipeline
// ─────────────────────────────────────────────────────────────────────────────
// Provenance-anchored analysis pipeline.
//
// Two-stage architecture (from the SARVA design):
//
//   Stage A — FeatureObserver: cheap, high-recall feature extraction over
//             chunks (deterministic geometry matching lives in
//             figeometrica-core; LLM-assisted observers implement the trait).
//   Stage B  — CriteriaVerifier: expensive, low-volume verification of
//             candidate findings against a figure's definition criteria,
//             always returning confidence and an `indeterminate` state when
//             evidence is insufficient.
//
// Every finding carries provenance: chunk_id + span, so negative space is
// auditable ("this figure was NOT found here" is checkable).
// ─────────────────────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

/// Coordinate of a chunk inside its modality. Text uses paragraph/sentence;
/// image uses bounding box; audio uses seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "modality", rename_all = "snake_case")]
pub enum Coord {
    Text { paragraph: u32, sentence: u32 },
    Space { x: f32, y: f32, width: f32, height: f32 },
    Time { start_sec: f32, end_sec: f32 },
}

/// Minimal analysis unit with full provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub content: String,
    pub source_id: String,
    pub coord: Coord,
}

/// Granularity of an observation relative to its source document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    /// Observation covers the whole document.
    Global,
    /// Observation covers one section/chapter.
    Sectional,
    /// Observation covers a single chunk.
    Local,
}

/// A candidate finding produced by Stage A. High recall, low precision —
/// verification happens later.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub feature: String,
    pub value: serde_json::Value,
    pub chunk_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_end: Option<usize>,
    pub scale: Scale,
}

/// Verdict of Stage B on one candidate finding.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Confirmed,
    Rejected,
    Indeterminate,
}

/// Result of verifying one observation against figure criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub observation: Observation,
    pub verdict: Verdict,
    /// 0.0–1.0; must be present even for rejections.
    pub confidence: f32,
    /// Which criteria were met / unmet / undecidable — the audit trail.
    pub criteria_met: Vec<String>,
    pub criteria_unmet: Vec<String>,
}

/// Error type for pipeline stages.
#[derive(Debug)]
pub struct StageError(pub String);

impl std::fmt::Display for StageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pipeline stage error: {}", self.0)
    }
}

impl std::error::Error for StageError {}

/// Stage A: extract features from chunks. Implementations may be pure
/// functions (geometry matcher) or wrap an LLM behind this trait.
pub trait FeatureObserver {
    fn observe(&self, chunks: &[Chunk]) -> Result<Vec<Observation>, StageError>;
}

/// Stage B: verify observations against a figure definition's criteria.
/// Must never guess: return `Verdict::Indeterminate` when evidence is
/// insufficient.
pub trait CriteriaVerifier {
    fn verify(&self, observations: &[Observation]) -> Result<Vec<Verification>, StageError>;
}

/// Adapter that runs the deterministic geometry matcher as a Stage A
/// observer. Each chunk becomes one `TextUnit`; findings keep their spans.
pub struct GeometryObserver;

impl FeatureObserver for GeometryObserver {
    fn observe(&self, chunks: &[Chunk]) -> Result<Vec<Observation>, StageError> {
        let units: Vec<figeometrica_core::TextUnit> = chunks
            .iter()
            .map(|c| figeometrica_core::TextUnit { chunk_id: &c.id, text: &c.content })
            .collect();
        let findings = figeometrica_core::GeometryMatcher::detect(&units);
        Ok(findings
            .into_iter()
            .map(|f| {
                let first = f.evidence.first();
                Observation {
                    feature: format!("geometri:{}", f.figure_name),
                    value: serde_json::to_value(&f).expect("finding serializes"),
                    chunk_id: first.map(|e| e.chunk_id.clone()).unwrap_or_default(),
                    span_start: first.map(|e| e.span_start),
                    span_end: first.map(|e| e.span_end),
                    scale: Scale::Local,
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunks() -> Vec<Chunk> {
        vec![
            Chunk { id: "c0".into(), content: "We came.".into(), source_id: "doc1".into(), coord: Coord::Text { paragraph: 0, sentence: 0 } },
            Chunk { id: "c1".into(), content: "We saw.".into(), source_id: "doc1".into(), coord: Coord::Text { paragraph: 0, sentence: 1 } },
        ]
    }

    #[test]
    fn geometry_observer_finds_anaphora_with_provenance() {
        let obs = GeometryObserver.observe(&chunks()).unwrap();
        let ana = obs.iter().find(|o| o.feature == "geometri:anaphora").expect("anaphora observed");
        assert_eq!(ana.chunk_id, "c0");
        assert_eq!(ana.scale, Scale::Local);
        assert_eq!(ana.span_end, Some(ana.span_start.unwrap() + 2));
    }

    #[test]
    fn verifier_trait_accepts_indeterminate() {
        struct AlwaysIndeterminate;
        impl CriteriaVerifier for AlwaysIndeterminate {
            fn verify(&self, observations: &[Observation]) -> Result<Vec<Verification>, StageError> {
                Ok(observations
                    .iter()
                    .map(|o| Verification {
                        observation: o.clone(),
                        verdict: Verdict::Indeterminate,
                        confidence: 0.5,
                        criteria_met: vec![],
                        criteria_unmet: vec!["definition not yet geometrized".into()],
                    })
                    .collect())
            }
        }
        let obs = GeometryObserver.observe(&chunks()).unwrap();
        let verifs = AlwaysIndeterminate.verify(&obs).unwrap();
        assert!(verifs.iter().all(|v| v.verdict == Verdict::Indeterminate));
    }

    #[test]
    fn chunk_roundtrips_through_json() {
        let c = &chunks()[0];
        let json = serde_json::to_string(c).unwrap();
        let back: Chunk = serde_json::from_str(&json).unwrap();
        assert_eq!(back.coord, Coord::Text { paragraph: 0, sentence: 0 });
    }
}
