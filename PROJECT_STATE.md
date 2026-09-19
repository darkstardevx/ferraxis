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
- Implementation status: not started; plan-only CI checkpoint required first.

## Recently completed milestone

- `P0-M018` — Differential lexer harness skeleton.
- Main merge: `694ba6a11ccf7c7136aa7649e9edb0dcbd970caf`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35415828585>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis currently provides source-file storage, byte positions and half-open spans, structured
diagnostic data, initial ASCII whitespace and identifier lexing, exact `fn` recognition, explicit
EOF tokens, deterministic token dumping, and a versioned differential lexer-observation harness.

Ordinary Rust line comments are not implemented yet on this checkpoint.

## P1-M001 semantic boundary

P1-M001 targets the Rust Reference non-doc `LINE_COMMENT` grammar.

Ordinary non-doc comments will be treated as whitespace. Outer `///` and inner `//!`
documentation comments remain explicitly unsupported so the lexer does not erase syntax that has
attribute semantics.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require this Approved P1-M001 plan checkpoint to pass the complete CI matrix. Only after that green
checkpoint may Rust implementation begin.
