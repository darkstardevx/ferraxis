# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M005` — Delimiters.
- Active plan: `.plans/P1-M005-delimiters.plan.md`.
- Plan status: Approved.
- Implementation status: not started; exact plan-only CI must succeed first.

## Recently completed milestone

- `P1-M004` — Punctuation.
- Main merge: `8af85fa0abb5e055dd0a3c1c2e6f53aa0fff7c2d`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422377545>.
- Result: all six CI jobs passed on the exact merge head.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary line comments,
recursively nested ordinary block comments, explicit non-delimiter punctuation identity, explicit
EOF tokens, deterministic token dumping, and a versioned differential lexer-observation harness.

P1-M005 will add six explicit flat delimiter token identities.

## P1-M005 architecture boundary

ADR-0013 assigns only delimiter spelling recognition to the lexer.

The lexer will emit open/close parenthesis, square bracket, and brace tokens without maintaining a
pairing stack. Group construction and unmatched/mismatched delimiter validation belong to a later
frontend layer.

Because the stable rustc differential harness uses a macro token-tree probe, unmatched or mismatched
delimiter cases are expected to become `rustc_rejects` observations after P1-M005 rather than lexer
rejections.

## Required adjustment

The ADR registry previously stopped at ADR-0009 even though ADR-0010 through ADR-0012 existed.
P1-M005's plan checkpoint brings that registry current and adds ADR-0013.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the exact Approved P1-M005 plan checkpoint to pass all six CI jobs. Inspect and classify any
failure before making a repair. Only after that exact green checkpoint may Rust or Cargo
implementation begin.
