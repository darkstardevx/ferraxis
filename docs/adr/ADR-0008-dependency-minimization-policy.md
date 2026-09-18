# ADR-0008: Dependency-minimization policy

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Early compiler dependencies can silently choose syntax models, diagnostics architecture, Unicode
behavior, or IR structure before Ferraxis has defined its own requirements.

## Decision

Phase 0 compiler crates use the Rust standard library unless an external dependency has a documented
technical justification. Dependencies are not forbidden permanently; consequential additions must
be deliberate and reviewable.

## Consequences

Initial code is more explicit and easier to audit. Mature, well-scoped libraries may be adopted
later through normal review or an ADR where the dependency shapes architecture.

## References

- `AGENTS.md`
