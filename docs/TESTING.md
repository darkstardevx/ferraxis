# Testing Strategy

Ferraxis separates different claims so that one passing test category cannot masquerade as another.

## Taxonomy

- `tests/lexer/` — lexical corpus and golden expectations.
- `tests/compile-pass/` — source expected to be accepted.
- `tests/compile-fail/` — source expected to be rejected in a controlled way.
- `tests/run-pass/` — accepted programs whose runtime behavior is checked.
- `tests/differential/` — versioned comparisons with external compilers.

Unit and crate-level integration tests live beside the implementation where appropriate.

## Differential testing

Differential testing is evidence level `L4`. It must record enough environment information to
reproduce the observation, including compiler identity and version. A difference does not by itself
prove Ferraxis is wrong; it creates evidence to resolve through the semantic-authority policy.

## Stable rustc lexical observation boundary

Stable `rustc` does not provide Ferraxis with a supported raw lexer-token dump interface. P0-M018
therefore uses a deliberately narrower observation: whether stable `rustc` can tokenize a source
fragment as a sequence of macro `tt` token trees inside a generated probe crate.

The probe shape is:

```rust
macro_rules! __ferraxis_probe {
    ($($_:tt)*) => {};
}

__ferraxis_probe! {
    /* corpus fragment */
}
```

This observation is useful but must not be described as token-for-token lexer equivalence.
Delimiter balancing and macro token-tree construction can reject input after or alongside lexical
processing.

From P1-M005 onward, the Ferraxis lexer intentionally emits flat delimiter tokens without checking
pairing or group balance under ADR-0013. Therefore unmatched or mismatched delimiter corpus cases
may classify as `rustc_rejects`: Ferraxis recognizes the flat delimiter tokens while the rustc
macro token-tree probe rejects the unbalanced group structure.

That classification is expected evidence about the observation boundary. It must not be used as a
reason to move delimiter grouping into the lexer.

Every differential run records:

- exact `rustc --version --verbose` output;
- Rust edition used by the probe;
- target information reported by `rustc`;
- exact generated probe source;
- Ferraxis acceptance or rejection;
- rustc probe exit status;
- classification of the pair of observations.

P0-M018 uses these classifications:

- `agree_accept` — both Ferraxis and the rustc token-tree probe accept;
- `agree_reject` — both reject within their documented observation boundaries;
- `ferraxis_rejects` — rustc token-tree probe accepts while Ferraxis rejects;
- `rustc_rejects` — Ferraxis accepts while the rustc token-tree probe rejects.

A classification is evidence, not a correctness verdict. Interpretation still follows
`docs/SEMANTICS_AUTHORITY.md`.
