// build.rs — merge per-figure files from data/figures/*.json into the
// embedded theory base. This is the contributor-facing layout: one figure,
// one file, one PR. Uniqueness of id/name is enforced here so bad merges
// fail the build, not production.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../../data/figures");

    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = manifest.join("../../data/figures");
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();

    let mut seen_ids: HashSet<u64> = HashSet::new();
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut figures: Vec<serde_json::Value> = Vec::new();

    for path in paths {
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let value: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", path.display()));

        let id = value["id"].as_u64().unwrap_or_else(|| panic!("missing numeric id in {}", path.display()));
        let name = value["name"].as_str().unwrap_or_else(|| panic!("missing name in {}", path.display())).to_string();
        if !seen_ids.insert(id) {
            panic!("duplicate figure id {id} ({})", path.display());
        }
        if !seen_names.insert(name.clone()) {
            panic!("duplicate figure name \"{name}\" ({})", path.display());
        }
        figures.push(value);
    }

    figures.sort_by_key(|v| v["id"].as_u64().unwrap_or(0));
    let merged = serde_json::json!({
        "theory": "classical-rhetoric",
        "version": 1,
        "figures": figures,
    });

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("figures.json"), serde_json::to_string_pretty(&merged).unwrap()).unwrap();
    println!("cargo:warning=figeometrica-rhetorica: merged {} figures", figures.len());
}
