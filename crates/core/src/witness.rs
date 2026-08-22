//! Deterministic witness engine — CONTRACT.md §8.
//!
//! For textual domains (unit = word/grapheme/syllable with positional
//! anchors) witnesses can be constructed algorithmically on segmented
//! carriers, so protocol validation never depends on an LLM. Higher domains
//! (entity/argument) return [`DeterministicUnsupported`] and wait for the
//! LLM constructor path; they are never judged by generation alone.
//!
//! Carrier encoding: segments joined by `-`, e.g. `ka-ta` is a word with
//! two segments. Encoding keeps segment addresses unambiguous so the
//! structural check and the inverse test are exact, not heuristic.

use crate::signature::FigureSignature;
use crate::Operation;
use serde::{Deserialize, Serialize};

/// CONTRACT §8.2 guided order: payload first, then locus, then anchor.
/// Implicit payload (degenerate at grapheme level) skips its slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WitnessKind {
    Positive,
    NegativePayload,
    NegativeLocus,
    NegativeAnchor,
}

impl WitnessKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            WitnessKind::Positive => "positive",
            WitnessKind::NegativePayload => "negative-payload",
            WitnessKind::NegativeLocus => "negative-locus",
            WitnessKind::NegativeAnchor => "negative-anchor",
        }
    }
}

/// A minimal textual artifact pair realizing (or violating) a signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextWitness {
    pub kind: WitnessKind,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationOutcome {
    /// Battery produced; each witness is ready for protocol validation.
    Generated(Vec<TextWitness>),
    /// Domain outside deterministic reach — LLM constructor path (§8 preamble).
    DeterministicUnsupported {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    /// Transformation does not realize the declared operation.
    OperationMismatch { declared: String, observed: String },
    /// Change lands elsewhere than the declared anchor.
    AnchorMismatch { declared: String, observed: String },
    /// Locus declaration contradicts the observed address.
    LocusMismatch { declared: String, observed: String },
    /// Carrier not parseable or feature beyond deterministic scope.
    Unsupported(String),
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::OperationMismatch { declared, observed } => write!(
                f, "operasi '{declared}' tidak cocok dengan yang teramati '{observed}'"
            ),
            Violation::AnchorMismatch { declared, observed } => write!(
                f, "anchor '{declared}' tidak cocok dengan posisi perubahan '{observed}'"
            ),
            Violation::LocusMismatch { declared, observed } => write!(
                f, "locus '{declared}' bertentangan dengan alamat '{observed}'"
            ),
            Violation::Unsupported(r) => write!(f, "di luar jangkauan deterministik: {r}"),
        }
    }
}

fn segmen(teks: &str) -> Vec<&str> {
    teks.split('-').filter(|s| !s.is_empty()).collect()
}

fn posisi_ke_anchor(idx: usize, len_before: usize, len_after: usize) -> &'static str {
    let base_len = len_before.max(len_after);
    if idx == 0 && base_len > 1 {
        "initial"
    } else if idx + 1 == base_len {
        "final"
    } else {
        "medial"
    }
}

fn anchor_sig_ke_str(anchor_id: &str) -> Option<&'static str> {
    match anchor_id {
        "initial-segment" => Some("initial"),
        "final-segment" => Some("final"),
        "medial-segment" => Some("medial"),
        _ => None,
    }
}

