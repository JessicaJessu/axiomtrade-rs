# Contributing to axiomtrade-rs

Thank you for your interest in contributing to axiomtrade-rs! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a professional and respectful environment. We are committed to providing a welcoming and inclusive experience for everyone.

## How to Contribute

### Reporting Issues

Before creating an issue, please check existing issues to avoid duplicates.

When reporting issues, include:
- Clear description of the problem
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information (OS, Rust version)
- Relevant code snippets or error messages

### Feature Requests

Feature requests are welcome. Please provide:
- Clear use case description
- Proposed API design (if applicable)
- Examples of how the feature would be used
- Any relevant prior art or references

### Pull Requests

1. **Fork the repository** and create your branch from `master`
2. **Follow the code style** guidelines below
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Ensure all tests pass** with `cargo test`
6. **Format your code** with `cargo fmt`
7. **Check for issues** with `cargo clippy`
8. **Submit a pull request** with clear description

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/axiomtrade-rs
cd axiomtrade-rs

# Add upstream remote
git remote add upstream https://github.com/vibheksoni/axiomtrade-rs

# Create a feature branch
git checkout -b feature/your-feature-name

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run a specific example
cargo run --example basic_login
```

## Code Style Guidelines

### Rust Conventions

- Follow standard Rust naming conventions
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Prefer explicit error handling over `.unwrap()`
- Use meaningful variable and function names

### Documentation

- Add docstrings to all public functions
- Include examples in documentation when helpful
- Keep documentation concise and accurate
- No comments in implementation code (self-documenting)

### Docstring Format

```rust
/// Brief description of the function
/// 
/// # Arguments
/// 
/// * `param_name` - Description of the parameter
/// 
/// # Returns
/// 
/// Description of return value
/// 
/// # Errors
/// 
/// Description of possible errors
/// 
/// # Examples
/// 
/// ```
/// // Example usage
/// ```
```

### Testing

- Write unit tests for new functionality
- Add integration tests for API interactions
- Ensure tests are deterministic
- Mock external dependencies when appropriate
- Aim for high test coverage

### Commit Messages

Follow conventional commit format:

```
type: brief description

Longer explanation if needed.

Fixes #123
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions or changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

## Project Structure

```
src/
├── client/         # Core client implementation
├── auth/           # Authentication logic
├── api/            # API endpoint implementations
├── websocket/      # WebSocket handling
├── models/         # Data structures
├── utils/          # Utility functions
└── error.rs        # Error types

examples/           # Example programs
tests/             # Integration tests
```

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use axiomtrade_rs::*;

#[tokio::test]
async fn test_integration_flow() {
    // Test implementation
}
```

## Performance Considerations

- Minimize allocations in hot paths
- Use `&str` instead of `String` where possible
- Prefer iterators over collecting into vectors
- Use `Arc` for shared immutable data
- Profile performance-critical code

## Security Guidelines

- Never commit credentials or API keys
- Sanitize all user inputs
- Use secure password hashing (PBKDF2)
- Implement proper error handling
- Follow principle of least privilege

## Documentation

### API Documentation

- Document all public APIs
- Include usage examples
- Describe error conditions
- Keep documentation up-to-date

### Examples

When adding examples:
- Place in `examples/` directory
- Include clear comments
- Show real-world usage
- Handle errors properly
- Follow existing naming patterns

## Release Process

1. Ensure all tests pass
2. Update version in `Cargo.toml`
3. Update CHANGELOG.md
4. Create pull request
5. After merge, tag the release
6. Publish to crates.io

## Getting Help

- Open an issue for bugs or features
- Start a discussion for questions
- Email vibheksoni@engineer.com for security issues

## Recognition

Contributors are recognized in:
- GitHub contributors page
- Release notes
- Project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.