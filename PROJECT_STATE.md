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

- `P1-M004` — Punctuation.
- Final validated implementation head: `7baa2974e1fbc109399ccc91e94d9802489d6b7e`.
- Exact implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35420906959>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10577067943`.
- Result: 33/33 committed differential classifications matched.
- Ferraxis now has explicit token identity for all 46 current non-delimiter Reference punctuation
  spellings with longest-first recognition.

## Earlier completed Phase 1 milestones

- `P1-M003` — Nested block comments.
- Merge commit: `1af475f1e269a841fad0440f2653e89abf7e037a`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419858406>.
- `P1-M002` — Block comments.
- `P1-M001` — Line comments.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary line comments,
recursively nested ordinary block comments, explicit non-delimiter punctuation identity, explicit
EOF tokens, deterministic token dumping, and a versioned differential lexer-observation harness.

Punctuation uses one public `Punctuation` variant per supported Reference spelling and
`TokenKind::Punctuation(Punctuation)`. Recognition is longest-first and comment recognition
retains lexical priority.

Rust-2024 multi-pound reserved forms and identifier-adjacent pound reserved prefixes are rejected.
Raw identifiers remain a P1-M011 gap. Matched bracket delimiters remain a P1-M005 gap. Bare
underscore, lifetimes, literals, and Unicode identifiers remain owned by later milestones.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No Rust or Cargo implementation may begin until a new milestone plan is created, reviewed,
Approved, committed by itself, and validated by CI. The next planned compiler milestone is
`P1-M005` — delimiters.