/// CONTRACT §8 structural verification: does (before → after) actually
/// realize the signature's operation/anchor/locus?
pub fn satisfies(
    sig: &FigureSignature,
    before: &str,
    after: &str,
) -> Result<(), Violation> {
    let b = segmen(before);
    let a = segmen(after);

    let (operation_observed, idx): (&str, usize) = match (b.len(), a.len()) {
        (x, y) if y + 1 == x => {
            // cari segmen yang hilang
            let mut i = 0;
            while i < a.len() && b[i] == a[i] {
                i += 1;
            }
            ("detractio", i.min(x - 1))
        }
        (x, y) if y == x + 1 => {
            let mut i = 0;
            while i < b.len() && b[i] == a[i] {
                i += 1;
            }
            ("adjectio", i.min(y - 1))
        }
        _ => {
            return Err(Violation::Unsupported(format!(
                "perubahan panjang {b:?} → {a:?} bukan tambah/hapus satu segmen"
            )))
        }
    };

    let op_declared = sig.operation.as_str();
    if op_declared != operation_observed {
        return Err(Violation::OperationMismatch {
            declared: op_declared.into(),
            observed: operation_observed.into(),
        });
    }

    let anchor_observed = posisi_ke_anchor(idx, b.len(), a.len());
    match anchor_sig_ke_str(&sig.anchor_id) {
        Some(declared) if declared != anchor_observed => {
            return Err(Violation::AnchorMismatch {
                declared: sig.anchor_id.clone(),
                observed: anchor_observed.into(),
            })
        }
        Some(_) => {}
        None => {
            return Err(Violation::Unsupported(format!(
                "anchor '{}' belum didukung pemeriksa deterministik",
                sig.anchor_id
            )))
        }
    }

    if let Some(locus) = &sig.locus_id {
        let cocok = match locus.as_str() {
            "initial" | "medial" | "cross_unit" | "cross-boundary" => {
                locus.starts_with(anchor_observed[..3].get(..3).unwrap_or(""))
                    || locus == "cross_unit"
                        && (anchor_observed == "initial" || anchor_observed == "final")
                    || locus == anchor_observed
            }
            other => other == anchor_observed,
        };
        if !cocok {
            return Err(Violation::LocusMismatch {
                declared: locus.clone(),
                observed: anchor_observed.into(),
            });
        }
    }

    Ok(())
}

/// CONTRACT §8.4 inverse test input: reconstruct operation + anchor from a
/// witness alone, without seeing the figure name or its signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferredTransform {
    pub operation: String,
    pub anchor: String,
}

pub fn infer_transform(before: &str, after: &str) -> Result<InferredTransform, Violation> {
    let b = segmen(before);
    let a = segmen(after);
    let (operation, idx) = match (b.len(), a.len()) {
        (x, y) if y + 1 == x => {
            let mut i = 0;
            while i < a.len() && b[i] == a[i] {
                i += 1;
            }
            ("detractio", i.min(x - 1))
        }
        (x, y) if y == x + 1 => {
            let mut i = 0;
            while i < b.len() && b[i] == a[i] {
                i += 1;
            }
            ("adjectio", i.min(y - 1))
        }
        _ => {
            return Err(Violation::Unsupported(
                "hanya tambah/hapus satu segmen yang bisa direkonstruksi".into(),
            ))
        }
    };
    Ok(InferredTransform {
        operation: operation.into(),
        anchor: posisi_ke_anchor(idx, b.len(), a.len()).into(),
    })
}

/// CONTRACT §8.4: signature → witness → reconstruction → signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InverseVerdict {
    Match,
    Mismatch { detail: String },
}

pub fn inverse_test(
    sig: &FigureSignature,
    before: &str,
    after: &str,
) -> InverseVerdict {
    let Ok(inf) = infer_transform(before, after) else {
        return InverseVerdict::Mismatch {
            detail: "witness tak bisa direkonstruksi".into(),
        };
    };
    let Some(declared_anchor) = anchor_sig_ke_str(&sig.anchor_id) else {
        return InverseVerdict::Mismatch {
            detail: format!("anchor '{}' di luar cakupan inversi", sig.anchor_id),
        };
    };
    if inf.operation != sig.operation.as_str() || inf.anchor != declared_anchor {
        return InverseVerdict::Mismatch {
            detail: format!(
                "terkonstruksi {}×{} ≠ diklaim {}×{}",
                inf.operation,
                inf.anchor,
                sig.operation.as_str(),
                declared_anchor
            ),
        };
    }
    InverseVerdict::Match
}

/// One row of the protocol report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolCheck {
    pub kind: WitnessKind,
    /// What the contract demands for this kind.
    pub expected: Expectation,
    /// Whether `satisfies` accepted the pair.
    pub observed_ok: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expectation {
    Pass,
    Fail,
}

/// CONTRACT §8 full deterministic battery result. `passed == true` means the
/// signature survived every probe and may advance one ladder rung with
/// reason "witness-protocol" — never silently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolReport {
    pub passed: bool,
    pub inverse: InverseVerdict,
    pub checks: Vec<ProtocolCheck>,
}

