# Systems Programming w/ Rust

Ever wonder what happens when you take C's performance obsession and add a compiler that actually cares about your mental health? 

That's basically Rust - and honestly, I got curious about the hype. Sure, C gives you that 3 AM adrenaline rush when malloc goes rogue, but I'd rather my code yell at me during compilation than crash in production (sorry C & C++ folks, but you know it's true lol :P).

This repo is my journey through Rust's take on systems programming - no claims of perfection, just exploration.

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
