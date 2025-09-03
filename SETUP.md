# Development Environment Setup

## Rust Toolchain Installation

### Complete Installation
```bash
# Install Rust with rustup (package manager for Rust toolchain)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment (or restart terminal)
source ~/.cargo/env
```

This single command installs:
- `rustc` - Rust compiler
- `cargo` - Package manager and build system  
- `clippy` - Advanced linter for catching common mistakes
- `rustfmt` - Code formatter following Rust conventions

### Verification
```bash
# Confirm installation
rustc --version  # Should show Rust compiler version
cargo --version  # Should show Cargo version

# Test with simple compilation
echo 'fn main() { println!("Hello, Rust!"); }' > test.rs
rustc test.rs && ./test && rm test test.rs
```

## Project Execution Patterns

### Running Code
```bash
# Execute specific implementations
cargo run --bin example_name

# Quiet execution (suppresses compilation output for cleaner terminal)
cargo run -q --bin example_name

# Check code without running (faster for syntax verification)
cargo check
```

### Code Quality Tools
```bash
# Advanced linting (catches logic errors, suggests improvements)
cargo clippy

# Format code according to official Rust style guide
cargo fmt

# Run tests (when available)
cargo test
```

### Advanced Project Setup (for ClipStash)
```bash
# Install database CLI tool for advanced projects
cargo install sqlx-cli

# Set up database schema (when working with ClipStash project)  
sqlx database setup
```

## Editor Configuration

### VS Code Extensions
- **rust-analyzer** - Language server for code completion and analysis
- **Better TOML** - Syntax highlighting for Cargo.toml files

### Configuration
```json
// settings.json
{
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

## Common Commands Reference

```bash
# Project initialization (if creating new project)
cargo new project_name        # Create new project
cargo init                    # Initialize in existing directory

# Dependency management
cargo add dependency_name     # Add dependency to Cargo.toml
cargo update                  # Update dependencies

# Build variants
cargo build                   # Debug build (faster compilation)
cargo build --release         # Optimized build (better performance)

# Documentation
cargo doc --open              # Generate and view project documentation
```

## Basic Development Cycle

1. **Edit code** in your preferred editor
2. **Quick check** with `cargo check` (fast syntax verification)
3. **Run** with `cargo run -q --bin name` (quiet output)
4. **Lint** with `cargo clippy` and **format** with `cargo fmt`

See [WORKFLOW.md](WORKFLOW.md) for detailed development practices.

## Environment Notes

- Rust installs to `~/.cargo/bin/` (added to PATH automatically)
- Project dependencies download to global cache `~/.cargo/registry/`
- Each project has local `target/` directory for build artifacts
- Use `-q` flag consistently to reduce terminal noise during development