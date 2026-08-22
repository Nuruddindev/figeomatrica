use crate::{Anchor, ElementClass, Grain, Operation, Direction, Transform, Locus};
use super::{GeometricHypothesis, MissingParam, CandidateValue, HypothesisStatus};

/// Generate missing parameter candidates for an under-specified hypothesis
pub fn generate_candidates(hypothesis: &mut GeometricHypothesis, definition: &str) {
    let d = definition.to_lowercase();
    
    // Unit candidates based on definition
    if hypothesis.unit_id == "unit" { // default fallback
        hypothesis.missing_params.push(MissingParam {
            name: "unit".into(),
            candidates: suggest_unit_candidates(&d),
            rationale: "Definition doesn't clearly specify the unit of operation".into(),
        });
    }
    
    // Locus candidates
    if hypothesis.locus_id.is_none() {
        hypothesis.missing_params.push(MissingParam {
            name: "locus".into(),
            candidates: suggest_locus_candidates(&d),
            rationale: "Definition doesn't specify locus of operation".into(),
        });
    }
    
    // Coordinate candidates
    if hypothesis.coordinate_id.is_none() {
        hypothesis.missing_params.push(MissingParam {
            name: "coordinate".into(),
            candidates: suggest_coordinate_candidates(&d),
            rationale: "Coordinate axis not specified".into(),
        });
    }
}

fn suggest_unit_candidates(d: &str) -> Vec<CandidateValue> {
    let mut candidates = Vec::new();
    
    if d.contains("grapheme") || d.contains("letter") || d.contains("character") || d.contains("sound") {
        candidates.push(CandidateValue { value: "grapheme".into(), label: "Grapheme/Letter/Sound".into(), confidence: 0.9 });
    }
    if d.contains("word") || d.contains("lexeme") {
        candidates.push(CandidateValue { value: "word".into(), label: "Word".into(), confidence: 0.85 });
    }
    if d.contains("phrase") || d.contains("collocation") {
        candidates.push(CandidateValue { value: "phrase".into(), label: "Phrase".into(), confidence: 0.8 });
    }
    if d.contains("clause") || d.contains("sentence") {
        candidates.push(CandidateValue { value: "unit".into(), label: "Clause/Sentence".into(), confidence: 0.8 });
    }
    if d.contains("discourse") || d.contains("paragraph") || d.contains("discourse") {
        candidates.push(CandidateValue { value: "discourse".into(), label: "Discourse".into(), confidence: 0.7 });
    }
    
    if candidates.is_empty() {
        candidates.push(CandidateValue { value: "word".into(), label: "Word (default)".into(), confidence: 0.5 });
    }
    
    candidates
}

fn suggest_locus_candidates(d: &str) -> Vec<CandidateValue> {
    let mut candidates = Vec::new();
    
    if d.contains("initial") || d.contains("beginning") || d.contains("start") || d.contains("left boundary") {
        candidates.push(CandidateValue { value: "initial".into(), label: "Initial".into(), confidence: 0.9 });
    }
    if d.contains("final") || d.contains("end") || d.contains("terminal") || d.contains("conclusion") {
        candidates.push(CandidateValue { value: "final".into(), label: "Final".into(), confidence: 0.9 });
    }
    if d.contains("medial") || d.contains("middle") || d.contains("interior") || d.contains("center") {
        candidates.push(CandidateValue { value: "medial".into(), label: "Medial".into(), confidence: 0.8 });
    }
    if d.contains("every") || d.contains("each") || d.contains("successive") || d.contains("every slot") {
        candidates.push(CandidateValue { value: "every_slot".into(), label: "Every Slot".into(), confidence: 0.8 });
    }
    if d.contains("response") || d.contains("answer") || d.contains("reply") || d.contains("retort") {
        candidates.push(CandidateValue { value: "response".into(), label: "Response Slot".into(), confidence: 0.8 });
    }
    if d.contains("cross") || d.contains("boundary") || d.contains("across") {
        candidates.push(CandidateValue { value: "cross_unit".into(), label: "Cross Unit".into(), confidence: 0.7 });
    }
    if d.contains("distributed") || d.contains("intervening") || d.contains("scattered") {
        candidates.push(CandidateValue { value: "distributed".into(), label: "Distributed".into(), confidence: 0.7 });
    }
    if d.contains("clustered") || d.contains("adjacent") || d.contains("consecutive") {
        candidates.push(CandidateValue { value: "clustered".into(), label: "Clustered".into(), confidence: 0.7 });
    }
    
    if candidates.is_empty() {
        candidates.push(CandidateValue { value: "whole_unit".into(), label: "Whole Unit (default)".into(), confidence: 0.4 });
    }
    
    candidates
}

fn suggest_coordinate_candidates(d: &str) -> Vec<CandidateValue> {
    let mut candidates = Vec::new();
    
    if d.contains("magnitude") || d.contains("size") || d.contains("degree") {
        candidates.push(CandidateValue { value: "magnitude".into(), label: "Magnitude".into(), confidence: 0.8 });
    }
    if d.contains("intensity") || d.contains("force") || d.contains("strength") {
        candidates.push(CandidateValue { value: "intensity".into(), label: "Intensity".into(), confidence: 0.8 });
    }
    if d.contains("status") || d.contains("rank") || d.contains("standing") {
        candidates.push(CandidateValue { value: "status".into(), label: "Status".into(), confidence: 0.7 });
    }
    if d.contains("importance") || d.contains("significance") || d.contains("weight") {
        candidates.push(CandidateValue { value: "importance".into(), label: "Importance".into(), confidence: 0.7 });
    }
    if d.contains("force") || d.contains("persuasive") || d.contains("power") {
        candidates.push(CandidateValue { value: "force".into(), label: "Force".into(), confidence: 0.7 });
    }
    if d.contains("explicit") || d.contains("implicit") || d.contains("overt") || d.contains("covert") {
        candidates.push(CandidateValue { value: "explicitness".into(), label: "Explicitness".into(), confidence: 0.7 });
    }
    if d.contains("social") || d.contains("polite") || d.contains("decorum") {
        candidates.push(CandidateValue { value: "social_acceptability".into(), label: "Social Acceptability".into(), confidence: 0.6 });
    }
    if d.contains("order") || d.contains("sequence") || d.contains("alphabet") {
        candidates.push(CandidateValue { value: "order".into(), label: "Order".into(), confidence: 0.8 });
    }
    
    if candidates.is_empty() {
        candidates.push(CandidateValue { value: "magnitude".into(), label: "Magnitude (default)".into(), confidence: 0.4 });
    }
    
    candidates
}
