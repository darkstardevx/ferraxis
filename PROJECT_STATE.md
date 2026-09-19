# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M001` — Line comments.
- Active plan: `.plans/P1-M001-line-comments.plan.md`.
- Plan status: Approved.
- Implementation status: implemented on the feature branch; exact final CI evidence and closure are
  still required.
- Approved plan checkpoint: `75f65b476606941e50a25b65cfef6d6c364da5a9`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35416909308>.

## Recently completed milestone

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

## P1-M001 semantic boundary

Ordinary Rust non-doc `LINE_COMMENT` input is treated as whitespace and terminates at LF or EOF.
The implementation supports UTF-8 comment bodies and `////...` ordinary comments while
preserving original byte offsets for following tokens.

Outer `///` and inner `//!` documentation comments remain explicitly unsupported so Ferraxis does
not erase syntax with attribute semantics. P1-M001 does not claim a complete Rust
input-normalization pipeline.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the P1-M001 implementation head to pass the complete PR CI matrix. Inspect the uploaded
differential evidence and verify the line-comment cases match committed expectations. Only then
record exact CI evidence and close P1-M001.
