# Plan: P1-M003 — Nested block comments

Status: Complete
Milestone: P1-M003
Created: 2026-09-18

## Goal

Complete Rust recursive nesting for ordinary non-documentation block comments in the Ferraxis
lexer.

P1-M003 replaces the temporary P1-M002 nested-comment rejection with an iterative depth-tracking
scanner. A top-level ordinary `/* ... */` comment may contain nested ordinary, outer-doc, or
inner-doc block-comment forms, and the entire outer ordinary comment remains lexical whitespace.

Top-level block documentation comments remain unsupported because their attribute semantics belong
to a separate future milestone.

## Non-goals

- No top-level outer block documentation-comment lowering.
- No top-level inner block documentation-comment lowering.
- No `#[doc]` or `#![doc]` synthesis.
- No line documentation-comment support.
- No parser, macro-expansion, HIR, type-system, or backend changes.
- No punctuation-token support.
- No arbitrary implementation-defined nesting limit.
- No recursive function calls for comment nesting.
- No new dependency.
- No change to the stable rustc token-tree observation boundary from ADR-0012.

## Context

The Rust Reference `BLOCK_COMMENT` grammar is recursive. Nested block comments are supported, and
ordinary block comments may contain any block-comment form through `BLOCK_COMMENT_OR_DOC`.

The Reference examples explicitly show ordinary, outer-doc, and inner-doc block comments nested
inside one another. P1-M002 intentionally stopped at depth one and recorded nested input as a
temporary variance assigned to P1-M003.

P1-M003 closes that variance for ordinary top-level block comments.

Primary references:

- <https://doc.rust-lang.org/reference/comments.html>
- <https://doc.rust-lang.org/reference/notation.html>
- <https://doc.rust-lang.org/reference/input-format.html>

## Architecture placement

This remains a lexer-only change inside `ferraxis-lexer`.

```text
SourceFile UTF-8 bytes
        |
        v
ferraxis-lexer
  ASCII whitespace
  non-doc line comments
  recursive ordinary block comments  <--- P1-M003
  identifiers / fn
  unsupported source
        |
        v
Token stream + EOF
```

No new compiler phase or backend dependency is introduced.

## Data flow

1. Detect a top-level ordinary non-doc `/*` opener using the existing P1-M002 doc-boundary helper.
2. Record the original outer opener offset.
3. Initialize `depth = 1` and advance past the opener.
4. Scan source bytes iteratively.
5. On any nested `/*` sequence, increment depth and advance past both bytes.
6. On any `*/` sequence, decrement depth and advance past both bytes.
7. When depth reaches zero, resume normal tokenization after the outer closing slash.
8. Treat all other bytes as comment-body content, including UTF-8 bytes, `//`, CR, and doc-comment
   marker bytes.
9. If EOF arrives while depth is nonzero, return controlled failure at the original outer opening
   slash.
10. Preserve original source byte offsets for following tokens.
11. Emit exactly one zero-width EOF token at original source byte length.

## Depth and resource behavior

The implementation uses an iterative `usize` depth counter rather than recursive calls.

No arbitrary language-level nesting cap is introduced. Every depth increment consumes a two-byte
`/*` opener from the source, so maximum reachable depth is bounded by source length. The existing
Ferraxis source-length model rejects files that cannot be represented by its `u32` byte-position
space before scanning. Therefore valid depth cannot overflow `usize` on supported targets.

The algorithm is linear in source bytes and constant in auxiliary memory.

## Invariants

- INV-005 — source positions remain byte offsets.
- INV-013 — disagreements require conformance evidence.
- INV-014 — accepted variances remain documented.
- INV-015 — compatibility claims require reproducible evidence.
- INV-016 — user source must not cause a compiler panic.
- INV-017 — invalid Rust produces controlled failure.
- INV-018 — new behavior has regression coverage.
- INV-019 — lexing remains deterministic.
- INV-021 — behavior documentation remains CI-validated.
- INV-022 — milestone and semantic-evidence IDs remain permanent.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0003 — byte-offset source spans.
- ADR-0006 — differential testing against rustc.
- ADR-0011 — toolchain and reproducible validation.
- ADR-0012 — stable rustc token-tree observation boundary.

No new architecture ADR is required. P1-M003 changes only the implementation of an already-owned
lexer responsibility.

## Semantic evidence

Create and maintain:

- `SEM-LEX-0006` — recursive nested block comments.

Primary authority is L1, the Rust Reference comment grammar.

The existing `nested-block-comment` differential case must change from `ferraxis_rejects` to
`agree_accept`. Additional cases cover deeper nesting, mixed block-doc forms inside an ordinary
outer comment, and unterminated nesting.

