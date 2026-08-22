use crate::{compile_definition, DraftGeometri};

/// Diagnose the geometricity of a definition.
/// Returns a diagnosis with status and rationale.
pub fn diagnose_geometricity(definition: &str) -> super::GeometricityDiagnosis {
    let draft = compile_definition(definition);
    
    match draft {
        Some(d) if d.confidence >= 0.75 => {
            let mut diag = super::GeometricityDiagnosis::new(
                super::HypothesisStatus::Geometric,
                "Definition provides sufficient geometric parameters with high confidence"
            );
            diag.geometric_elements = extract_geometric_elements(&d);
            diag
        }
        Some(d) if d.confidence >= 0.5 => {
            let mut diag = super::GeometricityDiagnosis::new(
                super::HypothesisStatus::UnderSpecified,
                "Definition has some geometric content but parameters are ambiguous or incomplete"
            );
            diag.geometric_elements = extract_geometric_elements(&d);
            diag
        }
        Some(_) => {
            super::GeometricityDiagnosis::new(
                super::HypothesisStatus::NonGeometric,
                "Definition does not contain sufficient geometric information"
            )
        }
        None => {
            super::GeometricityDiagnosis::new(
                super::HypothesisStatus::NonGeometric,
                "No geometric patterns detected in definition"
            )
        }
    }
}

fn extract_geometric_elements(d: &crate::DraftGeometri) -> Vec<String> {
    let mut elements = Vec::new();
    elements.push(format!("operation: {:?}", d.pattern.operation));
    elements.push(format!("anchor: {:?}", d.pattern.anchor));
    elements.push(format!("class: {:?}", d.pattern.class));
    elements.push(format!("unit: {:?}", d.pattern.unit_id));
    if let Some(locus) = &d.pattern.locus_id {
        elements.push(format!("locus: {}", locus));
    }
    if !d.transforms.is_empty() {
        elements.push(format!("transforms: {:?}", d.transforms));
    }
    if let Some(note) = &d.pattern.note {
        elements.push(format!("note: {}", note));
    }
    vec![]
}
