# Systems Programming w/ Rust

Exploring Rust's approach to systems programming through implementations. Rust caught my attention for systems programming - not claiming it's perfect, just curious about the hype. Building reliable, performant code with memory safety guarantees (well, C & C++ folks would somewhat not agree with me lol :P).

C gives you that 3 AM adrenaline rush when malloc goes rogue, but I prefer my crashes at compile time.

## What's Inside

**Core Concepts**
- Function modularity and clean separation of concerns
- Type-safe arithmetic operations with proper error handling
- Pattern matching for robust control flow
- Memory management through ownership and borrowing

**Future Exploration**
- Advanced web services with async patterns
- Database integration with SQLx
- Production deployment strategies

## The Classic Tradeoff

```
    Performance ↑
         │
         │  C/C++ ●
         │         ╲
         │          ╲
         │           ╲
         │    Rust 🦀 ●
         │             ╲
         │              ╲
         │               ● Others
         │
         └─────────────────────→ Memory Safety
         
    "Fast, safe, pick... both?" - Rust, probably
```

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
```

## Development Notes

Code follows idiomatic Rust patterns with error handling. All implementations pass clippy without warnings and use standard formatting via `cargo fmt`.

---

*Building reliable systems software with Rust*

<!-- AUTO-UPDATE: This section updates automatically as new implementations are completed -->
