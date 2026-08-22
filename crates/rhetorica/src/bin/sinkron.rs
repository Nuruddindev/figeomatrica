//! Sync geometry from a SARVA database dump into the per-figure dataset.
//!
//! SARVA remains the source of truth for definitions + compiled geometry;
//! this repo owns examples and attribution. The dump carries the legacy
//! Indonesian DB convention (jangkar/kelas/satuan/operasi/minim_ulangan);
//! this tool translates it into the public English schema (`geometry` with
//! anchor/class/grain/operation/min_repeats) before writing.
//!
//! Usage:
//!   1. Export the dump from your SARVA vault:
//!      sqlite3 -readonly "file:$HOME/.local/share/sarva/sarva_vault.db?mode=ro" \
//!        "SELECT json_group_array(json_object('id',id,'name',name,\
//!         'definition',definition,'geometri',geometri)) \
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
    #[serde(default)]
    definition: Option<String>,
    geometri: Option<serde_json::Value>,
}

/// Legacy Indonesian value → public English value.
const ANCHOR: &[(&str, &str)] = &[
    ("Awal", "Initial"),
    ("Akhir", "Final"),
    ("Sisipan", "Insertion"),
    ("UnitUtuh", "WholeUnit"),
    ("AntarUnit", "CrossUnit"),
];
const CLASS: &[(&str, &str)] = &[
    ("Leksikal", "Lexical"),
    ("Akar", "Root"),
    ("Gramatikal", "Grammatical"),
    ("Konseptual", "Conceptual"),
];
const GRAIN: &[(&str, &str)] = &[
    ("grafem", "grapheme"),
    ("kata", "word"),
    ("frasa", "phrase"),
    ("unit", "unit"),
    ("wacana", "discourse"),
];
const OPERATION: &[(&str, &str)] = &[
    ("adjectio", "addition"),
    ("detractio", "deletion"),
    ("immutatio", "substitution"),
    ("transmutatio", "permutation"),
    ("repetitio", "repetition"),
    ("ordering", "ordering"),
];

fn en(value: &str, table: &[(&str, &str)]) -> String {
    table
        .iter()
        .find(|(legacy, _)| *legacy == value)
        .map(|(_, public)| public.to_string())
        .unwrap_or_else(|| value.to_string())
}

