# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M003` — Nested block comments.
- Active plan: `.plans/P1-M003-nested-block-comments.plan.md`.
- Plan status: Approved.
- Implementation status: implemented on the feature branch; exact implementation CI and evidence
  inspection are still required.
- Approved plan checkpoint: `5c7cb524c4007e28a997b09d89660d520b51ebe9`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419633488>.

## Recently completed milestone

- `P1-M002` — Block comments.
- Merge commit: `6d6762aacdc0ab371404938c95d2a5832755ab6b`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35418267654>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, recursively nested ordinary non-doc block comments, explicit EOF tokens, deterministic
token dumping, and a versioned differential lexer-observation harness.

## P1-M003 semantic boundary

For a top-level ordinary block comment, every nested `/*` increments depth and every `*/`
decrements it. Nested ordinary, outer-doc, and inner-doc block forms all participate in depth.

The scanner is iterative, uses constant auxiliary memory, introduces no arbitrary nesting cap, and
anchors unterminated nested-comment failure at the original outer opener.

Top-level block documentation comments remain unsupported because their attribute semantics remain
out of scope.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the exact P1-M003 implementation head to pass the complete CI matrix. Inspect the expanded
differential artifact, then close P1-M003 only if all recursive and unterminated cases match their
committed classifications.
