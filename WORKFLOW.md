# Development Workflow

## Daily Development Cycle

### Starting a Session
```bash
# Navigate to working directory  
cd /Users/biju09/Downloads/rust-course-data/exercises/activities

# Check current state
git status
ls src/bin/ | tail -5  # See recent work
```

### Working on Implementation
```bash
# Development cycle
cargo run -q --bin a3    # Test implementation
cargo check             # Quick syntax check
cargo clippy && cargo fmt  # Quality check before commit
```

### Code Quality Standards
- All implementations pass `cargo clippy` without warnings
- Code formatted with `cargo fmt` before commits
- Use descriptive variable names and clear logic flow
- Handle errors explicitly (no unwrap() in production patterns)

## Professional Git Workflow

### Commit Message Format
```
implement [concept] for [purpose]

- [specific technical detail]
- [pattern or feature demonstrated]  
- [Rust idiom or best practice applied]
```

### Example Commits
```bash
# Good commit message
git commit -m "implement pattern matching for decision logic

- Add exhaustive match expressions for boolean evaluation
- Demonstrate compiler-enforced pattern coverage
- Apply idiomatic Rust control flow patterns"

# Avoid generic messages
git commit -m "completed exercise 3"  # Too vague
git commit -m "working on match"      # Not descriptive
```

### Repository Maintenance
```bash
# Regular cleanup
cargo clean            # Remove build artifacts when needed
git log --oneline -5   # Review recent progress

# Before pushing
cargo clippy           # Ensure code quality
cargo fmt              # Consistent formatting
git diff --cached      # Review changes before commit
```

## Code Organization Principles

### Function Design
- Keep functions small and focused (single responsibility)
- Use descriptive function names that explain purpose
- Separate computation logic from display/output logic
- Prefer returning values over printing directly (when possible)

### Error Handling Patterns
- Use `Result<T, E>` for operations that can fail
- Handle errors explicitly rather than panicking
- Provide meaningful error messages for debugging

### Rust Conventions
- Use snake_case for functions and variables
- Use PascalCase for types and structs  
- Follow ownership rules (prefer borrowing over cloning)
- Leverage the type system for compile-time guarantees

## Code Quality

- Code compiles without warnings
- Functions have clear, descriptive names
- Errors handled explicitly (no panics in production code)
- Follows Rust conventions (snake_case, borrowing over cloning)

---

*Consistent workflow produces reliable, maintainable Rust code*