# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M005`.
- Active plan: `.plans/P1-M005-delimiters.plan.md`.
- Plan status: `Approved`.
- Implementation status: six flat delimiter tokens implemented; exact implementation CI,
  differential artifact inspection, and closure remain.
- Approved plan checkpoint: `174a6e4c44b9f998d3ff3138fac895c071e22543`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422605365>.

## Strict resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Confirm the exact branch head before every mutation.
6. Read the active P1-M005 plan, ADR-0013, SEM-LEX-0008, and ADR-0012.
7. If CI fails, classify the failure before editing.
8. Repair forward; never force-reset or weaken a gate.

## Current work

P1-M005 now implements flat lexical delimiter tokens for:

- `(` and `)`;
- `[` and `]`;
- `{` and `}`.

The public model is `TokenKind::Delimiter(Delimiter)` with exact spelling returned by
`Delimiter::as_str()`.

## Architecture boundary

The lexer performs no grouping or balance validation.

Unmatched and mismatched delimiter sequences lex successfully as flat tokens. Pairing and grouped
token-tree or parser validation remain later frontend responsibilities under ADR-0013.

## Evidence expectations

The differential corpus now expects:

- `paired-delimiters`: `agree_accept`;
- balanced all/nested/punctuation-mix cases: `agree_accept`;
- `unmatched-open-delimiter`: `rustc_rejects`;
- `unmatched-close-delimiter`: `rustc_rejects`;
- `mismatched-delimiters`: `rustc_rejects`.

The `rustc_rejects` results are expected because the stable rustc probe requires balanced macro
token trees while the Ferraxis lexer emits flat tokens.

## Next exact action

Require the implementation head to pass all six CI jobs. Inspect the uploaded differential artifact
and verify both balanced acceptance and expected unbalanced `rustc_rejects` evidence before
closing P1-M005.

## Validation rule

Do not add delimiter pairing to the lexer to make the rustc token-tree probe agree. Differential
classifications are evidence, not correctness verdicts.
