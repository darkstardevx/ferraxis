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
