# Semantic Authority

Ferraxis is an independent Rust compiler implementation. It does not define Rust behavior by
copying rustc internals, and it does not assume any single source perfectly describes the whole
language.

## Authority order

When deciding Rust semantics, use the strongest available source in this order:

1. **Explicit Rust language definition/specification.** Normative or specification-oriented Rust
   language material is the first authority.
2. **Stable documented Rust behavior.** Stable user-facing Rust documentation is next when the
   explicit language definition is incomplete.
3. **`rustc` observable behavior.** The reference implementation is strong compatibility evidence,
   but implementation behavior is not automatically normative language law.
4. **Differential tests.** Reproducible comparisons with `rustc` and other implementations help
   characterize behavior and regressions.
5. **Documented Ferraxis extension/variance.** If Ferraxis intentionally differs, the variance must
   be explicit, tested, and traceable.

## Conflict handling

When sources disagree:

1. record each source;
2. create or update a semantic evidence record;
3. preserve a minimal reproducer;
4. record observed `rustc` behavior and compiler version when relevant;
5. avoid silently treating implementation behavior as specification text;
6. resolve the Ferraxis behavior through an ADR if the choice affects architecture or long-lived
   compatibility.

## Evidence levels

Semantic records use these authority labels:

- `L1` — explicit language definition/specification;
- `L2` — stable documented behavior;
- `L3` — `rustc` observable behavior;
- `L4` — differential testing evidence;
- `L5` — documented Ferraxis variance or extension.

A record may contain multiple levels. Its `Primary authority` field names the strongest applicable
level.
