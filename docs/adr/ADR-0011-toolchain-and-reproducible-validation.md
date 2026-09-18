# ADR-0011: Toolchain and reproducible validation

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Testing only current stable Rust can silently break the declared minimum supported Rust version.
Testing only all features can hide no-default-feature failures. Unlocked dependency resolution can
also make local and CI observations harder to reproduce.

## Decision

Ferraxis treats the workspace `rust-version` as an independently tested MSRV and uses the
committed `Cargo.lock` for repository validation.

Normal development tests stable Rust. Full validation also checks Rust 1.85.0, all features, and
no default features. CI keeps these compatibility boundaries visible as separate jobs.

Release readiness additionally requires package-content validation. Until the explicit licensing
milestone is complete, the release gate may report packaging readiness as blocked.

## Consequences

Stable Rust may advance without silently raising Ferraxis's supported compiler floor. Feature
configuration regressions and accidental dependency drift are detected earlier.

## References

- `Cargo.toml`
- `Cargo.lock`
- `scripts/check-msrv.sh`
- `scripts/check-features.sh`
