use serde::{Deserialize, Serialize};
use crate::{Anchor, ElementClass, Grain, Operation, Direction, Transform};

/// Geometric hypothesis generated from a definition.
/// Intermediate representation before user validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricHypothesis {
    pub operation: Operation,
    pub anchor: Anchor,
    pub unit_id: String,
    pub locus_id: Option<String>,
    pub coordinate_id: Option<String>,
    pub min_repeats: usize,
    pub transforms: Vec<Transform>,
    pub confidence: f32,
    pub family: String,
    pub status: HypothesisStatus,
    pub missing_params: Vec<MissingParam>,
    pub source_definition: String,
}

/// Status of the geometric hypothesis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HypothesisStatus {
    /// Definition provides enough info for complete signature
    Geometric,
    /// Some parameters missing, need candidate generation
    UnderSpecified,
    /// Definition has no geometric content
    NonGeometric,
    /// Mixed geometric + semantic content
    Mixed,
}

/// Missing parameter that needs candidate generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingParam {
    pub name: String,
    pub candidates: Vec<CandidateValue>,
    pub rationale: String,
}

/// Candidate value for a missing parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateValue {
    pub value: String,
    pub label: String,
    pub confidence: f32,
}

impl GeometricHypothesis {
    pub fn is_complete(&self) -> bool {
        self.missing_params.is_empty()
    }

    pub fn to_figure_pattern(&self) -> crate::FigurePattern {
        crate::FigurePattern {
            name: String::new(),
            template: vec![],
            anchor: self.anchor,
            class: ElementClass::Lexical, // default, should be set by user
            min_repeats: self.min_repeats,
            unit_id: Some(self.unit_id.clone()),
            operation: Some(self.operation),
            locus_id: self.locus_id.clone(),
            transforms: self.transforms.clone(),
            note: Some(self.family.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricityDiagnosis {
    pub status: HypothesisStatus,
    pub geometric_elements: Vec<String>,
    pub semantic_elements: Vec<String>,
    pub rationale: String,
}

impl GeometricityDiagnosis {
    pub fn new(status: HypothesisStatus, rationale: &str) -> Self {
        Self {
            status,
            geometric_elements: vec![],
            semantic_elements: vec![],
            rationale: rationale.to_string(),
        }
    }
}