/// Run the generated battery through structural verification. Positive
/// witnesses must satisfy; every negative witness must violate. A passing
/// negative means the signature cannot separate itself from its own
/// counterexamples — the definition is broken, not the engine.
pub fn run_protocol(sig: &FigureSignature) -> Result<ProtocolReport, String> {
    let battery = match generate_deterministic(sig) {
        GenerationOutcome::Generated(b) => b,
        GenerationOutcome::DeterministicUnsupported { reason } => return Err(reason),
    };

    let mut checks = Vec::new();
    let mut passed = true;
    let mut positive = None;

    for w in &battery {
        let ok = satisfies(sig, &w.before, &w.after).is_ok();
        let expected = match w.kind {
            WitnessKind::Positive => {
                if ok {
                    positive = Some((w.before.clone(), w.after.clone()));
                }
                Expectation::Pass
            }
            _ => Expectation::Fail,
        };
        if ok != (expected == Expectation::Pass) {
            passed = false;
        }
        checks.push(ProtocolCheck { kind: w.kind, expected, observed_ok: ok });
    }

    let inverse = match (&positive, passed) {
        (Some((b, a)), _) => inverse_test(sig, b, a),
        // No usable positive witness → inverse cannot even start.
        _ => InverseVerdict::Mismatch {
            detail: "tidak ada witness positif untuk rekonstruksi".into(),
        },
    };
    if inverse != InverseVerdict::Match {
        passed = false;
    }

    Ok(ProtocolReport { passed, inverse, checks })
}

/// Canonical three-segment carrier keeps every position distinguishable.
const KARIER: &str = "a-b-c";
const PAYLOAD_TOKEN: &str = "x";

fn terapkan(op: Operation, anchor_id: &str, karier: &str, payload: &str) -> Option<String> {
    let s = segmen(karier);
    match (op, anchor_id) {
        (Operation::Deletion, "initial-segment") => Some(s[1..].join("-")),
        (Operation::Deletion, "final-segment") => Some(s[..s.len() - 1].join("-")),
        (Operation::Deletion, "medial-segment") => {
            if s.len() < 3 {
                None
            } else {
                let mut out = s.clone();
                out.remove(1);
                Some(out.join("-"))
            }
        }
        (Operation::Addition, "initial-segment") => {
            let mut out = vec![payload];
            out.extend_from_slice(&s);
            Some(out.join("-"))
        }
        (Operation::Addition, "final-segment") => {
            let mut out = s.clone();
            out.push(payload);
            Some(out.join("-"))
        }
        (Operation::Addition, "medial-segment") => {
            if s.len() < 2 {
                None
            } else {
                let mut out = s.clone();
                out.insert(1, payload);
                Some(out.join("-"))
            }
        }
        _ => None,
    }
}

