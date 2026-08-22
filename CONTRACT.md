# Figeometrica Contract v1

**Normative specification.** This document defines the concepts, terms, and
rules that every implementation (Rust core, SARVA knowledge layer, Rhetoric
Lab UI) must obey. Implementation is subordinate to this contract — never
the reverse.

Term freeze: the vocabulary in §2 is frozen. Renaming these concepts
requires a contract version bump and a written rationale, because silent
renaming lets code and database drift into theory-making without anyone
noticing (the original disease this project set out to cure).

---

## 0. Core Principle

> **A rhetorical definition is not operational-geometric merely because an
> LLM can generate a plausible example from it. It is operational-geometric
> only when its signature can generate a valid witness and the witness can
> be inversely reconstructed into the same signature, subject to its
> contrastive boundary conditions.**

Everything below exists to make this sentence executable.

### 0.1 NO SILENT PROMOTION

Epistemic status may only rise through its designated evidence gate, and
every transition — including downgrades and rejections — must be recorded
with reason and provenance:

```text
PROSE_ONLY          →(extraction)          → EXTRACTED
EXTRACTED           →(structural check)    → STRUCTURALLY_VALID
STRUCTURALLY_VALID  →(witness battery)     → WITNESS_TESTED
WITNESS_TESTED      →(inverse test)        → INVERSE_VERIFIED
INVERSE_VERIFIED    →(contrastive family)  → CONTRASTIVE_VERIFIED
CONTRASTIVE_VERIFIED→(human judgment)      → USER_ACCEPTED
USER_ACCEPTED       →(canonicalization)    → CANONICAL
```

There is no path from PROSE_ONLY to CANONICAL. There is no boolean
rejection: every `INVALID`, `COLLISION`, `CONFLICTING`, `AMBIGUOUS`,
`UNDER_SPECIFIED` verdict carries a reason string and provenance.

### 0.2 Division of epistemic labor

- **Engine**: checks structure; runs protocols; never judges truth.
- **Knowledge base**: provides ontology (units, anchors, payloads, loci)
  and compatibility bindings; grows only through validated insertion.
- **LLM**: extracts, generates witnesses, proposes bindings and
  contrastive candidates. The LLM is **never the judge**.
- **User**: authorizes knowledge. Only human judgment promotes past
  CONTRASTIVE_VERIFIED.

---

## 1. Doctrine: closed algebra, open knowledge

The algebra of operations is **closed** (§5). Everything else — units,
scopes, anchors, payloads, loci, bindings — is **open knowledge** stored in
database tables with `provenance` and `status` columns. A value enters the
rhetorical ontology by INSERTION, not by recompiling Rust.

## 2. Geometric Signature (frozen fields)

A figure's machine-operational definition is its signature:

```text
unit        — what kind of thing is transformed
scope       — carrier structure hosting the unit
anchor      — what the operation binds to
operation   — which closed primitive acts
payload     — what material the operation carries
locus       — where the change lands (position or relation)
result      — declared outcome label
constraints — optional boundary conditions
```

`Figure`, `FigureSignature`, and `VerificationRecord` are three distinct
entities. A figure has a name and rhetorical identity; a signature is its
geometric claim; a verification record is the evidence that the claim was
tested. None of them subsumes another.

The prose definition remains as the human-readable description. The
signature is the machine-operational definition. Both coexist; neither
replaces the other.

### 2.1 Unit / Scope / Locus

| Axis | Answers | Test question |
|---|---|---|
| unit | ontological category of the object transformed | "what kind of thing?" |
| scope | structural carrier in which units live | "housed by what structure?" |
| locus | address of change within/among carriers | "where does it land?" |

Examples:

```text
aphaeresis:     unit=word     scope=phonological-form   locus=initial-segment
prosopopoeia:   unit=entity   scope=representation      locus=entity
procatalepsis:  unit=argument scope=discourse           locus=before-objection
synaloepha:     unit=word     scope=phonological-form   locus=cross-unit-boundary
```

**Scope is not a junk drawer.** If a piece of information is answerable by
unit or by locus, it MUST NOT be placed in scope.

Locus may be positional (`initial`, `medial`, `final`) or relational
(`inter-sentence`, `argumentative-relation`, `entity-representation`). For
entity-level figures locus is often degenerate (`locus=entity`) — allowed,
as payload degeneracy is allowed at grapheme level.

### 2.2 Anchor

What the operation binds to. Two legal flavors:

- **positional** for textual domains (initial/final/medial segment,
  clause boundary);
- **ontological** for entity domains (non-person, non-human, person,
  character, thesis).

Anchor is open vocabulary. Positional anchors do not exhaust it.

### 2.3 Payload

The material carried by the operation. `adjectio` is transitive; payload is
its object.

- At grapheme level the payload is degenerate (letters/syllables) and MAY
  be left implicit.
- At entity/argument level the payload is frequently THE discriminator:
  personification = addition(human-attribute), prosopopoeia =
  addition(person), ethopoeia = addition(characterological-attribute).

Payload vocabulary is open, per-domain, and may later form a taxonomy;
v1 keeps it flat.

## 3. Result

Stored, human-readable label of the transformation outcome
(`personated entity`, `shortened word`). Declared as part of the
definition's identity. Engines MUST NOT attempt to compute it; there is no
result calculus in v1.

## 4. Constraints (replaces "configuration")

Optional collection; absent slots are simply absent:

