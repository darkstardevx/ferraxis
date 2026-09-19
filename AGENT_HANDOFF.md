# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M003`.
- Active plan: `.plans/P1-M003-nested-block-comments.plan.md`.
- Plan status: `Approved`.
- Implementation status: not started.
- Required checkpoint: exact plan-only CI success before Rust or Cargo changes.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P1-M003 plan.
6. Read `SEM-LEX-0005` and `SEM-LEX-0006`.
7. Read ADR-0001, ADR-0003, ADR-0006, ADR-0011, and ADR-0012.
8. Confirm exact Approved-plan CI success before implementation.
9. Never bypass repository hooks or weaken a gate.

## Current work

P1-M003 will implement recursive nesting inside ordinary Rust block comments.

Frozen boundaries:

- the scanner uses an iterative depth counter;
- nested ordinary `/* ... */` contributes to depth;
- nested `/** ... */` and `/*! ... */` forms also contribute to depth when already inside an
  ordinary outer comment;
- top-level block documentation comments remain unsupported;
- EOF before depth returns to zero is a controlled failure at the outer opener;
- no arbitrary nesting cap is introduced;
- original byte offsets remain authoritative.

## Evidence basis

Primary authority is the Rust Reference recursive `BLOCK_COMMENT` grammar and
`BLOCK_COMMENT_OR_DOC` production.

The existing `nested-block-comment` differential case must move from `ferraxis_rejects` to
`agree_accept`. New cases will cover deeper and mixed nesting plus unterminated nested input.

## Observation boundary

The rustc side remains a stable macro token-tree acceptance probe. It is not a raw rustc lexer
dump and must not be described as token-for-token equivalence.

## Next exact action

Require the plan-only head to pass the complete CI matrix. If green, implement iterative nesting,
inspect the expanded differential artifact, close P1-M003, and require closed-state CI before
merge.

## Validation rule

Nested doc-comment forms inside an ordinary outer block comment are nesting syntax, not top-level
attributes. Top-level block docs remain unsupported. Differential classifications are evidence,
not correctness verdicts.
