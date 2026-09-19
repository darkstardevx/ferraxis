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
- Implementation status: not started; exact plan-only CI must succeed first.

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
comments, depth-one ordinary non-doc block comments, explicit EOF tokens, deterministic token
dumping, and a versioned differential lexer-observation harness.

P1-M003 will close the temporary nested-block-comment variance using iterative depth tracking.

## P1-M003 semantic boundary

For a top-level ordinary block comment, every nested `/*` increments comment depth and every `*/`
decrements it. Nested ordinary, outer-doc, and inner-doc block forms all participate in depth.

Top-level block documentation comments remain unsupported because their attribute semantics remain
out of scope.

No arbitrary nesting limit is planned. The scanner remains iterative and linear in source length.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the Approved P1-M003 plan checkpoint to pass the complete CI matrix. Only after that exact
green checkpoint may Rust or Cargo implementation begin.
