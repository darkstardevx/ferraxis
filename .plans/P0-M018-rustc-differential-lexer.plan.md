# Plan: P0-M018 — Stable rustc differential lexer harness skeleton

Status: Complete
Milestone: P0-M018
Created: 2026-09-18

## Goal

Build the first reproducible differential-observation harness for Ferraxis lexing.

The harness will run a small versioned fragment corpus through the current Ferraxis lexer and
through a generated stable-rustc token-tree probe, then record exact compiler identity and classify
the pair of acceptance observations.

## Non-goals

- No change to Ferraxis lexer semantics.
- No attempt to expose or depend on rustc-private lexer APIs.
- No nightly `rustc -Z` dependency.
- No claim of token-for-token rustc lexer equivalence.
- No comparison of token boundaries, spans, token kinds, or diagnostic wording.
- No parser, macro-expansion, name-resolution, type-system, or runtime differential testing.
- No broad lexical conformance claim; P1-M018 remains the full lexer differential suite.

## Context

ADR-0006 requires versioned differential evidence against rustc where meaningful. Stable rustc does
not expose a supported raw lexer-token dump. Rust macro `tt` matching does expose a stable,
documented token-tree acceptance boundary that can answer a narrower question: whether rustc can
tokenize and form token trees from a fragment embedded in a generated probe.

That observation is useful only if Ferraxis records its scope honestly.

This Approved plan must receive a complete green CI checkpoint before any Rust or Cargo harness
implementation is committed. The checkpoint itself is part of P0-M018 evidence.

## Architecture placement

The harness is repository tooling, not compiler frontend architecture.

A new non-publishable workspace tool crate, `tools/ferraxis-diff`, may depend on
`ferraxis-lexer` and `ferraxis-source`. Compiler crates must not depend on the harness.

```text
tests/differential/lexer corpus
        |
        v
tools/ferraxis-diff
   |             |
   v             v
Ferraxis lexer   stable rustc token-tree probe
   |             |
   +------ classification + evidence ------+
```

## Data flow

1. Discover corpus cases deterministically by case ID.
2. Read the exact fragment bytes.
3. Run Ferraxis `lex` on a `SourceFile`.
4. Generate a standalone Rust probe wrapping the fragment in a `macro_rules!` `tt` matcher.
5. Invoke stable `rustc` with edition 2024 and metadata-only output.
6. Capture rustc exit status and stderr.
7. Capture `rustc --version --verbose` once per run.
8. Classify the acceptance pair.
9. Compare classifications with committed expectations.
10. Emit machine-readable TSV and human-readable Markdown evidence under `target/`.

## Invariants

- INV-001 — no rustc-private dependencies.
- INV-013 — disagreements produce preserved conformance evidence.
- INV-015 — compatibility claims require reproducible evidence.
- INV-018 — new compiler behavior requires regression coverage; this milestone changes no compiler
  behavior.
- INV-019 — deterministic compiler-related analysis for identical inputs/configuration.
- INV-021 — current documentation remains CI-validated.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0006 — differential testing against rustc.
- ADR-0011 — toolchain and reproducible validation.
- ADR-0012 — stable rustc token-tree observation boundary.

## Semantic evidence

The harness will exercise existing lexical evidence records:

- SEM-LEX-0001 — initial identifier grammar;
- SEM-LEX-0002 — exact `fn` keyword boundary;
- SEM-LEX-0003 — EOF representation.

No new language-semantic claim is introduced by this milestone. The harness itself produces L4
observations used by future evidence updates.

## Public API / CLI

Initial repository-tool CLI:

```bash
cargo run -p ferraxis-diff --locked -- lexer
```

The command will:

- run the committed P0-M018 lexer corpus;
- fail if an observed classification differs from the committed expectation;
- write evidence to `target/differential/lexer/`;
- print a concise summary and evidence path.

No Ferraxis compiler CLI is changed.

## Compatibility analysis

The rustc probe uses only stable language/compiler behavior:

- stable `macro_rules!` token-tree matching;
- edition 2024;
- ordinary stable rustc command-line options;
- `rustc --version --verbose`.

The rustc observation is explicitly not a raw token-stream oracle. An `agree_reject` result can
include token-tree delimiter constraints and must not be promoted to a pure lexical claim without
stronger evidence.

## Dependency analysis

The tool should use the Rust standard library plus existing workspace crates. No JSON/CLI/tempfile
dependency is needed for the skeleton.

Temporary files should live under a run-specific directory below
`target/differential/lexer/work/`, not in the operating-system global temp directory, so the
evidence path is inspectable and cleanup is deterministic.

## Expected file boundary

- `Cargo.toml`
- `Cargo.lock` only if Cargo legitimately changes it
- `tools/ferraxis-diff/Cargo.toml`
- `tools/ferraxis-diff/src/main.rs`
- `tests/differential/README.md`
- `tests/differential/lexer/README.md`
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/TESTING.md`
- `docs/adr/ADR-0012-stable-rustc-token-tree-observation.md`
- `.github/workflows/ci.yml`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`
- this plan and `docs/MILESTONES.md` only at closure

