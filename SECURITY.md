# Security Policy

## Supported Versions

Security updates are released for the following versions of `soroban-test-suite`:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in `soroban-test-suite`, please report it privately so maintainers can investigate and ship a fix before public disclosure.

**Do NOT open a public GitHub issue, pull request, or discussion containing exploitable details.**

### Preferred Private Channels

1. **GitHub Security Advisories (Preferred):**
   - Navigate to the repository's **Security** tab: <https://github.com/HyperDevTools/soroban-test-suite/security/advisories/new>
   - Click **"Report a vulnerability"** and submit your report privately to the maintainers.
2. **Private Security Email:**
   - Send details to `security@hyperdevtools.com` if you cannot use GitHub Security Advisories.

### What to Include in a Report

- Affected crate version, commit SHA, and file/module path
- Clear description of the vulnerability and security impact
- Minimal, self-contained reproduction steps or proof-of-concept test
- Any suggested remediation or mitigation

## Response Timeline

- **Initial acknowledgment:** within **48 hours**
- **Status updates:** at least every **7 days** while triage and remediation are in progress
- **Remediation targets:**
  - **Critical / High severity:** patch or mitigation within **7 days**
  - **Medium / Low severity:** patch within **30 days**

## Coordinated Disclosure

Once a fix is verified and released, maintainers will publish a GitHub Security Advisory crediting the reporter (unless anonymity is requested). Please allow maintainers time to release a patch before disclosing findings publicly.

## Security Expectations for Contributors

- Never commit secrets, private keys, seed phrases, tokens, or credentials to source files, tests, fixtures, or documentation.
- Do not introduce build scripts (`build.rs`), proc-macros, or dependencies that execute arbitrary network or shell commands during `cargo test` or `cargo build`.
- All changes to `Cargo.toml`, `Cargo.lock`, `.github/workflows/`, and `src/` require maintainer review via `.github/CODEOWNERS` before merge.
