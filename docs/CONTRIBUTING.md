# Contributing to Synton

Thank you for your interest in contributing to Synton, an AI-native programming language designed for LLMs.

## Development Workflow

### Prerequisites

- Rust 1.70+ (edition 2021)
- Git
- Optional: `just` command runner (`cargo install just`)
- Optional: Z3 solver (for contract verification features)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/synton-lang/synton.git
cd synton

# Install development tools
rustup component add rustfmt clippy rust-analyzer

# Build all crates
cargo build --all-features

# Run tests
cargo test --all-features

# Install CLI locally
cargo install --path cli
```

### Available Commands (via Make)

| Command | Description |
|---------|-------------|
| `make help` | Show all available commands |
| `make build` | Build all crates |
| `make check` | Run full check (format, clippy, test) |
| `make test` | Run tests |
| `make test-watch` | Run tests in watch mode |
| `make fmt` | Check code formatting |
| `make fmt-fix` | Fix formatting issues |
| `make clippy` | Run linter |
| `make clean` | Clean build artifacts |
| `make docs` | Generate and open documentation |
| `make run` | Run CLI with args |
| `make install` | Install CLI locally |
| `make repl` | Start REPL |
| `make dev` | Format and check (development workflow) |
| `make watch` | Watch for changes and rebuild |

### Available Commands (via Just)

```bash
just                    # Show all available commands
just build              # Build all crates
just build-release      # Build release binary
just check              # Run all checks
just fmt                # Check formatting
just fmt-fix            # Fix formatting
just clippy             # Run linter
just test               # Run tests
just clean              # Clean build artifacts
just docs               # Generate documentation
just install            # Install CLI locally
just repl               # Start REPL
just dev                # Development workflow
```

### Environment Setup

#### Feature Flags

- `z3` - Enable Z3 SMT solver for contract verification (default: off)
- `lsp` - Enable LSP server support (CLI default: off)

```bash
# Build with Z3 support
cargo build --features z3

# Build CLI with LSP support
cargo build --bin synton --features lsp

# Build all features
cargo build --all-features
```

#### Pre-commit Hooks

```bash
# Install pre-commit hooks (requires pre-commit installed)
pre-commit install
```

### Project Structure

```
synton-lang/
├── crates/
│   ├── synton-ast/          # AST definitions
│   ├── synton-lexer/        # Tokenizer (Logos)
│   ├── synton-parser/       # Parser (Chumsky)
│   ├── synton-typeck/       # Type checker
│   ├── synton-contract/     # Contract verification (Z3)
│   ├── synton-runtime/      # WebAssembly runtime
│   ├── synton-decompiler/   # Transpiler to Python/TS
│   └── synton-lsp/          # Language Server Protocol
├── cli/                     # Command-line tool
├── runtime-wasm/            # WebAssembly runtime components
├── examples/                # Example Synton programs
└── tests/                   # Integration tests
```

### Testing Procedures

```bash
# Run all tests
cargo test --all-features

# Run tests for specific crate
cargo test -p synton-typeck

# Run tests with output
cargo test --all-features -- --nocapture

# Run tests in watch mode
cargo watch -x test

# Generate coverage report
make coverage  # or: cargo tarpaulin --out Html --all-features
```

### Code Style

- Use `rustfmt` for formatting (configured in `rustfmt.toml`)
- Follow clippy lints (configured in `clippy.toml`)
- Maximum line width: 100 characters
- Indentation: 2 spaces

### Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(parser): implement prefix expression parsing

Add support for Polish notation in expression parsing.
Handles unary and binary operators with proper precedence.
```

### Pull Request Process

1. Fork and create a feature branch
2. Make your changes with `cargo fmt` and `cargo clippy`
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test --all-features`
5. Submit a pull request with description

### Getting Help

- GitHub Issues: https://github.com/synton-lang/synton/issues
- Documentation: https://docs.synton-lang.org
- Discord: (coming soon)