## Public API / CLI

No token kind or public API shape changes.

Ordinary block comments remain lexical whitespace.

The token-dump CLI should accept recursively nested ordinary comments without emitting comment
tokens.

Unterminated nested comments continue to use the existing structured `LexError` /
`UnexpectedByte` mechanism and report the original outer opening slash.

## Compatibility analysis

Rust supports nested block comments recursively.

For an ordinary outer block comment:

- nested `/* ... */` ordinary comments contribute to depth;
- nested `/** ... */` outer-doc forms contribute to depth;
- nested `/*! ... */` inner-doc forms contribute to depth;
- nested doc forms are comment syntax inside the outer comment and do not become attributes;
- `//` remains ordinary body text;
- `*/` closes only the current nesting level;
- EOF before depth returns to zero is invalid.

Top-level `/** ... */` and `/*! ... */` remain unsupported by Ferraxis because their doc-attribute
semantics are out of scope.

P1-M003 does not introduce a general CRLF-normalization layer.

## Dependency analysis

No dependency is required.

The implementation uses existing byte slices, source spans, diagnostics, and standard-library
integer operations only.

## Expected file boundary

Plan checkpoint:

- `.plans/ACTIVE`
- `.plans/P1-M003-nested-block-comments.plan.md`
- `docs/MILESTONES.md`
- `docs/semantics/README.md`
- `docs/semantics/SEM-LEX-0006-nested-block-comments.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Implementation:

- `crates/ferraxis-lexer/src/lib.rs`
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/semantics/SEM-LEX-0006-nested-block-comments.md`
- `docs/semantics/README.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Closure may additionally update:

- this plan;
- `docs/MILESTONES.md`;
- `.plans/ACTIVE`.

No Cargo manifest or dependency changes are expected.

## Test-first matrix

| Behavior | Test | Expected result |
| --- | --- | --- |
| One nested ordinary comment | `/* a /* b */ c */fn` | comment skipped, then `Fn` |
| Three levels | `/* a /* b /* c */ b */ a */fn` | comment skipped, then `Fn` |
| Nested empty comment | `/* a /**/ b */fn` | comment skipped, then `Fn` |
| Nested triple-star ordinary | `/* a /***/ b */fn` | comment skipped, then `Fn` |
| Nested outer-doc form | `/* a /** docs */ b */fn` | entire outer comment skipped |
| Nested inner-doc form | `/* a /*! docs */ b */fn` | entire outer comment skipped |
| Mixed nested forms | ordinary containing ordinary + outer-doc + inner-doc | entire outer skipped |
| Adjacent openers/closers | nested delimiters with no body text | depth tracked correctly |
| Unicode at multiple depths | UTF-8 in outer and inner bodies | safely skipped |
| Line markers at multiple depths | `//` inside nested comments | body text only |
| CR at multiple depths | bare CR in nested bodies | safely skipped |
| Unterminated outer comment | `/* a /* b */` | controlled failure at outer opening `/` |
| Unterminated deep comment | `/* a /* b /* c */ b */` missing outer close | controlled failure |
| Span after nesting | nested comment followed by `fn` | original byte offset preserved |
| Top-level outer block doc | `/** docs */fn` | remains controlled unsupported failure |
| Top-level inner block doc | `/*! docs */fn` | remains controlled unsupported failure |
| Differential existing nested case | corpus case | changes to `agree_accept` |
| Differential depth-three case | corpus case | `agree_accept` |
| Differential nested outer-doc case | corpus case | `agree_accept` |
| Differential nested inner-doc case | corpus case | `agree_accept` |
| Differential mixed case | corpus case | `agree_accept` |
| Differential unterminated nested case | corpus case | `agree_reject` |

## Implementation sequence

1. Commit this Approved plan, Planned semantic record, milestone activation, and state/handoff
   records without Rust or Cargo changes.
2. Require exact plan-only CI success.
3. Convert P1-M002 nested-rejection tests into nested-acceptance tests.
4. Add deeper, mixed-form, unterminated, span, UTF-8, CR, and line-marker regression tests.
5. Replace depth-one rejection with an iterative depth counter.
6. Count every nested `/*` inside an already-open ordinary comment, regardless of doc form.
7. Decrement depth for every `*/` and stop only when depth reaches zero.
8. Preserve top-level doc-comment rejection.
9. Keep unterminated-comment error anchored at the original outer opener.
10. Update differential corpus expectations and add deeper/mixed cases.
11. Require complete implementation CI success.
12. Inspect the uploaded differential artifact.
13. Record exact implementation CI and artifact identity.
14. Close P1-M003 and remove `.plans/ACTIVE` only after evidence is clean.
15. Require exact closed-state CI success.
16. Merge with history preservation.
17. Require post-merge `main` CI success.

