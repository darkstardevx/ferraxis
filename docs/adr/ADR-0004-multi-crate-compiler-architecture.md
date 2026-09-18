# ADR-0004: Multi-crate compiler architecture

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Compiler subsystems become tightly coupled when boundaries exist only by convention inside one
large crate.

## Decision

Ferraxis uses a Cargo workspace with subsystem crates created when their abstraction becomes real.
Phase 0 starts with span, source, diagnostics, lexer, driver, and repository tooling crates.

## Consequences

Dependency direction becomes visible and reviewable. Empty speculative crates are avoided until a
subsystem has concrete responsibilities.

## References

- `docs/architecture/OVERVIEW.md`
