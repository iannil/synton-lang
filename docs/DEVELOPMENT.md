# Synton Development Guide

This document provides comprehensive information for developers working on the Synton programming language.

## Table of Contents

- [Quick Start](#quick-start)
- [Project Structure](#project-structure)
- [Available Commands](#available-commands)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [CI/CD](#cicd)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/synton-lang/synton.git
cd synton

# Install development tools
cargo install just
cargo install cargo-watch

# Build the project
just build

# Run tests
just test

# Start development
just dev
```

## Project Structure

```
synton-lang/
├── crates/                   # Workspace crates
│   ├── synton-ast/          # AST definitions
│   ├── synton-lexer/        # Lexical analysis (Logos)
│   ├── synton-parser/       # Parser (Chumsky)
│   ├── synton-typeck/       # Type checker
│   ├── synton-contract/     # Contract verifier (Z3)
│   ├── synton-runtime/      # Runtime VM
│   ├── synton-decompiler/   # Decompiler (Python/TS)
│   └── synton-lsp/          # Language Server Protocol
├── cli/                     # Command-line interface
├── docs/                    # Documentation
├── examples/                # Example programs
├── tests/                   # Integration tests
├── Cargo.toml               # Workspace configuration
├── justfile                 # Just commands
└── Makefile                 # Make commands
```

### Crate Overview

| Crate | Purpose | Status |
|-------|---------|--------|
| `synton-ast` | AST node definitions, types, operators | ✅ Complete |
| `synton-lexer` | Tokenization using Logos | ✅ Complete |
| `synton-parser` | Parser using Chumsky (Polish notation) | 🚧 In Progress |
| `synton-typeck` | Type checking and inference | ⏳ Pending |
| `synton-contract` | Z3 contract verification | ⏳ Pending |
| `synton-runtime` | Bytecode VM execution | ⏳ Pending |
| `synton-decompiler` | Python/TypeScript code generation | ⏳ Pending |
| `synton-lsp` | Language Server for editors | ⏳ Pending |

## Available Commands

The project supports both `just` (recommended) and `make` commands.

### Build Commands

```bash
# Build all crates (dev mode)
just build
make build

# Build release binary
just build-release
make release

# Install CLI locally
just install
make install
```

### Testing Commands

```bash
# Run all tests
just test
make test

# Watch mode (re-run on changes)
just test-watch
make test-watch

# Generate coverage report
make coverage
```

### Code Quality Commands

```bash
# Format code
just fmt-fix        # Apply formatting
make fmt-fix

just fmt            # Check formatting
make fmt

# Run linters
just clippy
make clippy

# Run all checks
just check
make check

# Development workflow
just dev            # Format + Clippy + Build
make dev
```

### Documentation Commands

```bash
# Generate and open documentation
just docs
make docs

# Generate with dependencies
make docs-deps
```

### Running the Language

```bash
# Start REPL
just repl
make repl

# Run example
just run-example fib
make example-fib

# Parse example (syntax check)
just parse-example fib

# Type check example
just check-example fib

# Decompile to Python/TypeScript
just decompile-example fib python
```

## Development Workflow

### 1. Setup Development Environment

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tools
cargo install just
cargo install cargo-watch
cargo install cargo-tarpaulin  # For coverage
```

### 2. Make Changes

```bash
# Watch for changes and rebuild
just watch

# Run tests in watch mode
just test-watch
```

### 3. Submit Changes

1. Format code: `just fmt-fix`
2. Run checks: `just check`
3. Commit changes
4. Push and create PR

## Testing

### Unit Tests

Each crate has its own unit tests:

```bash
# Test specific crate
cargo test -p synton-parser
cargo test -p synton-lexer
cargo test -p synton-ast
```

### Integration Tests

```bash
# Run all integration tests
cargo test --test '*'
```

### Test Organization

```
tests/
├── samples/
│   ├── valid/       # Valid syntax samples
│   └── invalid/     # Invalid syntax samples
└── integration/     # Integration tests
```

## Code Style

### Formatting

We use `rustfmt` with default configuration:

```bash
cargo fmt --all
```

### Linting

We use `clippy` with strict warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Conventions

- **Expression syntax**: Polish notation (prefix): `(+ 1 2)`
- **Statement syntax**: Parenthesized: `(let x = 42)`
- **Naming**:
  - Types: `PascalCase`
  - Functions: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
  - Macros: `snake_case!`

## CI/CD

### GitHub Actions

CI runs on:
- Every push to `main` or `master`
- Every pull request
- Manual workflow dispatch

### CI Checks

1. **Format check**: `cargo fmt --all -- --check`
2. **Clippy**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Tests**: `cargo test --all-features`
4. **Documentation**: `cargo doc --no-deps --all-features`

### Pre-commit Hooks

The project uses pre-commit hooks (`.pre-commit-config.yaml`):

```bash
# Install pre-commit hooks
pip install pre-commit
pre-commit install
```

## Performance Benchmarks

Run benchmarks:

```bash
just bench
make bench
```

### Benchmark Categories

- **Token efficiency**: Compare Synton vs Python token count
- **Parse speed**: MB/s parsing performance
- **Verification speed**: Z3 constraint checking time
- **Execution speed**: Runtime VM performance

## Adding Features

### 1. Add a New Expression Type

1. Update `synton-ast/src/expr.rs`: Add to `ExprKind`
2. Update `synton-parser/src/expr_parser.rs`: Add parsing logic
3. Add tests in `synton-parser/src/lib.rs`
4. Update documentation

### 2. Add a New Statement Type

1. Update `synton-ast/src/stmt.rs`: Add to `StmtKind`
2. Update `synton-parser/src/stmt_parser.rs`: Add parsing logic
3. Add tests
4. Update documentation

### 3. Add a New Token

1. Update `synton-lexer/src/lib.rs`: Add to `Token` enum
2. Update lexer helpers if needed
3. Update parser to use new token
4. Add tests

## Troubleshooting

### Build Errors

**Error**: "missing lifetime specifier"

**Solution**: Ensure parser functions use proper lifetime annotations:
```rust
pub fn parser<'a>() -> impl Parser<'a, &'a [TokenKind], T> + Clone + 'a
```

**Error**: "type mismatch"

**Solution**: Chumsky's `ParseResult` needs `.into_result()`:
```rust
parser.parse(tokens).into_result().map_err(|e| ...)
```

### Test Failures

**Error**: Parser returns `()`

**Solution**: Check `select!` macro patterns match token structure exactly.

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Chumsky Parser](https://github.com/zesterer/chumsky)
- [Logos Lexer](https://github.com/maciejhirsz/logos)
- [Z3 SMT Solver](https://github.com/Z3Prover/z3)

## Communication

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Discord**: [Invite link](https://discord.gg/synton)

## License

This project is licensed under MIT OR Apache-2.0.
