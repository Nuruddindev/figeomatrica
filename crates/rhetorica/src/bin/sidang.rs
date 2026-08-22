//! CONTRACT.md §12 — sidang kontrak atas dataset publik.
//!
//! Mesin gerbang CI: muat manifest knowledge versi kanon (folder vN
//! tertinggi) + dataset tertanam (data/figures/*.json), lalu untuk setiap
//! figur bersignature:
//!   1. slot ∈ vocabulary manifest (fail-closed);
//!   2. bindings: INVALID = gugur, UNKNOWN legal tapi dicatat;
//!   3. protokol witness dijalankan ulang (`run_protocol_auto`);
//!   4. konsistensi tangga: klaim status tanpa bukti = GUGUR
//!      (NO SILENT PROMOTION).
//!
//! Usage:
//!   cargo run -p figeometrica-rhetorica --bin sidang [-- --ci] [-- versi N]
//!
//! Exit 0 = semua lulus. `--ci` mengubah kegagalan jadi exit code 1.

use figeometrica_core::{check_compatibility, run_protocol_auto, BindingVerdict};
use figeometrica_rhetorica::Rhetorica;
use std::path::PathBuf;

/// Manifest knowledge satu versi (subset yang dipakai sidang).
#[derive(serde::Deserialize)]
struct Manifest {
    versi: u32,
    domains: Vec<Slot>,
    units: Vec<Slot>,
    scopes: Vec<Slot>,
    anchors: Vec<Slot>,
    payloads: Vec<Slot>,
    loci: Vec<Slot>,
    bindings: Vec<Binding>,
}

#[derive(serde::Deserialize)]
struct Slot {
    id: String,
}

#[derive(serde::Deserialize)]
struct Binding {
    domain_id: String,
    anchor_id: String,
    operation_id: String,
    payload_id: String,
    status: String,
}

/// Binding store di atas manifest JSON — tanpa database.
struct ManifestBindings<'m> {
    bindings: &'m [Binding],
}

impl figeometrica_core::BindingStore for ManifestBindings<'_> {
    fn lookup(
        &self,
        anchor: &str,
        payload: &str,
        operation: &str,
        domain: &str,
    ) -> BindingVerdict {
        match self
            .bindings
            .iter()
            .find(|b| {
                b.anchor_id == anchor
                    && b.payload_id == payload
                    && b.operation_id == operation
                    && b.domain_id == domain
            })
            .map(|b| b.status.as_str())
        {
            Some("valid") => BindingVerdict::Valid,
            Some("invalid") => BindingVerdict::Invalid,
            _ => BindingVerdict::Unknown,
        }
    }
}

/// Versi kanon = nomor folder vN tertinggi di data/knowledge/.
fn versi_kanon(base: &std::path::Path) -> u32 {
    std::fs::read_dir(base)
        .ok()
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter_map(|e| {
                    let nama = e.file_name().into_string().ok()?;
                    let n: u32 = nama.strip_prefix('v')?.parse().ok()?;
                    Some(n)
                })
                .max()
                .unwrap_or(1)
        })
        .unwrap_or(1)
}

