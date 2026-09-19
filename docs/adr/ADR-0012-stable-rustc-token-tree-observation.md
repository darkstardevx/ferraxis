# ADR-0012: Stable rustc token-tree observation boundary

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

ADR-0006 requires Ferraxis to gather reproducible differential evidence against rustc where that
comparison is meaningful.

Stable rustc does not expose a supported raw lexer-token dump interface. Compiler `-Z` options are
unstable/nightly interfaces and rustc-private crates would violate Ferraxis's independence
invariants.

Rust's stable macro system does expose a documented token-tree boundary: a `tt` macro fragment
matches one token or a properly delimited token tree. A repetition of `tt` fragments can therefore
be used to observe whether stable rustc accepts a fragment as token trees inside a generated probe.

## Decision

P0-M018 and later stable differential tooling may use a generated `macro_rules!` token-tree probe
as a **stable rustc token-tree acceptance observation**.

It must not be described as:

- a raw rustc lexer dump;
- token-for-token lexer equivalence;
- token-span equivalence;
- a parser-independent verdict for unmatched delimiters.

The probe uses ordinary stable rustc options and records exact `rustc --version --verbose` output,
edition, generated source, exit status, and stderr.

Nightly `-Z` options and rustc-private crates are not part of this observation path.

## Consequences

Ferraxis can begin differential lexical evidence gathering without coupling to rustc internals.

The observation is intentionally narrower than a raw lexer oracle. Delimiter balancing and
token-tree construction can influence rejection. A mismatch is evidence to investigate under
`docs/SEMANTICS_AUTHORITY.md`, not an automatic correctness verdict.

If Ferraxis later adopts a stronger independent token-level oracle, that capability must be
documented separately rather than silently changing the meaning of existing P0-M018 evidence.

## References

- [rustc command-line arguments](https://doc.rust-lang.org/rustc/command-line-arguments.html)
- [Rust Reference: macro metavariables](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables)
- [Rust Reference: tokens](https://doc.rust-lang.org/reference/tokens.html)
- `docs/adr/ADR-0006-differential-testing-against-rustc.md`
- `docs/TESTING.md`
