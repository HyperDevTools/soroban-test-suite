# Contributing to Soroban Test Suite

Thank you for your interest in contributing to Soroban Test Suite! We welcome contributions from the community.

## 🤝 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🔒 Security & Vulnerability Reporting

- **Never report security vulnerabilities in public GitHub issues or pull requests.** Follow the private disclosure instructions in [SECURITY.md](SECURITY.md) (via [GitHub Security Advisories](https://github.com/HyperDevTools/soroban-test-suite/security/advisories/new) or `security@hyperdevtools.com`).
- **No secrets in commits:** Never include Stellar secret keys (`S...`), seed phrases, API tokens, or credentials in code, tests, examples, or documentation. Use deterministic Soroban test generators such as `Address::generate(&env)` instead.

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/soroban-test-suite.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run all unit, integration, and example targets: `cargo test --all-targets`
6. Run linter: `cargo clippy --all-targets -- -D warnings`
7. Check formatting: `cargo fmt --all -- --check`
8. Commit your changes: `git commit -m "feat: your feature description"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## 📝 Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

## 🧪 Testing Requirements

- All new features must include unit or integration tests
- All targets must compile and pass: `cargo test --all-targets`
- Code must pass clippy without warnings: `cargo clippy --all-targets -- -D warnings`
- Code must be formatted: `cargo fmt --all -- --check`

## 🔍 Security & Maintainer Review Process

Every pull request must be reviewed and approved by a maintainer listed in [`.github/CODEOWNERS`](.github/CODEOWNERS) before merge:

1. **Dependency & Supply-Chain Review (`Cargo.toml`, `Cargo.lock`):**
   - Avoid unnecessary new dependencies. Any dependency addition or version bump must justify why it is needed, use minimal feature flags, and avoid unvetted `build.rs` or proc-macro network/filesystem side effects.
2. **Source & Workflow Review (`src/`, `examples/`, `.github/workflows/`):**
   - Obfuscated code, unexplained binary blobs, arbitrary command execution, or changes that weaken CI permissions will be rejected immediately.
   - Documentation and examples must match the actual implemented crate APIs without claiming unimplemented features.
3. **Maintainer Branch-Protection Requirements:**
   - Note that `.github/CODEOWNERS` alone does not enforce approval or block direct pushes unless repository branch protection (or a repository ruleset) is enabled on `main`.
   - Maintainers should configure branch protection on `main` (**Settings → Branches → Branch protection rules** or **Settings → Rules → Rulesets**) with:
     - **Require a pull request before merging** enabled
     - **Require approvals** (at least 1) and **Require review from Code Owners** enabled
     - **Dismiss stale pull request approvals when new commits are pushed** enabled
     - **Require status checks to pass before merging** enabled (gating on the CI workflow)
     - **Do not allow bypassing the above settings** / block force pushes and branch deletion on `main`
   - Ensure **Private vulnerability reporting** is enabled under **Settings → Code security and analysis**.

## 📋 Pull Request Checklist

1. Update `README.md` and examples if public APIs change
2. Verify `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --all -- --check` pass
3. Confirm no secrets, credentials, or sensitive payloads are included in the diff
4. Link the related issue in your PR description (`Closes #<issue>`)
5. Wait for code-owner review and CI checks to complete

## 🐛 Reporting Non-Security Bugs

When reporting non-security bugs, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Your environment (`rustc --version`, OS, `soroban-sdk` version)
- Minimal reproduction code

---

Happy coding! 🌟
