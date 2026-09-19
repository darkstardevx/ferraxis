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

- `P1-M005` — Delimiters.
- Final validated implementation head: `95c8eb93183d12d4665c9031c49d9492d35c3316`.
- Exact implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422871724>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10578220814`.
- Result: 38/38 committed differential classifications matched.
- ADR-0013 permanently assigns delimiter grouping and balance validation after lexing.

## Earlier completed Phase 1 milestones

- `P1-M004` — Punctuation.
- Main merge: `8af85fa0abb5e055dd0a3c1c2e6f53aa0fff7c2d`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422377545>.
- `P1-M003` — Nested block comments.
- `P1-M002` — Block comments.
- `P1-M001` — Line comments.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary line comments,
recursively nested ordinary block comments, explicit non-delimiter punctuation identity, six flat
delimiter token identities, explicit EOF tokens, deterministic token dumping, and a versioned
differential lexer-observation harness.

Delimiter tokens preserve exact one-byte spans and remain flat. The lexer does not validate
matching or construct groups.

## Delimiter observation boundary

Balanced delimiter sequences agree with the stable rustc macro token-tree probe.

Unmatched and mismatched delimiters are accepted as flat Ferraxis lexer tokens but rejected by the
rustc token-tree probe. Their `rustc_rejects` classifications are expected evidence under
ADR-0012 and ADR-0013, not Ferraxis language extensions.

A later frontend stage must validate delimiter pairing before complete Rust source is accepted.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No Rust or Cargo implementation may begin until a new milestone plan is created, reviewed,
Approved, committed by itself, and validated by CI. The next planned compiler milestone is
`P1-M006` — integer literals.
