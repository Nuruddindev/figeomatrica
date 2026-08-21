# Figeometrica

**Figures are geometric.** Every rhetorical figure — if well defined — is an
operation over a sequence: *what operation, at which anchor, on which grain,
repeated how many times*. Figeometrica turns that thesis into executable
infrastructure.

It is a theory-compilation framework: humanistic theories of style (starting
with classical rhetoric — the world's oldest text-analysis taxonomy) compiled
into structured, machine-checkable specifications, plus engines that execute
them deterministically and auditably.

```
figure = OPERATION × ANCHOR × GRAIN × REPETITION
         (adjectio | detractio | immutatio | transmutatio | repetitio)
         × (initial | final | insertion | whole-unit | cross-unit)
         × (grapheme | word | phrase | unit | discourse)
```

Example — `tmesis` ("abso-bloody-lutely"), as stored in
[`data/figures/tmesis.json`](data/figures/tmesis.json):

```json
{ "jangkar": "Sisipan", "kelas": "Leksikal", "satuan": "kata",
  "operasi": "adjectio", "minim_ulangan": 1, "template": [] }
```

## Crates

| Crate | What it is |
|---|---|
| [`figeometrica-core`](crates/core) | Geometry spec format (`FigurePattern`, `Anchor`, `ElementClass`, slot templates with equality classes) + deterministic matcher (`GeometryMatcher`) |
| [`figeometrica-pipeline`](crates/pipeline) | Provenance-anchored analysis pipeline: modality-aware chunks, LLM observation/verification stage traits, findings with chunk+span evidence |
| [`figeometrica-rhetorica`](crates/rhetorica) | The classical-rhetoric theory base as data: figures, geometric specs, categories, loader |

## Design principles

1. **Ontology as data, not prose** — definitions compile to formal specs;
   unmet criteria are computable, so negative evidence is real.
2. **Deterministic where possible, LLM where necessary** — geometry matching
   never calls a model; models observe features and verify semantics, always
   with confidence and `indeterminate` states.
3. **Provenance everywhere** — every finding carries `chunk_id + span`.
4. **Falsifiable catalog** — a definition that cannot be written in canonical
   form is a bad definition, not a non-geometric figure.

## Status

Early development. Core matcher covers 9 patterns (anaphora, epistrophe,
symploce, anadiplosis, gradatio/climax, antimetabole, chiasmus, tmesis,
parenthesis); the rhetoric theory base is being geometrized incrementally.

## Participate

447 of 456 figures still need their geometry compiled — and the machine
checks your work: every contribution ships with example sentences that CI
runs through the deterministic matcher. No code required; one JSON file is
enough.

**How to contribute (± 15 minutes for your first figure):**

1. **Claim a figure** — open an issue with the
   ["Geometrize a figure"](../../issues/new?template=geometrize-figure.md)
   template, or pick any file in [`data/figures/`](data/figures) whose
   `"geometri"` is `null` (e.g. `epizeuxis.json`).
2. **Fill the canonical form** — `jangkar`, `kelas`, `satuan`, `operasi`,
   `minim_ulangan`, plus a slot `template` if the figure has one. The field
   reference is in [CONTRIBUTING.md](CONTRIBUTING.md).
3. **Add examples** — positive sentences that *must* trigger the pattern,
   near-miss negatives that *must not*. This is what makes your entry
   machine-checkable.
4. **Check locally** — `cargo run -p figeometrica-rhetorica --bin validate`
5. **Open a PR** — CI verifies automatically: pass = merged with your name
   in the entry's `atribusi`; fail = you get the exact failing example.

Patterns outside the matcher's current family (conceptual-class figures like
chiasmus, insertions like tmesis) are welcome too — they route to maintainer
review instead of automatic verification.

## License

MIT — see [LICENSE-MIT](LICENSE-MIT).