## Failure modes

- Stopping at the first inner `*/` instead of the outer close.
- Counting only ordinary nested comments but not nested doc-comment forms.
- Treating nested doc comments as attributes inside an outer ordinary comment.
- Accidentally accepting top-level block doc comments.
- Recursive function calls causing stack growth on deeply nested input.
- Depth underflow on closing delimiters.
- Depth overflow from an artificial or incorrect counter model.
- Skipping bytes after the outer close.
- Shifting token spans after nested comments.
- Losing controlled failure for unterminated comments.
- Breaking P1-M001 line comments or P1-M002 non-nested cases.
- Calling differential agreement a proof of token-for-token lexer equivalence.

## Documentation impact

- Add `SEM-LEX-0006` as `Planned` before implementation.
- Mark it `Active` only after exact implementation validation.
- Update `SEM-LEX-0005` variance at implementation or closure so recursive nesting is no longer
  listed as unsupported.
- Keep top-level block doc comments explicitly unsupported.
- Update project state and handoff at implementation and closure checkpoints.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- `cargo run -p ferraxis-diff --locked -- lexer`
- exact GitHub Actions success for the implementation commit
- inspection of uploaded differential evidence
- exact GitHub Actions success for the closed-state commit
- successful post-merge `main` CI

## Acceptance criteria

- [x] One-level nested ordinary block comments are supported.
- [x] Multi-level nested ordinary block comments are supported.
- [x] Nested empty and triple-star ordinary comments are supported.
- [x] Nested outer block-doc forms are counted and skipped inside an ordinary outer comment.
- [x] Nested inner block-doc forms are counted and skipped inside an ordinary outer comment.
- [x] Mixed nested block-comment forms are handled correctly.
- [x] No arbitrary nesting-depth cap is introduced.
- [x] Scanner is iterative rather than recursively calling itself.
- [x] Unterminated nested comments fail in a controlled way at the outer opener.
- [x] UTF-8, CR, and line-marker bytes inside nested bodies are safe.
- [x] Token spans after nested comments preserve original byte offsets.
- [x] Top-level outer block doc comments remain unsupported.
- [x] Top-level inner block doc comments remain unsupported.
- [x] Existing line comments remain green.
- [x] Existing non-nested block comments remain green.
- [x] Existing lexer behavior remains green.
- [x] Existing `nested-block-comment` differential case becomes `agree_accept`.
- [x] Added deeper and mixed differential cases classify `agree_accept`.
- [x] Unterminated nested differential case classifies `agree_reject`.
- [x] No dependency is added.
- [x] SEM-LEX-0005 variance is updated.
- [x] SEM-LEX-0006 is current.
- [x] Exact implementation commit passes the complete CI matrix.
- [x] Differential artifact is inspected before closure.
- [x] Exact closed-state commit passes CI.
- [x] `PROJECT_STATE.md` is current.
- [x] `AGENT_HANDOFF.md` is current.

## Completion record

Implementation commit: e3c37407fed0592c1b6f5c71e24ff355d5fa664b
CI run: <https://github.com/darkstardevx/ferraxis/actions/runs/35419741904>
CI result: success
Completed: 2026-09-18
Notes: P1-M003 completed with exact implementation CI and inspected differential evidence.

Evidence:

- Approved plan checkpoint: `5c7cb524c4007e28a997b09d89660d520b51ebe9`.
- Approved plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419633488>.
- Final validated implementation head: `e3c37407fed0592c1b6f5c71e24ff355d5fa664b`.
- Exact successful implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419741904>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10577216286`.
- Artifact digest:
  `sha256:103644d06152b940675780c9bc555061a2b006cf26a0f22197825b92888376c7`.
- CI rustc identity: `rustc 1.98.1 (48a229cea 2026-09-01)`,
  host `x86_64-unknown-linux-gnu`.
- Differential classifications matched: 27/27.
- Existing `nested-block-comment` changed from the P1-M002 `ferraxis_rejects` gap to
  `agree_accept`.
- `nested-block-comment-depth3`, `nested-block-comment-inner-doc`,
  `nested-block-comment-outer-doc`, and `nested-block-comment-mixed` all classified
  `agree_accept`.
- `nested-block-comment-unterminated` classified `agree_reject`.
- Existing bare-underscore, integer-literal, and Unicode-identifier gaps remained unchanged.
- Existing unmatched-delimiter and non-nested unterminated-comment observations remained
  `agree_reject`.
- Stable code, feature isolation, MSRV 1.85.0, repository workflow, differential evidence, and
  documentation validation all passed on the exact implementation head.
