# Security Policy

## Supported Versions

`soroban-test-suite` is currently in pre-1.0 active development (`0.1.x`). Security fixes are applied to the latest `0.1.x` release and the `main` branch.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.**

Instead, please report suspected vulnerabilities privately using one of the following channels:

1. **GitHub Private Vulnerability Reporting (Preferred)**:
   Navigate to the repository's **Security** tab and click **Report a vulnerability** (`https://github.com/HyperDevTools/soroban-test-suite/security/advisories/new`).
2. **Include in Your Report**:
   - Affected crate version or commit SHA
   - Minimal reproducible Soroban test case or proof of concept
   - Description of the impact (e.g. unsound mock behavior, false-positive assertion pass, or dependency vulnerability)
   - Suggested remediation, if available

## Response Timeline

- **Initial Acknowledgment**: Within **48 hours** of submission.
- **Triage & Severity Assessment**: Within **5 business days**.
- **Fix & Coordinated Disclosure**: Within **14–30 days** depending on complexity.

## Scope

Although `soroban-test-suite` is a `[dev-dependencies]` testing library rather than an on-chain production contract, vulnerabilities that can cause developer test suites to silently pass broken contract invariants (false-negative assertions, incorrect time/ledger simulation, or unsafe dependency advisories) are treated as high-priority security issues.
