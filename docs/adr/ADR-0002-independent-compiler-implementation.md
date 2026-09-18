# ADR-0002: Independent compiler implementation

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

A different backend plugged into rustc increases backend diversity but does not create an
independent compiler frontend and semantic implementation.

## Decision

Ferraxis implements its own compiler pipeline and does not depend on rustc-private crates or rustc
internal compiler APIs.

## Consequences

Compatibility must be earned through specifications, observable behavior, and tests. Development
may be slower than reusing rustc internals, but the resulting implementation provides genuine
compiler diversity.

## References

- `docs/COMPILER_INVARIANTS.md`
