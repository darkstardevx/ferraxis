# ADR-0005: Backend-neutral Ferraxis IR

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Using a backend's IR as the compiler's core IR would couple frontend and optimization architecture
to that backend.

## Decision

Ferraxis will own and specify its eventual backend-neutral code-generation IR. LLVM IR, GCC IR,
Cranelift IR, and machine-specific structures must not become the compiler's semantic boundary.

## Consequences

Backends become replaceable consumers. The Ferraxis IR itself will require dedicated future ADRs
before implementation.

## References

- `docs/architecture/OVERVIEW.md`
- `docs/COMPILER_INVARIANTS.md`
