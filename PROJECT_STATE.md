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
- Implementation status: implemented on the feature branch; exact implementation CI and evidence
  inspection are still required.
- Approved plan checkpoint: `174a6e4c44b9f998d3ff3138fac895c071e22543`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422605365>.

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
recursively nested ordinary block comments, explicit non-delimiter punctuation identity, six flat
delimiter token identities, explicit EOF tokens, deterministic token dumping, and a versioned
differential lexer-observation harness.

## P1-M005 architecture boundary

ADR-0013 assigns delimiter spelling recognition to the lexer and grouping after lexing.

The implementation emits open/close parenthesis, square bracket, and brace tokens with exact
one-byte spans. It does not maintain pairing state, reject mismatched kinds, or construct groups.

Unmatched and mismatched delimiters therefore remain valid flat lexer results while invalid group
structure must be rejected by a later frontend layer.

## Differential boundary

The stable rustc harness uses a macro token-tree probe, so its grouping requirements are stronger
than the Ferraxis flat lexer boundary.

P1-M005 expects:

- balanced delimiter cases to classify `agree_accept`;
- unmatched open, unmatched close, and mismatched delimiter cases to classify `rustc_rejects`.

Those classifications are evidence of ADR-0012 and ADR-0013 working together, not language
extensions.

## Required adjustment completed in the plan checkpoint

The ADR registry was brought current through ADR-0013 before implementation.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the exact P1-M005 implementation head to pass all six CI jobs. Inspect and classify any
failure before changing code. Inspect the uploaded differential artifact before closure.
