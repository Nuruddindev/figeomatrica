//! FigureSignature — CONTRACT.md v1 §2.
//!
//! Data-only representation of a figure's machine-operational definition.
//! Every vocabulary slot is an id referencing the knowledge layer
//! (SARVA tables: domains/units/scopes/anchors/payloads/loci); the closed
//! algebra (Operation) is the only enum. Implementation obeys the
//! contract: no concept enters here that the contract does not name.

use crate::Operation;
use serde::{Deserialize, Serialize};

/// CONTRACT §4 constraints collection. Absent slots are simply absent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Constraints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_occurrences: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_occurrences: Option<u32>,
    /// Free-form until v1.x tightens it; renaming requires a contract bump.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjacency: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclusions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_relation: Option<String>,
}

/// The machine-operational definition of a figure (CONTRACT §2).
/// `Figure`, `FigureSignature`, and `VerificationRecord` are three distinct
/// entities; this struct is only the geometric claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FigureSignature {
    pub domain_id: String,
    pub unit_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    pub anchor_id: String,
    pub operation: Operation,
    /// Degenerate at grapheme level; MAY be implicit (CONTRACT §2.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locus_id: Option<String>,
    /// Stored, human-readable outcome label. Never computed (CONTRACT §3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Constraints::is_empty")]
    pub constraints: Constraints,
}

impl Constraints {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// CONTRACT §6: compatibility verdicts are knowledge-defined. There is no
/// Rust match on entity types — only a lookup against recorded bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingVerdict {
    /// Binding recorded valid in the knowledge layer.
    Valid,
    /// No binding record. Not an error — the system stays epistemically
    /// open and flags the binding as candidate knowledge.
    Unknown,
    /// Binding recorded invalid, with reason + provenance in the store.
    Invalid,
}

/// Minimal read interface over the knowledge layer's bindings table so the
/// checker stays dependency-free (no rusqlite in core). SARVA implements
/// this with its SQLite connection.
pub trait BindingStore {
    fn lookup(
        &self,
        anchor: &str,
        payload: &str,
        operation: &str,
        domain: &str,
    ) -> BindingVerdict;
}

/// Type check per CONTRACT §6. An implicit payload cannot be checked and
/// yields Unknown rather than failing — degeneracy is legal, not invalid.
pub fn check_compatibility(
    sig: &FigureSignature,
    store: &dyn BindingStore,
) -> BindingVerdict {
    let Some(payload) = sig.payload_id.as_deref() else {
        return BindingVerdict::Unknown;
    };
    store.lookup(
        &sig.anchor_id,
        payload,
        sig.operation.as_str(),
        &sig.domain_id,
    )
}

impl Operation {
    /// Stable id used by the knowledge layer's bindings.operation_id.
    pub fn as_str(&self) -> &'static str {
        match self {
            Operation::Addition => "adjectio",
            Operation::Deletion => "detractio",
            Operation::Substitution => "immutatio",
            Operation::Permutation => "transmutatio",
            Operation::Repetition => "repetitio",
            Operation::Ordering => "ordering",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prosopopoeia() -> FigureSignature {
        serde_json::from_value(serde_json::json!({
            "domain_id": "entity",
            "unit_id": "entity",
            "scope_id": "representation",
            "anchor_id": "non-person",
            "operation": "addition",
            "payload_id": "person",
            "locus_id": "entity",
            "result": "personated entity"
        }))
        .unwrap()
    }

    fn personification() -> FigureSignature {
        let mut s = prosopopoeia();
        s.anchor_id = "non-human".into();
        s.payload_id = Some("human-attribute".into());
        s.result = Some("anthropomorphized entity".into());
        s
    }

    fn ethopoeia() -> FigureSignature {
        let mut s = prosopopoeia();
        s.unit_id = "character".into();
        s.anchor_id = "person".into();
        s.payload_id = Some("characterological-attribute".into());
        s.result = Some("characterized entity".into());
        s
    }

    struct Fixed(BindingVerdict);
    impl BindingStore for Fixed {
        fn lookup(&self, _: &str, _: &str, _: &str, _: &str) -> BindingVerdict {
            self.0
        }
    }

    #[test]
    fn same_operation_different_payloads_are_distinct_signatures() {
        let p = prosopopoeia();
        let pe = personification();
        let e = ethopoeia();
        assert_ne!(p, pe);
        assert_ne!(p, e);
        assert_ne!(pe, e);
        // All three share only the operation — proving the discriminator
        // lives in anchor/payload, exactly as CONTRACT §2.3 states.
        assert_eq!(p.operation, pe.operation);
        assert_eq!(pe.operation, e.operation);
    }

    #[test]
    fn verdicts_pass_through_from_store() {
        assert_eq!(
            check_compatibility(&prosopopoeia(), &Fixed(BindingVerdict::Valid)),
            BindingVerdict::Valid
        );
        assert_eq!(
            check_compatibility(&personification(), &Fixed(BindingVerdict::Unknown)),
            BindingVerdict::Unknown
        );
        assert_eq!(
            check_compatibility(&ethopoeia(), &Fixed(BindingVerdict::Invalid)),
            BindingVerdict::Invalid
        );
    }

    #[test]
    fn implicit_payload_is_unknown_not_invalid() {
        let mut s = prosopopoeia();
        s.payload_id = None;
        assert_eq!(
            check_compatibility(&s, &Fixed(BindingVerdict::Invalid)),
            BindingVerdict::Unknown
        );
    }

    #[test]
    fn roundtrip_json_keeps_optional_slots_absent() {
        let s = prosopopoeia();
        let j = serde_json::to_string(&s).unwrap();
        assert!(!j.contains("constraints"), "empty constraints must vanish");
        let back: FigureSignature = serde_json::from_str(&j).unwrap();
        assert_eq!(back, s);
    }
}
