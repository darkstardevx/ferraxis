# ADR-0003: Byte-offset source spans

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Compiler phases need compact, deterministic source locations that map directly to UTF-8 source
storage.

## Decision

Source positions use zero-based byte offsets. Spans are half-open `[lo, hi)` ranges. Phase 0 stores
positions in `u32` through the `BytePos` abstraction.

## Consequences

Slicing requires UTF-8 boundary validation. Files larger than the representable Phase 0 byte range
must be rejected rather than truncated. The abstraction permits a future representation change.

## References

- `crates/ferraxis-span/`
- `crates/ferraxis-source/`