/// Translate a geometry object into the public English schema. Accepts the
/// value either as a real object or as a string containing JSON (the SQLite
/// dump wraps TEXT columns as strings). Returns `None` when required fields
/// are missing/malformed.
fn terjemahkan(geo: &serde_json::Value) -> Option<serde_json::Value> {
    let geo = match geo {
        serde_json::Value::String(s) => serde_json::from_str::<serde_json::Value>(s).ok()?,
        v => v.clone(),
    };
    let o = geo.as_object()?;
    fn as_str(v: Option<&serde_json::Value>) -> Option<&str> {
        v?.as_str()
    }
    let pick = |en_key: &str, id_key: &str| -> Option<&serde_json::Value> {
        o.get(en_key).or_else(|| o.get(id_key))
    };

    let mut out = serde_json::Map::new();
    out.insert(
        "anchor".into(),
        serde_json::Value::String(en(as_str(pick("anchor", "jangkar"))?, ANCHOR)),
    );
    out.insert(
        "class".into(),
        serde_json::Value::String(en(as_str(pick("class", "kelas"))?, CLASS)),
    );
    if let Some(s) = as_str(pick("grain", "satuan")) {
        out.insert("grain".into(), serde_json::Value::String(en(s, GRAIN)));
    }
    if let Some(s) = as_str(pick("operation", "operasi")) {
        out.insert("operation".into(), serde_json::Value::String(en(s, OPERATION)));
    }
    if let Some(n) = pick("min_repeats", "minim_ulangan").and_then(|v| v.as_u64()) {
        out.insert("min_repeats".into(), serde_json::Value::from(n));
    }
    if let Some(t) = pick("template", "template") {
        out.insert("template".into(), t.clone());
    }
    if let Some(arr) = pick("transforms", "transformasi").and_then(|v| v.as_array()) {
        let mapped: Vec<serde_json::Value> = arr
            .iter()
            .filter_map(|x| {
                let o = x.as_object()?;
                let axis = o.get("axis").or_else(|| o.get("sumbu"))?.as_str()?;
                let dir = o.get("direction").or_else(|| o.get("arah"))?.as_str()?;
                let direction = match dir {
                    "up" | "naik" => "up",
                    "down" | "turun" => "down",
                    _ => "neutral",
                };
                Some(serde_json::json!({ "axis": axis, "direction": direction }))
            })
            .collect();
        if !mapped.is_empty() {
            out.insert("transforms".into(), serde_json::Value::Array(mapped));
        }
    }
    if let Some(l) = as_str(pick("locus", "locus")) {
        let v = match l {
            "setiap" => "every",
            "respons" => "response",
            "ujung" => "terminal",
            "tersebar" => "distributed",
            "berumpun" => "clustered",
            "tengah" => "medial",
            other => other,
        };
        out.insert("locus".into(), serde_json::Value::String(v.into()));
    }
    if let Some(s) = as_str(pick("note", "catatan")) {
        out.insert("note".into(), serde_json::Value::String(s.into()));
    }
    Some(serde_json::Value::Object(out))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(dump_path) = args.get(1) else {
        eprintln!("Usage: sinkron <sarva_dump.json>");
        eprintln!("See the header comment of this file for how to create the dump.");
        std::process::exit(2);
    };

    let dump_raw = fs::read_to_string(dump_path).unwrap_or_else(|e| {
        eprintln!("Cannot read {dump_path}: {e}");
        std::process::exit(1);
    });
    let entries: Vec<DumpEntry> = serde_json::from_str(&dump_raw).unwrap_or_else(|e| {
        eprintln!("Dump is not a valid array: {e}");
        std::process::exit(1);
    });

    // Dataset lives at the repository root (same as build.rs reads).
    let dataset_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/figures");

    let mut updated = 0usize;
    let mut skipped = 0usize;
    let mut no_file = 0usize;

    for e in &entries {
        let path = dataset_dir.join(format!("{}.json", slug(&e.name)));
        if !path.exists() {
            no_file += 1;
            continue;
        }
        // Geometry is optional; a definition alone still syncs.
        let translated = match e.geometri.as_ref().filter(|g| !g.is_null()) {
            Some(geo) => match terjemahkan(geo) {
                Some(t) => Some(t),
                None => {
                    eprintln!("⚠ {}: geometri tidak bisa diterjemahkan, dilewati", e.name);
                    None
                }
            },
            None => None,
        };
        if translated.is_none() && e.definition.is_none() {
            skipped += 1;
            continue;
        }
        if apply(&path, translated.as_ref().unwrap_or(&serde_json::Value::Null), e.definition.as_deref()) {
            println!("↺ {}: geometry/definition copied from SARVA", e.name);
            updated += 1;
        } else {
            skipped += 1;
        }
    }

    println!("{updated} files updated, {skipped} already equal/skipped, {no_file} figures without a dataset file.");
    if updated > 0 {
        println!("Next: git diff data/figures && cargo run -p figeometrica-rhetorica --bin validate");
    }
}

/// Overwrite the `geometry` and `definition` fields when they differ.
fn apply(
    path: &Path,
    geometry_new: &serde_json::Value,
    definition_new: Option<&str>,
) -> bool {
    let raw = fs::read_to_string(path).expect("dataset file readable");
    let mut v: serde_json::Value = serde_json::from_str(&raw).expect("valid dataset JSON");
    let geo_equal = match geometry_new {
        serde_json::Value::Null => true, // no geometry upstream — leave untouched
        g => v.get("geometry").map(|x| x == g).unwrap_or(false),
    };
    let def_equal = match (v.get("definition"), definition_new) {
        (Some(a), Some(b)) => a.as_str() == Some(b),
        (None, None) => true,
        _ => false,
    };
    if geo_equal && def_equal {
        return false;
    }
    if !geo_equal {
        v["geometry"] = geometry_new.clone();
    }
    if let Some(d) = definition_new {
        v["definition"] = serde_json::Value::String(d.to_string());
    }
    // Drop the legacy Indonesian key so the entry never carries both
    // (serde treats field+alias as one target → duplicate-field error).
    if let Some(obj) = v.as_object_mut() {
        obj.remove("geometri");
        obj.retain(|_, val| !val.is_null());
    }
    fs::write(
        path,
        serde_json::to_string_pretty(&v).expect("serialize") + "\n",
    )
    .expect("write dataset file");
    true
}
