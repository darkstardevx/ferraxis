# ADR-0001: Semantic authority hierarchy

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Rust semantics are described by multiple sources, and implementation behavior can expose details
that are absent from or inconsistent with documentation.

## Decision

Ferraxis uses the five-level semantic authority order defined in `docs/SEMANTICS_AUTHORITY.md`.
The strongest available source governs a compatibility decision unless a documented variance is
accepted.

## Consequences

Semantic work must carry traceable evidence. `rustc` remains crucial compatibility evidence but is
not silently promoted above explicit language specification.

## References

- `docs/SEMANTICS_AUTHORITY.md`
- `docs/semantics/README.md`