/// Status yang diklaim hanya sah bila buktinya cukup.
fn status_sah(klaim: &str, protokol_lulus: bool, binding: BindingVerdict) -> bool {
    match klaim {
        "EXTRACTED" | "UNDER_SPECIFIED" | "PROSE_ONLY" => true,
        "STRUCTURALLY_VALID" => binding != BindingVerdict::Invalid,
        "WITNESS_TESTED" | "INVERSE_VERIFIED" | "CONTRASTIVE_VERIFIED" | "USER_ACCEPTED"
        | "CANONICAL" => protokol_lulus && binding != BindingVerdict::Invalid,
        // Status samping (AMBIGUOUS/INVALID/...) tidak dinilai di sini.
        _ => true,
    }
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mode_ci = args.iter().any(|a| a == "--ci");
    let paksa_versi: Option<u32> = args
        .iter()
        .position(|a| a == "versi")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok());

    // ── Manifest kanon ───────────────────────────────────────────────
    let kb = PathBuf::from("data").join("knowledge");
    let versi = paksa_versi.unwrap_or_else(|| versi_kanon(&kb));
    let jalur_manifest = kb.join(format!("v{versi}")).join("manifest.json");
    let isi_manifest = match std::fs::read_to_string(&jalur_manifest) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("GAGAL membaca {}: {e}", jalur_manifest.display());
            return std::process::ExitCode::FAILURE;
        }
    };
    let manifest: Manifest = match serde_json::from_str(&isi_manifest) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("GAGAL parse manifest v{versi}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    println!(
        "⚖️  Sidang kontrak — knowledge v{} · {} bindings",
        manifest.versi,
        manifest.bindings.len()
    );

    // ── Dataset tertanam ─────────────────────────────────────────────
    let base = match Rhetorica::embedded() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("GAGAL memuat dataset: {e:?}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let mut diperiksa = 0u32;
    let mut lulus = 0u32;
    let mut n_unknown = 0u32;
    let mut gugur: Vec<String> = Vec::new();

    for f in base.figures.iter() {
        let Some(sig) = &f.signature else { continue };
        diperiksa += 1;
        let nama = &f.name;
        let mut sanggahan: Vec<String> = Vec::new();

        // 1. Slot ∈ vocabulary manifest.
        let wajib = [
            ("domain_id", &manifest.domains, sig.domain_id.clone()),
            ("unit_id", &manifest.units, sig.unit_id.clone()),
            ("anchor_id", &manifest.anchors, sig.anchor_id.clone()),
        ];
        for (label, daftar, nilai) in wajib {
            if !daftar.iter().any(|s| s.id == nilai) {
                sanggahan.push(format!("{label} '{nilai}' bukan anggota manifest v{versi}"));
            }
        }
        let opsional = [
            ("scope_id", &manifest.scopes, &sig.scope_id),
            ("payload_id", &manifest.payloads, &sig.payload_id),
            ("locus_id", &manifest.loci, &sig.locus_id),
        ];
        for (label, daftar, nilai) in opsional {
            if let Some(v) = nilai {
                if !daftar.iter().any(|s| s.id == *v) {
                    sanggahan.push(format!("{label} '{v}' bukan anggota manifest v{versi}"));
                }
            }
        }

        // 2. Bindings (CONTRACT §6).
        let store = ManifestBindings { bindings: &manifest.bindings };
        let verdict = check_compatibility(sig, &store);
        match verdict {
            BindingVerdict::Invalid => sanggahan.push(format!(
                "binding INVALID untuk {}×{}×{}",
                sig.domain_id,
                sig.anchor_id,
                sig.operation.as_str()
            )),
            BindingVerdict::Unknown if sig.payload_id.is_some() => n_unknown += 1,
            _ => {}
        }

        // 3. Protokol witness dijalankan ulang.
        let protokol_lulus = match run_protocol_auto(sig) {
            Ok(lap) => {
                if !lap.passed {
                    sanggahan.push(format!("protokol witness GAGAL (inverse={:?})", lap.inverse));
                }
                lap.passed
            }
            // Di luar jangkauan deterministik: legal, tapi otomatis
            // menggagalkan klaim WITNESS_TESTED ke atas lewat langkah 4.
            Err(_) => false,
        };

        // 4. Konsistensi tangga (NO SILENT PROMOTION).
        let klaim = f.epistemic.as_ref().map(|e| e.status.as_str()).unwrap_or("PROSE_ONLY");
        if !status_sah(klaim, protokol_lulus, verdict) {
            sanggahan.push(format!("status '{klaim}' tak didukung bukti"));
        }

        if sanggahan.is_empty() {
            lulus += 1;
        } else {
            for s in sanggahan {
                gugur.push(format!("{nama}: {s}"));
            }
        }
    }

    // ── Laporan ──────────────────────────────────────────────────────
    println!(
        "figur bersignature: {diperiksa} · lulus penuh: {lulus} · binding-UNKNOWN dicatat: {n_unknown}"
    );
    if gugur.is_empty() {
        println!("✅ tidak ada yang gugur");
        std::process::ExitCode::SUCCESS
    } else {
        println!("\n❌ GUGUR ({}):", gugur.len());
        for g in &gugur {
            println!("  · {g}");
        }
        if mode_ci {
            std::process::ExitCode::FAILURE
        } else {
            std::process::ExitCode::SUCCESS
        }
    }
}
