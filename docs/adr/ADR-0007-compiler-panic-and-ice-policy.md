# ADR-0007: Compiler panic and ICE policy

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Compilers process hostile, malformed, incomplete, and simply incorrect user input. Panics caused by
normal source input make diagnostics unreliable and complicate fuzzing.

## Decision

User-controlled source input must not cause a compiler panic. Invalid programs produce structured
failure data or diagnostics. Any user-input ICE requires a regression test.

## Consequences

Error paths are first-class implementation paths. Internal assertions remain available only for
violations of true compiler invariants rather than ordinary source errors.

## References

- `docs/COMPILER_INVARIANTS.md`
