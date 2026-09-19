# Ferraxis Agent Handoff

## Repository state

- Active milestone: none.
- Active plan: none.
- P0-M018 status: Complete.
- Final validated P0-M018 head: `dc9b792c42e1fe111ea60c88358ec8c2f9d9038a`.
- P0-M018 CI: <https://github.com/darkstardevx/ferraxis/actions/runs/35415625792>.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Confirm there is no active implementation plan before choosing new work.
6. Read the relevant ADRs and semantic evidence for the next milestone.
7. Never bypass repository hooks or weaken a gate to make a change pass.

## Completed work

P0-M018 established the non-publishable `ferraxis-diff` tool, committed lexer corpus and expected
classifications, deterministic evidence output, and a dedicated CI differential job.

The final artifact contained all expected evidence. Eleven of eleven committed classifications
matched. Only the unmatched-open-delimiter probe emitted rustc stderr, and that result remains
explicitly scoped as token-tree evidence.

## Observation boundary

The rustc side is a stable macro token-tree acceptance probe. It is not a raw rustc lexer dump and
must not be described as token-for-token equivalence.

## Next compiler milestone

The next planned compiler milestone is `P1-M001` — line comments.

Before Rust or Cargo implementation:

1. create a P1-M001 plan with `./scripts/plan new P1-M001 line-comments`;
2. complete its design, evidence, compatibility, and test-first sections;
3. approve it with `./scripts/plan approve`;
4. commit the Approved plan by itself;
5. require that plan checkpoint to pass CI;
6. only then implement line-comment lexing.

P0-M021 licensing remains separate and continues to block release readiness.

## Validation rule

Differential classifications are evidence, not correctness verdicts. Interpret mismatches through
Ferraxis's semantic-authority hierarchy.