No files under `crates/ferraxis-lexer/src/` are expected to change.

## Test-first matrix

| Case | Fragment | Ferraxis | rustc token-tree probe | Expected classification |
| --- | --- | --- | --- | --- |
| empty | empty | accept | accept | `agree_accept` |
| fn | `fn` | accept | accept | `agree_accept` |
| fn-main | `fn main` | accept | accept | `agree_accept` |
| fnn | `fnn` | accept | accept | `agree_accept` |
| underscore-main | `_main` | accept | accept | `agree_accept` |
| ascii-digit-tail | `a1` | accept | accept | `agree_accept` |
| bare-underscore | `_` | reject | accept | `ferraxis_rejects` |
| integer | `123` | reject | accept | `ferraxis_rejects` |
| unicode-ident | `é` | reject | accept | `ferraxis_rejects` |
| line-comment | `// comment` | reject | accept | `ferraxis_rejects` |
| unmatched-open-delimiter | `(` | reject | reject | `agree_reject` |

The unmatched-delimiter case is explicitly token-tree evidence, not proof of a raw lexer rejection.

## Implementation sequence

1. Land this Approved plan, ADR-0012, testing-policy update, milestone activation, and current-state
   records as a documentation-only commit.
2. Require the plan-only commit to pass all repository CI jobs.
3. Add the non-publishable `ferraxis-diff` workspace tool.
4. Add corpus fragments and committed expected classifications.
5. Implement deterministic Ferraxis observation.
6. Implement generated stable-rustc token-tree probes.
7. Record exact rustc identity, generated probes, stderr, and results under `target/`.
8. Add unit/integration tests for classification and output determinism where practical.
9. Add a dedicated CI differential job and evidence artifact upload.
10. Run full local gates and exact PR CI.
11. Close the plan only after the final implementation head has exact successful CI evidence.

## Failure modes

- Treating rustc token-tree acceptance as raw lexer-token equivalence.
- Accidentally depending on nightly `-Z` behavior.
- Accidentally depending on rustc-private crates.
- Allowing ordinary parser/type errors in the generated wrapper to contaminate the lexical
  observation.
- Failing to capture exact rustc identity.
- Classifying a mismatch as a Ferraxis bug without consulting semantic authority.
- Writing non-deterministic evidence ordering.
- Letting temporary output outside `target/` dirty the repository.
- Making network access part of the differential test.
- Comparing unstable diagnostic wording as if it were semantic evidence.

## Documentation impact

- ADR-0012 freezes the stable rustc observation boundary.
- `docs/TESTING.md` documents what the harness proves and does not prove.
- Differential corpus README documents case format and classification vocabulary.
- CI documentation will identify the new differential job and artifact.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- dedicated differential harness CI job
- exact GitHub Actions success for the implementation commit

## Acceptance criteria

- [x] Stable rustc raw lexer internals are not required.
- [x] No nightly or rustc-private dependency is introduced.
- [x] Exact `rustc --version --verbose` evidence is recorded.
- [x] Corpus execution is deterministic.
- [x] Both agreement and known Ferraxis-gap classifications are represented.
- [x] Expected classifications are committed and checked.
- [x] Generated probe source and rustc stderr are preserved under `target/`.
- [x] Human-readable and machine-readable result summaries are emitted.
- [x] CI runs the harness on stable Rust.
- [x] CI uploads differential evidence for inspection.
- [x] Existing stable, feature-isolation, MSRV, workflow, and docs gates remain green.
- [x] Exact implementation CI evidence is recorded before closure.

## Completion record

Implementation commit: dc9b792c42e1fe111ea60c88358ec8c2f9d9038a
CI run: <https://github.com/darkstardevx/ferraxis/actions/runs/35415625792>
CI result: success
Completed: 2026-09-18
Notes: 11/11 committed classifications matched; final evidence artifact inspected.

Evidence:

- Approved-plan checkpoint head: `72caa6023b2879f23eaabeb037aa684d793a1f83`.
- Harness implementation commit: `1e65997b1ae80b2deb560a1774c678e7109d9e1c`.
- Final validated head: `dc9b792c42e1fe111ea60c88358ec8c2f9d9038a`.
- Final CI run: <https://github.com/darkstardevx/ferraxis/actions/runs/35415625792>.
- Artifact: `p0-m018-differential-lexer`, GitHub artifact ID `10575304463`.
- Artifact digest:
  `sha256:4bd6cc5a32d9faaf728ae07110fae4aa3cbd917135fc902f9ddfd0c915691f44`.
- CI rustc identity: `rustc 1.98.1 (48a229cea 2026-09-01)`,
  host `x86_64-unknown-linux-gnu`.
- Only the unmatched-open-delimiter probe produced rustc stderr; its delimiter rejection matched
  the committed `agree_reject` expectation and remains token-tree evidence, not a raw lexer claim.
