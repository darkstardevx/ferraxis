# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M004`.
- Active plan: `.plans/P1-M004-punctuation.plan.md`.
- Plan status: `Approved`.
- Implementation status: not started.
- Required checkpoint: exact plan-only CI success before Rust or Cargo changes.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P1-M004 plan.
6. Read `SEM-LEX-0007`.
7. Read ADR-0001, ADR-0003, ADR-0006, ADR-0011, and ADR-0012.
8. Confirm exact Approved-plan CI success before implementation.
9. Never bypass repository hooks or weaken a gate.

## Current work

P1-M004 will implement the 46 non-delimiter punctuation spellings in the current Rust Reference.

Frozen boundaries:

- use explicit `Punctuation` token identity;
- longest spelling wins;
- comments beat slash/star punctuation;
- unsupported doc comments remain failures;
- `##` and longer pound runs remain rejected;
- identifier-adjacent `#` remains guarded;
- `r#name` stays unsupported until P1-M011 rather than being split;
- `( ) [ ] { }` stay P1-M005;
- `_` stays with keywords;
- `'` stays with lifetimes.

## Evidence basis

Primary authority is the Rust Reference `PUNCTUATION` production and reserved-token rules.

Differential evidence will cover representative punctuation groups, comment coexistence, reserved
pounds/prefixes, the raw-identifier gap, and the matched-delimiter gap.

## Observation boundary

The rustc side remains a stable macro token-tree acceptance probe. It does not expose rustc's raw
token boundaries, so exact longest-match claims are established by the Reference plus Ferraxis unit
tests, not by differential acceptance alone.

## Next exact action

Require the plan-only head to pass the complete CI matrix. If green, implement the approved token
representation and punctuation recognizer, inspect differential evidence, close P1-M004, and
require closed-state CI before merge.

## Validation rule

Do not let punctuation support silently absorb future token families or previously unsupported
documentation comments. Differential classifications are evidence, not correctness verdicts.
