*English version · [Versi Indonesia](CONTRIBUTING.id.md)*

# Contributing to Figeometrica

Thank you! This guide explains how to contribute a **figure geometry** —
a structured specification that makes a rhetorical figure machine-detectable.
You don't need to write code; a single JSON file plus example sentences is
enough.

## Thesis in one line

> Every figure, when properly defined, is an operation over a series:
> **operation × anchor × unit × repetition**.

## Canonical form

| Field | Content | Valid values |
|---|---|---|
| `jangkar` (anchor) | where the pattern attaches | `Awal` (Start), `Akhir` (End), `Sisipan` (Insertion), `UnitUtuh` (WholeUnit), `AntarUnit` (BetweenUnits) |
| `kelas` (class) | equivalence class of elements | `Leksikal` (Lexical), `Akar` (Root), `Gramatikal` (Grammatical), `Konseptual` (Conceptual) |
| `satuan` (unit) | unit of the operated element | `grafem` (grapheme), `kata` (word), `frasa` (phrase), `unit`, `wacana` (discourse) |
| `operasi` (operation) | classical operation (operae) | `adjectio`, `detractio`, `immutatio`, `transmutatio`, `repetitio` |
| `minim_ulangan` (min. repetitions) | minimum repetition count of the pattern | integer ≥ 1 |
| `template` | pattern slots (optional) | `["A","*","A"]` — same id = same label, `*` = anything |
| `catatan` (notes) | brief explanation (optional) | free text |

Note: field names and enum values stay in Indonesian in the actual JSON
(`jangkar`, `kelas`, `satuan`, etc.) — this table only translates their
meaning for English-speaking contributors. Use the Indonesian keys exactly
as shown when writing your entry.

Full example entry (`data/figures/anaphora.json`):

```json
{
  "id": 59,
  "name": "anaphora",
  "categories": ["of Repetition"],
  "geometri": {
    "jangkar": "Awal", "kelas": "Leksikal", "satuan": "unit",
    "operasi": "repetitio", "minim_ulangan": 2,
    "template": ["A", "*", "A"]
  },
  "contoh": {
    "positif": [["We came.", "We saw.", "We conquered."]],
    "negatif": [["He came.", "They saw.", "It ended."]]
  },
  "atribusi": { "geometri": "your-username", "contoh": "your-username", "lisensi": "MIT" }
}
```

## Example rules (the most important part)

- **Positive** examples MUST trigger the pattern; **negative** examples
  MUST NOT trigger it (similar-looking, but not the figure itself).
- Each example is an array of discourse units (sentences). Cross-unit
  figures (anaphora, anadiplosis, climax) need multiple units; antimetabole
  only needs one.
- The validator runs a deterministic matcher against your examples. Passing
  means your contribution is machine-validated; failing means CI tells you
  exactly which example is problematic.
- Patterns outside the current matcher family (e.g. Conceptual-class figures
  like chiasmus, or insertion figures like tmesis/parenthesis) are still
  accepted — CI flags them for a *manual review* path by maintainers.

## Workflow

1. Open an issue using the **"Geometrize a figure"** template, and claim one figure.
2. Fork → branch → edit **a single file** `data/figures/<name>.json`.
3. Run locally: `cargo run -p figeometrica-rhetorica --bin validate`
4. Push and open a PR. CI verifies automatically.

## License & attribution

- Contributions are licensed under **MIT** as soon as the PR is opened (inbound = outbound).
- Your name is stored in the entry's `atribusi` field and in CONTRIBUTORS.md.
- **Do not copy prose definitions** from copyrighted sources (Silva
  Rhetoricae, etc.). The classical taxonomic structure is public domain;
  other people's writing is not.

## Dataset paper co-authorship

Contributors with **≥ 10 accepted entries**, or who serve as a validator,
are added to the co-author list of the dataset publication. Final criteria
will be announced before the paper is written and are not retroactive.
