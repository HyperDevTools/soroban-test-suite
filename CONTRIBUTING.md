# Contributing to `soroban-test-suite`

Thank you for contributing to `soroban-test-suite`! This document outlines our development workflow, verification requirements, and pull request review standards.

## Prerequisites

- **Rust Toolchain**: Stable Rust (`1.78+` recommended) via [`rustup`](https://rustup.rs/)
- **Components**: `rustfmt` and `clippy` (`rustup component add rustfmt clippy`)

## Local Development Workflow

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/<your-username>/soroban-test-suite.git
   cd soroban-test-suite
   ```
2. **Create a Feature or Fix Branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. **Run All Required Checks Before Submitting**:
   Every pull request must pass formatting, linting, unit/integration tests, documentation builds, and example compilation:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-targets --all-features
   cargo test --doc
   cargo run --example basic_contract_test
   ```

## Pull Request Review & Safety Guidelines

- **Zero `unsafe` Code**: Do not introduce `unsafe` blocks unless strictly required and justified with a `// SAFETY:` comment.
- **Deterministic Tests**: All unit tests and doctests must be deterministic and self-contained using `soroban_sdk::Env::default()`.
- **Public API Documentation**: Every new public struct, trait, method, or macro exported from `src/` must include rustdoc comments and unit test coverage.
- **Security-Sensitive Changes**: Changes to `.github/workflows/`, `Cargo.toml` dependencies, `SECURITY.md`, or core assertion/mock semantics require maintainer review as configured in [`.github/CODEOWNERS`](.github/CODEOWNERS).
