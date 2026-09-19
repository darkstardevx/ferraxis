# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M002`.
- Active plan: `.plans/P1-M002-block-comments.plan.md`.
- Plan status: `Approved`.
- Implementation status: depth-one ordinary block comments implemented; exact implementation CI,
  evidence inspection, and closure remain.
- Approved plan checkpoint: `0ce202345327701705763364da3dc3859a55a376`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35417983279>.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P1-M002 plan.
6. Read `SEM-LEX-0005`.
7. Read ADR-0001, ADR-0003, ADR-0006, ADR-0011, and ADR-0012.
8. Never bypass repository hooks or weaken a gate.

## Current work

P1-M002 now implements the non-nested subset of ordinary Rust block comments.

Validated-by-code boundaries awaiting exact CI:

- `/* ... */` ordinary comments are skipped as whitespace;
- multiline, UTF-8, line-marker, and bare-CR bodies are supported;
- `/**/`, `/***/`, and `/*** text */` are ordinary comments;
- `/** text */` remains unsupported outer block documentation syntax;
- `/*! text */` remains unsupported inner block documentation syntax;
- EOF before `*/` fails at the opening slash;
- nested `/*` fails at the nested opener until P1-M003;
- token spans after comments retain original byte offsets.

## Evidence basis

Primary authority is the Rust Reference comments grammar. The differential harness supplies L4
observation evidence.

The expanded corpus expects supported block comments to `agree_accept`, an unterminated block
comment to `agree_reject`, and nested block comments to remain `ferraxis_rejects` until P1-M003.

## Observation boundary

The rustc side remains a stable macro token-tree acceptance probe. It is not a raw rustc lexer
dump and must not be described as token-for-token equivalence.

## Next exact action

Require the implementation head to pass the complete CI matrix, inspect the differential artifact,
record exact evidence, close P1-M002, and require closed-state CI before merge.

## Validation rule

Do not collapse block documentation comments into ordinary comments. Do not truncate nested input
at the inner `*/`. Differential classifications are evidence, not correctness verdicts.
