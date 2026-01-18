# Contributing to Tesserax Protocol

Thank you for your interest in contributing to Tesserax Protocol! This document provides guidelines for contributing to the project.

## 🌟 Ways to Contribute

- 🐛 **Bug Reports**: Submit detailed bug reports via GitHub Issues
- 💡 **Feature Requests**: Propose new features and improvements
- 📖 **Documentation**: Improve docs, add examples, fix typos
- 🔧 **Code Contributions**: Submit pull requests for bug fixes and features
- 🧪 **Testing**: Write tests, report test failures
- 🔐 **Security**: Responsibly disclose security vulnerabilities

## 📋 Before You Start

1. **Check existing issues/PRs** to avoid duplicates
2. **Read the documentation**:
   - [Whitepaper v3.0](docs/whitepaper-v3.0-id.md)
   - [Architecture Overview](docs/PROJECT_REVIEW.md)
   - [API Reference](docs/api-reference.md)
3. **Set up development environment**:
   ```bash
   git clone https://github.com/Tesserax-Protocol/tesserax-node.git
   cd tesserax-node
   cargo build --release
   cargo test
   ```

## 🔧 Development Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/tesserax-node.git
cd tesserax-node
git remote add upstream https://github.com/Tesserax-Protocol/tesserax-node.git
```

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

Branch naming convention:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 3. Make Changes

- Follow Rust style guide (`cargo fmt`)
- Run linter (`cargo clippy`)
- Add tests for new features
- Update documentation as needed

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific pallet tests
cargo test -p pallet-emission
cargo test -p pallet-quantum-vault
cargo test -p pallet-reml-verifier

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features
```

### 5. Commit

Use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat(emission): add bonus mint verification"
git commit -m "fix(quantum-vault): resolve nonce overflow issue"
git commit -m "docs(readme): update installation instructions"
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, semicolons, etc.)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## 📝 Pull Request Guidelines

### PR Title Format

```
<type>(<scope>): <short description>

Examples:
feat(reml): add STARK proof verification precompiles
fix(vault): prevent double-spend in transfer logic
docs(whitepaper): clarify emission schedule calculation
```

### PR Description Template

```markdown
## Description
Brief description of what this PR does.

## Motivation
Why is this change needed? What problem does it solve?

## Changes
- Change 1
- Change 2
- Change 3

## Testing
How was this tested?

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Tests pass (`cargo test`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for notable changes)
- [ ] No breaking changes (or clearly documented)
```

## 🧪 Testing Requirements

All code contributions must include tests:

### Unit Tests
```rust
#[test]
fn test_feature_works() {
    new_test_ext().execute_with(|| {
        // Test implementation
        assert_ok!(MyPallet::my_function(origin, params));
    });
}
```

### Integration Tests
Place in `tests/integration_tests.rs` or similar.

### Test Coverage
- **Minimum**: 80% coverage for new code
- **Preferred**: 90%+ coverage

## 📖 Documentation Standards

### Code Comments

```rust
/// Brief description of what this function does.
///
/// # Arguments
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When this function might error
///
/// # Example
/// ```
/// let result = my_function(arg1, arg2)?;
/// ```
pub fn my_function(param1: Type1, param2: Type2) -> Result<ReturnType, Error> {
    // Implementation
}
```

### Documentation Updates

When making changes, update:
- **README.md**: If affecting user-facing features
- **docs/**: Relevant documentation files
- **CHANGELOG.md**: For notable changes
- **Inline docs**: Function/struct documentation

## 🔐 Security

### Reporting Security Issues

**DO NOT** open public issues for security vulnerabilities.

Instead:
1. Email: security@tesserax.network
2. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We aim to respond within 48 hours.

### Security Best Practices

- Never commit secrets (keys, passwords, tokens)
- Use `saturating_*` operations to prevent overflow
- Validate all user inputs
- Follow Substrate security best practices
- Review cryptographic implementations carefully

## 🏗️ Architecture Guidelines

### Pallet Development

When creating/modifying pallets:

1. **Keep pallets focused**: One responsibility per pallet
2. **Minimize storage**: Use `StorageValue`, `StorageMap` efficiently
3. **Weight accurately**: Benchmark all extrinsics
4. **Error handling**: Define clear, descriptive errors
5. **Events**: Emit events for state changes

### Code Style

- **Formatting**: Use `cargo fmt` (enforced)
- **Naming**:
  - `snake_case` for functions, variables
  - `PascalCase` for types, structs, enums
  - `SCREAMING_SNAKE_CASE` for constants
- **Line length**: Max 100 characters (soft limit)
- **Imports**: Group by: std → external → internal

## 🤝 Code Review Process

### What We Look For

✅ **Approve**:
- Clean, readable code
- Comprehensive tests
- Proper documentation
- Follows style guide
- No breaking changes (or well-documented)

❌ **Request Changes**:
- Missing tests
- Insufficient documentation
- Style violations
- Breaking changes without migration path
- Security concerns

### Review Timeline

- **Small PRs** (< 200 lines): 1-2 days
- **Medium PRs** (200-500 lines): 3-5 days
- **Large PRs** (> 500 lines): 1 week+

**Tip**: Smaller, focused PRs get reviewed faster!

## 🎓 Learning Resources

### Substrate Development
- [Substrate Docs](https://docs.substrate.io/)
- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/)
- [Substrate Tutorials](https://docs.substrate.io/tutorials/)

### Tesserax-Specific
- [Whitepaper v3.0](docs/whitepaper-v3.0-id.md)
- [Project Review](docs/PROJECT_REVIEW.md)
- [Re-ML Architecture](docs/Re-ML.md)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT-0 License.

## 🙏 Recognition

Contributors will be recognized in:
- CHANGELOG.md (for significant contributions)
- GitHub contributors page
- Release notes

## ❓ Questions?

- **General questions**: Open a GitHub Discussion
- **Bug reports**: GitHub Issues
- **Security**: security@tesserax.network
- **Feature requests**: GitHub Issues with `enhancement` label

---

Thank you for contributing to Tesserax Protocol! 🚀

**"Mathematics-as-Money"** - Where supply meets the universal constants
