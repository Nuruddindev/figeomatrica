//! Sync geometry from a SARVA database dump into the per-figure dataset.
//!
//! SARVA remains the source of truth for definitions + compiled geometry;
//! this repo owns examples (contoh) and attribution. The merge keeps both:
//! `geometri` comes from the dump, everything already present in
//! `data/figures/` stays untouched.
//!
//! Usage:
//!   1. Export the dump from your SARVA vault:
//!      sqlite3 -readonly "file:$HOME/.local/share/sarva/sarva_vault.db?mode=ro" \
//!        "SELECT json_group_array(json_object('id',id,'name',name,'geometri',geometri)) \
//!         FROM figures WHERE definition IS NOT NULL;" > /tmp/sarva_dump.json
//!   2. Run: cargo run -p figeometrica-rhetorica --bin sinkron -- /tmp/sarva_dump.json
//!   3. Review with: git diff && cargo run -p figeometrica-rhetorica --bin validate

use std::fs;
use std::path::{Path, PathBuf};

fn slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(serde::Deserialize)]
struct DumpEntry {
    name: String,
    geometri: Option<serde_json::Value>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(dump_path) = args.get(1) else {
        eprintln!("Pemakaian: sinkron <sarva_dump.json>");
        eprintln!("Lihat komentar header file ini untuk cara membuat dump.");
        std::process::exit(2);
    };

    let dump_raw = fs::read_to_string(dump_path).unwrap_or_else(|e| {
        eprintln!("Gagal membaca {dump_path}: {e}");
        std::process::exit(1);
    });
    let entries: Vec<DumpEntry> = serde_json::from_str(&dump_raw).unwrap_or_else(|e| {
        eprintln!("Dump bukan array yang valid: {e}");
        std::process::exit(1);
    });

    let dataset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/figures");

    let mut diperbarui = 0usize;
    let mut dilewati = 0usize;
    let mut tanpa_berkas = 0usize;

    for e in &entries {
        // Only sync figures that actually carry geometry in SARVA.
        let Some(geo) = e.geometri.as_ref().filter(|g| !g.is_null()) else {
            dilewati += 1;
            continue;
        };
        let path = dataset_dir.join(format!("{}.json", slug(&e.name)));
        if !path.exists() {
            tanpa_berkas += 1;
            continue;
        }
        if terapkan(&path, geo) {
            println!("↺ {}: geometri disalin dari SARVA", e.name);
            diperbarui += 1;
        } else {
            dilewati += 1;
        }
    }

    println!(
        "{diperbarui} berkas diperbarui, {dilewati} sudah sama/dilewati, {tanpa_berkas} figur tanpa berkas dataset."
    );
    if diperbarui > 0 {
        println!("Selanjutnya: git diff data/figures && cargo run -p figeometrica-rhetorica --bin validate");
    }
}

/// Overwrite only the `geometri` field when it actually differs.
fn terapkan(path: &Path, geo_baru: &serde_json::Value) -> bool {
    let raw = fs::read_to_string(path).expect("dataset file readable");
    let mut v: serde_json::Value = serde_json::from_str(&raw).expect("valid dataset JSON");
    if v.get("geometri") == Some(geo_baru) {
        return false;
    }
    v["geometri"] = geo_baru.clone();
    fs::write(
        path,
        serde_json::to_string_pretty(&v).expect("serialize") + "\n",
    )
    .expect("write dataset file");
    true
}
