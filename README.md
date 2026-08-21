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

Example — `tmesis` ("abso-bloody-lutely"):

```json
{ "anchor": "Insertion", "class": "Lexical", "grain": "word",
  "operation": "adjectio", "min_repeats": 1, "template": [] }
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
runs through the deterministic matcher. Pick a figure, fill one JSON file,
open a PR. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE-MIT](LICENSE-MIT).
