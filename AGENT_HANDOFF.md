# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M005`.
- Active plan: `.plans/P1-M005-delimiters.plan.md`.
- Plan status: `Approved`.
- Implementation status: not started.
- Required checkpoint: exact plan-only CI success across all six jobs before Rust or Cargo changes.

## Strict resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Confirm the exact branch head before every mutation.
6. Inspect existing branch and PR state before creating or resetting anything.
7. Read the active P1-M005 plan, ADR-0013, SEM-LEX-0008, and ADR-0012.
8. Confirm exact Approved-plan CI success before implementation.
9. If CI fails, classify the failure before editing:
   semantic/test, rustfmt/clippy, generated-content corruption, docs/text policy,
   workflow/governance, or infrastructure.
10. Repair forward; never force-reset or weaken a gate.

## Current work

P1-M005 will add flat lexical delimiter tokens for:

- `(` and `)`;
- `[` and `]`;
- `{` and `}`.

The lexer will not validate pairing or construct groups.

## Architecture boundary

ADR-0013 places delimiter pairing and group construction after lexing.

Unmatched or mismatched delimiter sequences may therefore be accepted by the flat lexer while the
stable rustc macro token-tree probe rejects them. Those expected `rustc_rejects` classifications
are evidence of the observation boundary from ADR-0012, not a request to move grouping into the
lexer.

## Evidence plan

P1-M005 will:

- move `paired-delimiters` from `ferraxis_rejects` to `agree_accept`;
- move `unmatched-open-delimiter` from `agree_reject` to expected `rustc_rejects`;
- add balanced nested and punctuation-adjacent delimiter cases;
- add unmatched-close and mismatched delimiter cases expected `rustc_rejects`;
- add direct unit and CLI tests for exact delimiter token identity and spans.

## Next exact action

Require the plan-only head to pass all six CI jobs. Do not begin implementation on a queued,
in-progress, cancelled, or partially green run.

## Validation rule

The lexer recognizes delimiter spellings. It does not decide whether those delimiters form valid
groups. Differential classifications are evidence, not correctness verdicts.
