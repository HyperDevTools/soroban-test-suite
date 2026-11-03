# Contributing to Soroban Test Suite

Thank you for your interest in contributing to Soroban Test Suite! We welcome contributions from the community.

## 🤝 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/soroban-test-suite.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run linter: `cargo clippy -- -D warnings`
7. Format code: `cargo fmt`
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

- All new features must include tests
- All tests must pass: `cargo test`
- Code must pass clippy: `cargo clippy -- -D warnings`
- Code must be formatted: `cargo fmt --check`
- Aim for comprehensive test coverage

## 📋 Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md with your changes
3. Ensure all tests pass and code is formatted
4. Link any related issues in your PR description
5. Request review from maintainers

## 🐛 Reporting Bugs

When reporting bugs, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (Rust version, OS, etc.)
- Code samples if applicable

## 💡 Suggesting Features

We love new ideas! When suggesting features:

- Check if it's already been suggested
- Explain the use case clearly
- Describe how it would work
- Consider backward compatibility

## 📚 Development Guidelines

### Code Style

- Follow Rust conventions and idioms
- Use meaningful variable and function names
- Keep functions focused and modular
- Add documentation comments for public APIs
- Run `cargo fmt` before committing

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Arrange
        let context = setup_test();
        
        // Act
        let result = context.perform_action();
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## 🎯 Good First Issues

Look for issues labeled `good-first-issue` - these are great for new contributors!

## 💬 Questions?

Feel free to:
- Open an issue with the `question` label
- Join our community discussions
- Reach out to maintainers

## 🙏 Thank You!

Every contribution, no matter how small, helps make Soroban Test Suite better!

---

Happy coding! 🌟
