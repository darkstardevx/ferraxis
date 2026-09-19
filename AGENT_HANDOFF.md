# Ferraxis Agent Handoff

## Repository state

- Active milestone: none.
- Active plan: none.
- P1-M005 status: Complete.
- Final validated P1-M005 implementation head:
  `95c8eb93183d12d4665c9031c49d9492d35c3316`.
- P1-M005 implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35422871724>.
- Differential artifact ID: `10578220814`.
- Differential result: 38/38 committed classifications matched.

## Strict resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Confirm there is no active implementation plan before choosing new work.
6. Confirm the previous milestone's post-merge `main` CI before creating the next branch.
7. Inspect existing branch and PR state before any mutation.
8. Classify any CI failure before editing.
9. Repair forward; never force-reset or weaken a gate.

## Completed work

P1-M005 adds six explicit flat delimiter identities:

- open and close parenthesis;
- open and close square bracket;
- open and close brace.

The public model is `TokenKind::Delimiter(Delimiter)`, and `Delimiter::as_str()` returns the exact
source spelling.

Validated behavior includes:

- balanced and nested delimiter sequences emitted in flat source order;
- exact one-byte spans;
- punctuation/comment coexistence;
- deterministic CLI token rendering;
- unmatched open and close delimiters remaining lexical tokens;
- mismatched delimiter kinds remaining lexical tokens;
- no delimiter stack or group construction inside the lexer.

## Architecture boundary

ADR-0013 assigns grouping and pairing validation after lexing.

The final differential artifact matched all 38 committed classifications. Balanced delimiter cases
classified `agree_accept`; unmatched and mismatched cases classified `rustc_rejects` because
the stable rustc observation operates at a macro token-tree boundary.

Do not move delimiter pairing into the lexer to make that probe agree.

## Implementation history

- Plan checkpoint: `174a6e4c44b9f998d3ff3138fac895c071e22543`, CI run 35422605365 green.
- Semantic implementation: `75277f990b68334b1aa5fcfe0fc04fa62c480aab`.
- Run 35422801222 failed only `cargo fmt --check`; the other five jobs passed.
- Rustfmt-only repair: `95c8eb93183d12d4665c9031c49d9492d35c3316`.
- Final implementation CI run 35422871724 passed all six jobs.

## Next compiler milestone

The next planned compiler milestone is `P1-M006` — integer literals.

Before Rust or Cargo implementation:

1. create a P1-M006 plan;
2. research Rust integer-literal lexical grammar, bases, separators, suffixes, and token boundaries;
3. keep semantic numeric value/type interpretation out of the lexer unless explicitly justified;
4. define unit, CLI, and differential evidence;
5. approve and commit the plan by itself;
6. require exact plan-checkpoint CI success;
7. only then implement integer literals.

P0-M021 licensing remains separate and continues to block release readiness.

## Validation rule

Differential classifications are evidence, not correctness verdicts. Preserve phase boundaries even
when the rustc token-tree observation is stricter than Ferraxis's lexer boundary.
