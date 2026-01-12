# Contributing to Prune

Thank you for your interest in contributing to Prune! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)
- Prune version

### Suggesting Features

Feature suggestions are welcome! Please:
- Check if the feature has already been requested
- Describe the use case clearly
- Explain why it would be valuable
- Consider implementation complexity

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/yourusername/prune
   cd prune
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed
   - Keep commits focused and atomic

4. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Submit a pull request**
   - Describe what changed and why
   - Link related issues
   - Ensure CI passes

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
prune/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI handling
│   ├── engine.rs         # Main discovery engine
│   ├── intelligence.rs   # Adaptive intelligence
│   ├── scanner.rs        # Directory scanner
│   ├── subdomain.rs      # Subdomain enumerator
│   ├── crawler.rs        # Web crawler
│   ├── session.rs        # Session management
│   ├── wordlist.rs       # Wordlist management
│   ├── ui.rs             # Terminal UI
│   └── utils.rs          # Utility functions
├── Cargo.toml            # Dependencies
└── README.md             # Documentation
```

## Code Style Guidelines

### Rust Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add documentation comments for public APIs

### Comments

- Use `///` for documentation comments
- Use `//` for inline explanations
- Explain *why*, not just *what*

```rust
/// Calculate confidence score based on historical patterns
/// 
/// This function weighs recent successes more heavily than
/// older results to adapt to changing target behavior.
pub fn calculate_confidence(&self, pattern: &str) -> f32 {
    // Recent hits should have more weight
    let recent_weight = 0.7;
    // ... implementation
}
```

### Error Handling

- Use `Result<T, Error>` for fallible operations
- Use `anyhow::Result` for application errors
- Provide context with `.context()` or `.with_context()`

```rust
let config = fs::read_to_string(&path)
    .context("Failed to read config file")?;
```

### Testing

Add tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection() {
        let engine = IntelligenceEngine::new();
        // ... test implementation
    }
}
```

## Intelligence Engine Guidelines

When adding new intelligence features:

1. **Keep it adaptive**: Features should learn and improve over time
2. **Minimize noise**: Filter aggressively to reduce false positives
3. **Be conservative**: Respect rate limits and target resources
4. **Document behavior**: Explain how learning algorithms work

## UI Guidelines

When modifying the UI:

1. **Color consistency**: Use the defined color palette
   - Primary: `#2596be` (blue)
   - Secondary: `#5621d5` (violet)

2. **Progressive disclosure**: Show essential info, hide details
3. **Clear feedback**: Make actions and results obvious
4. **Calm design**: Avoid overwhelming the user

## Performance Considerations

- Use async/await for I/O operations
- Leverage concurrency with reasonable limits
- Profile before optimizing
- Consider memory usage for large wordlists

## Security Considerations

- Validate all user input
- Sanitize paths and URLs
- Respect robots.txt (optional flag)
- Implement proper timeout handling
- Rate limit by default

## Documentation

When adding features:

1. Update README.md
2. Add inline documentation
3. Include usage examples
4. Update CHANGELOG.md

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag
4. Build release binaries
5. Create GitHub release

## Questions?

Feel free to open an issue for:
- Implementation questions
- Design discussions
- Feature clarifications
- General feedback

Thank you for contributing to Prune! 🌿
