# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M004` — Punctuation.
- Active plan: `.plans/P1-M004-punctuation.plan.md`.
- Plan status: Approved.
- Implementation status: not started; exact plan-only CI must succeed first.

## Recently completed milestone

- `P1-M003` — Nested block comments.
- Merge commit: `1af475f1e269a841fad0440f2653e89abf7e037a`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419858406>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, recursively nested ordinary non-doc block comments, explicit EOF tokens, deterministic
token dumping, and a versioned differential lexer-observation harness.

P1-M004 will add explicit token identity for the 46 non-delimiter punctuation spellings in the
current Rust Reference.

## P1-M004 semantic boundary

- longest valid punctuation spellings win over shorter prefixes;
- comments retain priority over slash/star punctuation;
- top-level documentation comments remain unsupported;
- Rust-2024 multi-pound reserved forms remain rejected;
- identifier-adjacent pound prefixes remain rejected;
- valid raw identifiers remain a P1-M011 gap rather than being split;
- six bracket delimiters remain P1-M005;
- bare underscore and lifetimes remain outside this milestone.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the Approved P1-M004 plan checkpoint to pass the complete CI matrix. Only after that exact
green checkpoint may Rust or Cargo implementation begin.
