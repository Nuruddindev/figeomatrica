// figeometrica-rhetorica validator — the machine gatekeeper for
// contributions.
//
// For every figure with a compiled geometry:
//   1. examples (`examples`) must exist with at least one positive;
//   2. if the pattern belongs to the matcher's detectable family, EVERY
//      positive example must trigger the figure and EVERY negative must not;
//   3. otherwise the entry is flagged for maintainer review (schema and
//      example presence are still enforced).
//
// Exit code 0 = all checks pass. CI runs this on every PR, so a contributor
// gets objective feedback within minutes.

use figeometrica_core::{GeometryMatcher, TextUnit};
use figeometrica_rhetorica::Rhetorica;

/// Figure names the deterministic matcher can actually produce. Entries
/// outside this family cannot be machine-verified yet and go to human review.
const DETECTABLE: &[&str] = &[
    "anaphora",
    "epistrophe",
    "symploce",
    "anadiplosis",
    "gradatio (climax)",
    "antimetabole (phrase inversion)",
];

fn nama_cocok(entry: &str, finding: &str) -> bool {
    let e = entry.to_lowercase();
    let f = finding.to_lowercase();
    e == f || f.contains(&e) || e.contains(&f)
}

fn terdeteksi(nama: &str) -> bool {
    DETECTABLE.iter().any(|d| nama_cocok(nama, d))
}

fn temuan_untuk(nama: &str, units: &[String]) -> bool {
    let unit_refs: Vec<TextUnit> = units
        .iter()
        .enumerate()
        .map(|(i, u)| TextUnit { chunk_id: Box::leak(format!("u{i}").into_boxed_str()), text: u })
        .collect();
    GeometryMatcher::detect(&unit_refs).iter().any(|f| nama_cocok(nama, &f.figure_name))
}

fn main() -> std::process::ExitCode {
    let base = match Rhetorica::embedded() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let mut errors = 0usize;
    let mut manual = 0usize;
    let mut mesin = 0usize;

    for f in &base.figures {
        if f.geometry.is_none() {
            continue;
        }
        let Some(ex) = &f.examples else {
            eprintln!("[gagal] {}: geometri ada tetapi 'examples' kosong — wajib minimal satu contoh positif", f.name);
            errors += 1;
            continue;
        };
        if ex.positive.is_empty() {
            eprintln!("[gagal] {}: tidak ada contoh positif", f.name);
            errors += 1;
            continue;
        }

        if terdeteksi(&f.name) {
            mesin += 1;
            for ex in &ex.positive {
                if !temuan_untuk(&f.name, ex) {
                    eprintln!("[gagal] {}: contoh positif TIDAK memicu geometri: {:?}", f.name, ex);
                    errors += 1;
                }
            }
            for ex in &ex.negative {
                if temuan_untuk(&f.name, ex) {
                    eprintln!("[gagal] {}: contoh negatif justru MEmicu geometri: {:?}", f.name, ex);
                    errors += 1;
                }
            }
        } else {
            manual += 1;
            println!("[manual] {}: pola di luar keluarga matcher — butuh review maintainer (skema + contoh OK)", f.name);
        }
    }

    let total_geometri = base.geometrized().count();
    println!(
        "{} figur / {} ber-geometri ({} tervalidasi mesin, {} jalur manual), {} galat",
        base.figures.len(),
        total_geometri,
        mesin,
        manual,
        errors
    );

    if errors == 0 {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