```text
min_occurrences, max_occurrences,
ordering, adjacency, dependency,
exclusions, required_relation
```

No separate `configuration` axis exists.

## 5. Operations (closed algebra)

Exactly six primitives, defined in Rust, referenced elsewhere by id:

```text
ADJECTIO      addition of payload
DETRACTIO     removal of payload
IMMUTATIO     substitution of payload
TRANSMUTATIO  permutation/transposition of payload
REPETITIO     recurrence of payload
ORDERING      arrangement relative to external reference (derived)
```

Operation does NOT determine domain: `adjectio × grapheme`,
`adjectio × entity`, `adjectio × argument` are all well-formed. The same
algebra instantiates across unit-spaces; that is the point of Figeometrica.

## 6. Type Compatibility (knowledge-defined)

Compatibility between anchor, payload, and operation lives in the
knowledge layer, never in Rust match arms:

```text
bindings(anchor_id, payload_id, operation_id, domain_id,
         provenance, status ∈ {valid, unknown, invalid})
```

Type check returns three outcomes:

- **VALID** — binding recorded valid.
- **UNKNOWN / UNVERIFIED** — no binding record. Not an error; the system
  stays epistemically open and flags the binding as candidate knowledge.
- **INVALID** — binding recorded invalid, with reason + provenance.

Example: `addition(person)` on anchor `non-person` → VALID.
`addition(characterological-attribute)` on `non-person` → UNKNOWN until a
binding with a basis exists.

## 7. Epistemic Status Ladder

Canonical ladder (see §0.1 for gates):

```text
PROSE_ONLY
EXTRACTED
STRUCTURALLY_VALID
WITNESS_TESTED
INVERSE_VERIFIED
CONTRASTIVE_VERIFIED
USER_ACCEPTED
CANONICAL
```

Side states (each requires reason + provenance):

```text
UNDER_SPECIFIED — extraction could not fill required slots
INVALID         — failed structural check or binding marked invalid
CONFLICTING     — two extractions/signatures disagree
AMBIGUOUS       — one prose yields multiple irreconcilable signatures
COLLISION       — inseparable from another figure (see §9)
```

UI color rule: green is reserved for USER_ACCEPTED and CANONICAL.
Nothing else may render green.

### 7.1 Legacy migration policy (non-destructive)

Existing geometry labels are preserved, never deleted:

- old state moves to `legacy_status`;
- new `epistemic_status` starts at PROSE_ONLY and climbs independently;
- `legacy_green ≠ verified_green`: the UI stops interpreting legacy green
  as evidence of geometric validity.

The message is not "old figures were wrong" but "old status is no longer
accepted as proof".

## 8. Witness Protocol

Given a signature S, the engine produces a witness battery. Deterministic
generation first (grapheme/syllable/word domains are algorithmically
constructible); LLM enters only as **witness constructor** for higher
domains (entity/character/concept/argument/discourse), followed by
structural verification. Never: "LLM says correct → green".

```text
deterministic generation
        ↓
protocol validation
        ↓
LLM construction when necessary
        ↓
structural verification
        ↓
human judgment
```

### 8.1 Positive witness

An instance satisfying every slot of S.

### 8.2 Negative witnesses (guided, non-combinatorial)

Remove exactly one component, in informativeness order:

1. Negative-Payload (usually the sharpest boundary probe)
2. Negative-Locus
3. Negative-Anchor

Maximum three per figure; the engine selects, it does not enumerate all
combinations. A good negative witness must FAIL the signature check.

### 8.3 Contrastive witness

Built against the contrastive family (§9). The system asks whether an
instance generated from S(A) is rejected by S(B) and vice versa. This
tests discriminability, not plausibility.

### 8.4 Inverse test

From the positive witness alone, reconstruct a signature S′ (extraction
without access to the figure name). If S′ ≠ S, the definition fails the
inverse gate regardless of how plausible its examples look. Round-trip:
signature → witness → reconstruction → signature.

## 9. Separability

Official concept. For figures A, B with witness sets W(A), W(B):

- `W(A) ≠ W(B)` → A and B are candidates for distinct geometric figures;
- `W(A) = W(B)` → `A ≡ B` geometrically: either true aliases or
  under-specified definitions. Recorded as COLLISION, never silently
  merged nor silently kept apart.

Contrastive families are computed, not hand-maintained. Candidate query:

```text
same domain + same operation + overlapping unit/scope/locus
```

Members are then subjected to mutual witness rejection testing. Persistent
mutual acceptance raises COLLISION with reason and provenance.

## 10. Pipeline (Rhetoric Lab)

```text
                 Natural Definition
                       │
                       ▼
                 Signature Extractor
                       │
                       ▼
                  Type Checker ──── bindings table
                       │
                       ▼
                Witness Generator
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       positive     negative     contrastive
          └────────────┼────────────┘
                       ▼
                   Inverse Test
                       ▼
                 Human Judgment
                       ▼
              Figeometrica Knowledge
```

Rhetoric Lab is therefore not a definition editor. It is a laboratory for
testing whether a definition is an executable geometric transformation.

## 11. Success criterion

454 figures is not the target. The target is proof that the pipeline can:
take a definition → extract a signature → run the transformation →
generate witnesses → invert the witness back into the signature →
separate the figure from its neighbors.

Sixty CANONICAL figures that are truly executable and verified outweigh
454 green-labeled ones that are not.
