//! Recompile figure definitions through the CURRENT heuristic compiler and
//! emit SQL UPDATE statements that refresh stale machine-generated geometry
//! in the SARVA database.
//!
//! Contract:
//! - Only rows whose existing geometry is NULL or carries the provenance
//!   marker "kompilasi heuristik" are touched. Manually authored or
//!   user-validated geometry is never clobbered.
//! - Only compiled results with confidence >= 0.75 are emitted.
//! - Output is SQL; nothing touches the database directly.
//!
//! Usage:
//!   sqlite3 sarva_vault.db "SELECT json_object('id',id,'name',name,\
//!       'definition',definition,'geometri',geometri) \
//!       FROM figures WHERE definition IS NOT NULL;" > dump.jsonl
//!   recompile dump.jsonl > refresh.sql
//!   sqlite3 sarva_vault.db < refresh.sql

use figeometrica_core::{compile_definition, ke_json_konvensi_sarva};
use std::io::{BufRead, Write};

const CONFIDENCE_FLOOR: f32 = 0.75;
const PROVENANCE_MARKER: &str = "kompilasi heuristik";

#[derive(serde::Deserialize)]
struct Baris {
    id: i64,
    name: String,
    definition: String,
    geometri: Option<String>,
}

fn dapat_di_regenerate(existing: &Option<String>) -> bool {
    match existing {
        None => true,
        Some(g) => g.contains(PROVENANCE_MARKER),
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("pemakaian: recompile <dump.jsonl>");
    let f = std::fs::File::open(&path).expect("gagal membuka dump");
    let reader = std::io::BufReader::new(f);
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut total = 0usize;
    let mut diperbarui = 0usize;
    let mut dilewati_manual = 0usize;
    let mut gagal_kompilasi = 0usize;

    for line in reader.lines() {
        let line = line.expect("baca baris");
        if line.trim().is_empty() {
            continue;
        }
        // Dump bisa berupa satu objek per baris atau satu array json_group_array.
        let b: Baris = if line.trim_start().starts_with('[') {
            // array dibaca lewat jalur terpisah di bawah
            break;
        } else {
            serde_json::from_str(&line).expect("parse baris json")
        };
        total += 1;

        if !dapat_di_regenerate(&b.geometri) {
            dilewati_manual += 1;
            continue;
        }

        match compile_definition(&b.definition) {
            Some(d) if d.confidence >= CONFIDENCE_FLOOR => {
                diperbarui += 1;
                let json = ke_json_konvensi_sarva(&d.pattern)
                    .replace('\'', "''");
                writeln!(
                    out,
                    "UPDATE figures SET geometri = '{}' WHERE id = {}; -- {} (conf {:.2}, {})",
                    json, b.id, b.name, d.confidence, d.family
                )
                .ok();
            }
            Some(d) => {
                eprintln!(
                    "[rendah] {}: conf {:.2} < {CONFIDENCE_FLOOR} — butuh review",
                    b.name, d.confidence
                );
            }
            None => {
                gagal_kompilasi += 1;
            }
        }
    }

    // Dukung format array (hasil json_group_array).
    let f2 = std::fs::read_to_string(&path).unwrap_or_default();
    if f2.trim_start().starts_with('[') {
        #[derive(serde::Deserialize)]
        struct B2 {
            id: i64,
            name: String,
            definition: String,
            geometri: Option<String>,
        }
        let arr: Vec<B2> = serde_json::from_str(&f2).expect("parse array json");
        for b in arr {
            total += 1;
            if !dapat_di_regenerate(&b.geometri) {
                dilewati_manual += 1;
                continue;
            }
            match compile_definition(&b.definition) {
                Some(d) if d.confidence >= CONFIDENCE_FLOOR => {
                    diperbarui += 1;
                    let json =
                        ke_json_konvensi_sarva(&d.pattern).replace('\'', "''");
                    writeln!(
                        out,
                        "UPDATE figures SET geometri = '{}' WHERE id = {}; -- {} (conf {:.2}, {})",
                        json, b.id, b.name, d.confidence, d.family
                    )
                    .ok();
                }
                _ => {}
            }
        }
    }

    eprintln!(
        "recompile: {total} baris / {diperbarui} SQL / {dilewati_manual} manual-dilindungi / {gagal_kompilasi} tanpa hasil"
    );
}
