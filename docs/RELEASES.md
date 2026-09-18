# Release Ladder

Ferraxis uses small pre-1.0 releases to make capability boundaries explicit. Version numbers do not
imply broad Rust compatibility unless the corresponding conformance evidence exists.

## Initial ladder

| Version | Intended capability |
| --- | --- |
| `0.0.1` | Repository, policy, ADR, evidence, CI, and documentation bootstrap. |
| `0.0.2` | Source-file and span foundations. |
| `0.0.3` | First lexer slice. |
| `0.0.4` | Basic token-stream tooling and early differential harness. |
| `0.1.0` | Lexer milestone complete for the declared supported Rust lexical surface. |
| `0.2.0` | Parser foundation. |
| `0.3.0` | Item and expression parser expansion. |
| `0.4.0` | Name-resolution foundation. |
| `0.5.0` | HIR foundation. |
| `0.6.0` | Type-system foundation. |

Versions after the type-system foundation will be assigned when their scope can be defended by
acceptance tests. Trait solving, borrow checking, standard-library bring-up, and self-hosting are
not assigned speculative dates or compatibility percentages.

## Release criteria

A release requires:

1. the complete code gate to pass;
2. the complete documentation gate to pass;
3. release-relevant milestones to have acceptance evidence;
4. no known regression hidden by disabling tests;
5. affected semantic records to be current;
6. affected ADRs and invariants to be current;
7. release notes to distinguish implemented, unsupported, and intentionally variant behavior.

## Validation layers

Normal development validates stable Rust with the committed lockfile. The full gate additionally
validates Rust 1.85.0, all features, no default features, documentation, repository workflow
structure, and shell tooling.

The release gate adds package-content validation. It is intentionally blocked until P0-M021
completes the explicit licensing decision and package metadata is release-ready.

A release candidate must be traceable to an exact successful CI run for the exact commit being
released.
