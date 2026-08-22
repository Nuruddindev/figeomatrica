//! Geometric formalization pipeline for rhetorical figures.
//!
//! This module implements the geometric formalization pipeline:
//! Definition → Diagnosis → Hypothesis → Witness → Validation → Knowledge

pub mod hypothesis;
pub mod diagnosis;
pub mod candidates;
pub mod witness;

pub use hypothesis::{GeometricHypothesis, HypothesisStatus, MissingParam, CandidateValue, GeometricityDiagnosis};
pub use diagnosis::diagnose_geometricity;
pub use candidates::generate_candidates;
pub use witness::{GeometricWitness, WitnessGenerationResult, WitnessRequest, GeometricSignature, WitnessRequest, WitnessExample, WitnessValidation, UserJudgment, Revision, generate_witness_prompt};

use crate::{compile_definition, DraftGeometri, FigurePattern, Anchor, ElementClass, Operation, Grain, Direction};

/// Generate a geometric hypothesis from a definition
pub fn generate_hypothesis(definition: &str) -> super::GeometricHypothesis {
    use crate::{compile_definition, Anchor, ElementClass, Operation};
    
    let draft = compile_definition(definition);
    
    match draft {
        Some(draft) => {
            let status = if draft.confidence >= 0.75 {
                crate::geometry::HypothesisStatus::Geometric
            } else if draft.confidence >= 0.5 {
                super::HypothesisStatus::UnderSpecified
            } else {
                super::HypothesisStatus::NonGeometric
            };
            
            super::GeometricHypothesis {
                operation: draft.pattern.operation.unwrap_or(crate::Operation::Addition),
                anchor: draft.pattern.anchor,
                unit_id: draft.pattern.unit_id.unwrap_or_else(|| "word".into()),
                locus_id: draft.pattern.locus_id.clone(),
                coordinate_id: None, // TODO: extract from transforms
                min_repeats: draft.pattern.min_repeats,
                transforms: draft.pattern.transforms.clone(),
                confidence: draft.confidence,
                family: draft.family,
                status,
                missing_params: vec![],
                source_definition: String::new(),
            }
        }
        None => {
            super::GeometricHypothesis {
                operation: crate::Operation::Addition,
                anchor: crate::Anchor::WholeUnit,
                unit_id: "word".into(),
                locus_id: None,
                coordinate_id: None,
                min_repeats: 1,
                transforms: vec![],
                confidence: 0.0,
                family: String::new(),
                status: super::HypothesisStatus::NonGeometric,
                missing_params: vec![],
                source_definition: String::new(),
            }
        }
    }
}

/// Complete formalization pipeline
pub fn formalize(definition: &str) -> FormalizationResult {
    // 1. Diagnose geometricity
    let diagnosis = crate::geometry::diagnosis::diagnose_geometricity(definition);
    
    // 2. Generate hypothesis
    let mut hypothesis = generate_hypothesis(definition);
    hypothesis.source_definition = definition.into();
    
    // 3. Generate candidates for missing params
    if hypothesis.status == super::HypothesisStatus::UnderSpecified {
        crate::geometry::candidates::generate_candidates(&mut hypothesis, definition);
    }
    
    // 3. Generate candidate witnesses (LLM would do this)
    let witness_request = build_witness_request(&hypothesis);
    
    FormalizationResult {
        diagnosis,
        hypothesis,
        witness_request,
    }
}

pub struct FormalizationResult {
    pub diagnosis: super::GeometricityDiagnosis,
    pub hypothesis: super::GeometricHypothesis,
    pub witness_request: WitnessRequest,
}

fn build_witness_request(hypothesis: &crate::geometry::GeometricHypothesis) -> WitnessRequest {
    WitnessRequest {
        hypothesis_id: format!("hyp_{}", uuid::Uuid::new_v4().simple()),
        signature: crate::geometry::witness::GeometricSignature {
            operation: format!("{:?}", hypothesis.operation),
            anchor: format!("{:?}", hypothesis.anchor),
            unit: hypothesis.unit_id.clone(),
            locus: hypothesis.locus_id.clone(),
            coordinate: hypothesis.coordinate_id.clone(),
            repetition: hypothesis.min_repeats,
            constraints: vec![],
        },
        prompt: String::new(),
        examples: vec![],
    }
}

use uuid::Uuid;