/// Generate the deterministic battery for a textual-domain signature.
/// Negative order follows CONTRACT §8.2; implicit payload skips
/// Negative-Payload (nothing removable that the signature names).
pub fn generate_deterministic(sig: &FigureSignature) -> GenerationOutcome {
    if sig.domain_id != "textual" {
        return GenerationOutcome::DeterministicUnsupported {
            reason: format!(
                "domain '{}' menunggu jalur konstruktor LLM (CONTRACT §8)",
                sig.domain_id
            ),
        };
    }
    if ![Operation::Deletion, Operation::Addition].contains(&sig.operation) {
        return GenerationOutcome::DeterministicUnsupported {
            reason: format!(
                "operasi '{}' belum punya generator deterministik",
                sig.operation.as_str()
            ),
        };
    }

    let mut battery = Vec::new();

    // Positive — kalau kombinasi operasi×anchor bahkan tak bisa
    // dikonstruksi, seluruh signature di luar jangkauan deterministik;
    // baterai tanpa positif tidak boleh dihukum sebagai GAGAL.
    let Some(after) = terapkan(sig.operation, &sig.anchor_id, KARIER, PAYLOAD_TOKEN) else {
        return GenerationOutcome::DeterministicUnsupported {
            reason: format!(
                "kombinasi {}×{} belum punya konstruktor deterministik",
                sig.anchor_id,
                sig.operation.as_str()
            ),
        };
    };
    battery.push(TextWitness {
        kind: WitnessKind::Positive,
        before: KARIER.into(),
        after,
    });

    // Negative-Payload — hanya jika payload eksplisit: hapus payload dari
    // transformasi (tanpa mengubah apa pun = bukan figur).
    if sig.payload_id.is_some() {
        battery.push(TextWitness {
            kind: WitnessKind::NegativePayload,
            before: KARIER.into(),
            after: KARIER.into(),
        });
    }

    // Negative-Locus — operasi sama, alamat berbeda: harus GAGAL cek.
    for alt in ["initial-segment", "final-segment", "medial-segment"] {
        if alt != sig.anchor_id {
            if let Some(after) = terapkan(sig.operation, alt, KARIER, PAYLOAD_TOKEN) {
                battery.push(TextWitness {
                    kind: WitnessKind::NegativeLocus,
                    before: KARIER.into(),
                    after,
                });
                break; // satu probe informatif cukup (maks 3, §8.2)
            }
        }
    }

    // Negative-Anchor — operasi diterapkan ke unit utuh: pasti melanggar.
    if sig.anchor_id != "whole-unit" {
        battery.push(TextWitness {
            kind: WitnessKind::NegativeAnchor,
            before: KARIER.into(),
            after: String::new(),
        });
    }

    GenerationOutcome::Generated(battery)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn syncope_sig() -> FigureSignature {
        serde_json::from_value(serde_json::json!({
            "domain_id": "textual",
            "unit_id": "word",
            "anchor_id": "medial-segment",
            "operation": "detractio",
            "locus_id": "medial"
        }))
        .unwrap()
    }

    #[test]
    fn positif_memenuhi_signature() {
        let sig = syncope_sig();
        assert!(satisfies(&sig, "a-b-c", "a-c").is_ok());
        assert!(satisfies(&sig, "a-b-c", "b-c").is_err()); // salah alamat
    }

    #[test]
    fn baterai_negatif_gagal_cekh_sesuai_kontrak() {
        let sig = syncope_sig();
        let GenerationOutcome::Generated(battery) = generate_deterministic(&sig) else {
            panic!("harus tergenerasi");
        };
        assert_eq!(battery[0].kind, WitnessKind::Positive);
        // semua negatif HARUS gagal satisfies (§8.2)
        for w in battery.iter().skip(1) {
            assert!(
                satisfies(&sig, &w.before, &w.after).is_err(),
                "{:?} harus melanggar", w.kind
            );
        }
    }

    #[test]
    fn inverse_roundtrip_match_untuk_syncope() {
        let sig = syncope_sig();
        assert_eq!(inverse_test(&sig, "a-b-c", "a-c"), InverseVerdict::Match);
        // witness salah arah → rekonstruksi beda → Mismatch
        assert!(matches!(
            inverse_test(&sig, "a-b-c", "b-c"),
            InverseVerdict::Mismatch { .. }
        ));
    }

    #[test]
    fn protokol_penuh_lulus_untuk_signature_sehat() {
        let sig = syncope_sig();
        let laporan = run_protocol(&sig).expect("textual harus didukung");
        assert!(laporan.passed, "{laporan:?}");
        assert_eq!(laporan.inverse, InverseVerdict::Match);
        assert!(laporan.checks.len() >= 3); // positif + negatif-locus + negatif-anchor
    }

    #[test]
    fn protokol_menolak_signature_yang_tak_bisa_memisahkan_dirinya() {
        // Klaim medial, tapi locus menyebut initial — probe positif generator
        // mematuhi ANCHOR, sehingga cek locus kontradiksi terdeteksi.
        let mut sig = syncope_sig();
        sig.locus_id = Some("initial".into());
        let laporan = run_protocol(&sig).expect("textual harus didukung");
        assert!(!laporan.passed, "kontradiksi locus harus digagalkan: {laporan:?}");
    }

    #[test]
    fn domain_entitas_menunggu_llm_bukan_dianggap_geometris() {
        let mut sig = syncope_sig();
        sig.domain_id = "entity".into();
        assert!(matches!(
            generate_deterministic(&sig),
            GenerationOutcome::DeterministicUnsupported { .. }
        ));
    }

    #[test]
    fn prothesis_positif_adjectio_awal() {
        let sig: FigureSignature = serde_json::from_value(serde_json::json!({
            "domain_id": "textual",
            "unit_id": "word",
            "anchor_id": "initial-segment",
            "operation": "addition"
        }))
        .unwrap();
        let GenerationOutcome::Generated(b) = generate_deterministic(&sig) else {
            panic!("harus tergenerasi");
        };
        let p = &b[0];
        assert_eq!((p.before.as_str(), p.after.as_str()), ("a-b-c", "x-a-b-c"));
        assert!(satisfies(&sig, &p.before, &p.after).is_ok());
        assert_eq!(inverse_test(&sig, &p.before, &p.after), InverseVerdict::Match);
    }
}
