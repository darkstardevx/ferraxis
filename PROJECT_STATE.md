# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M002` — Block comments.
- Active plan: `.plans/P1-M002-block-comments.plan.md`.
- Plan status: Approved.
- Implementation status: not started; exact plan-only CI must succeed first.

## Recently completed milestone

- `P1-M001` — Line comments.
- Merge commit: `cc28d3265d6cc18af520593226158016b5423990`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35417271818>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, explicit EOF tokens, deterministic token dumping, and a versioned differential
lexer-observation harness.

P1-M002 will add depth-one ordinary non-doc block comments. Recursive nested block comments remain
P1-M003. Block documentation comments remain explicitly unsupported.

## P1-M002 semantic boundary

The Rust Reference treats non-doc block comments as whitespace and supports nesting.

P1-M002 intentionally implements only non-nested ordinary comments:

- `/**/` and `/***/` are ordinary block comments;
- `/** text */` is outer block documentation syntax and remains unsupported;
- `/*! text */` is inner block documentation syntax and remains unsupported;
- nested `/*` inside an open block comment is a controlled temporary rejection until P1-M003;
- unterminated block comments are controlled lexical failures.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the Approved P1-M002 plan checkpoint to pass the complete CI matrix. Only after that exact
green checkpoint may Rust or Cargo implementation begin.
