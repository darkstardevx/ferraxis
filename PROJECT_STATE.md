# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

No implementation milestone is currently active and `.plans/ACTIVE` is intentionally absent.

## Recently completed milestone

- `P1-M001` — Line comments.
- Final validated implementation head: `b4b8e10f4a95dfde6837e0e8d605746eab695317`.
- Exact implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35417098551>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10575823857`.
- Result: 14/14 committed differential classifications matched.

## Earlier completed foundation

- `P0-M018` — Differential lexer harness skeleton.
- Main merge: `694ba6a11ccf7c7136aa7649e9edb0dcbd970caf`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35415828585>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, explicit EOF tokens, deterministic token dumping, and a versioned differential
lexer-observation harness.

Ordinary Rust non-doc `LINE_COMMENT` input is treated as whitespace and terminates at LF or EOF.
The implementation supports UTF-8 comment bodies and `////...` ordinary comments while
preserving original byte offsets for following tokens.

Outer `///` and inner `//!` documentation comments remain explicitly unsupported. P1-M001 does not
claim a complete Rust input-normalization pipeline.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No implementation may begin until a new milestone plan is created, reviewed, Approved, committed,
and validated by CI. The next planned compiler milestone is `P1-M002` — block comments. P0-M021
licensing remains a separate Phase 0 release-governance decision.
