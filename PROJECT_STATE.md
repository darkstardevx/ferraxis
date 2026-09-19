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
- Implementation status: implemented on the feature branch; exact implementation CI and evidence
  inspection remain.
- Approved plan checkpoint: `d7070206d1cbb8cff04edea49ca688f653259060`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35420536691>.

## Recently completed milestone

- `P1-M003` — Nested block comments.
- Merge commit: `1af475f1e269a841fad0440f2653e89abf7e037a`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419858406>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line and
recursive block comments, explicit non-delimiter punctuation identity, explicit EOF tokens,
deterministic token dumping, and a versioned differential lexer-observation harness.

## P1-M004 semantic boundary

Ferraxis recognizes all 46 non-delimiter punctuation spellings in the current Rust Reference using
longest-first matching.

Comments retain priority over slash/star punctuation. Unsupported top-level doc comments remain
controlled failures. Rust-2024 multi-pound reserved forms and identifier-adjacent pound prefixes
remain rejected.

Raw identifiers remain a P1-M011 gap. Bracket delimiters remain P1-M005. Bare underscore and
lifetimes remain outside P1-M004.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the exact P1-M004 implementation head to pass the complete CI matrix. Inspect uploaded
differential evidence, then close P1-M004 only if punctuation, reserved-boundary, raw-identifier,
and delimiter cases match their committed classifications.
