# Systems Programming with Rust

Exploring Rust's approach to systems programming through hands-on implementations. This repository tracks my journey building reliable, performant code with Rust's memory safety guarantees.

## What's Inside

**Core Concepts**
- Function modularity and clean separation of concerns
- Type-safe arithmetic operations with proper error handling
- Pattern matching for robust control flow
- Memory management through ownership and borrowing

**Advanced Implementation**
- ClipStash: Full-stack web service with async patterns and database integration

## Current Progress

**Completed:**
- Function Design Patterns (`01-functions/`) - Clean modular architecture
- Arithmetic Operations (`02-arithmetic/`) - Type-safe computational abstractions

**In Progress:**
- Pattern Matching - Exploring Rust's match expressions
- Loop Constructs - Implementing iterative patterns

## Quick Start

```bash
# Run any implementation
cargo run --bin example_name

# Quiet execution (cleaner output)
cargo run -q --bin example_name
```

See [SETUP.md](SETUP.md) for complete environment configuration.

## Structure

```
fundamentals/     Core Rust concepts through practical examples
projects/         Real-world applications demonstrating advanced patterns
reference/        Implementation notes and development workflow
```

## Development Notes

Code follows idiomatic Rust patterns with comprehensive error handling. All implementations pass clippy without warnings and use standard formatting via `cargo fmt`.

---

*Building reliable systems software with Rust*

<!-- AUTO-UPDATE: This section updates automatically as new implementations are completed -->