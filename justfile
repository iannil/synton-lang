# Justfile for Synton-lang development
# Install: cargo install just

default: help

help:
    @just --list

# Build all crates
build:
    cargo build --all-features

# Build release
build-release:
    cargo build --release --bin synton

# Run all checks
check: fmt clippy test

# Format code
fmt:
    cargo fmt --all -- --check

# Fix formatting
fmt-fix:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test --all-features

# Run tests with watch
test-watch:
    cargo watch -x test

# Clean build artifacts
clean:
    cargo clean
    rm -rf examples/output

# Generate documentation
docs:
    cargo doc --no-deps --all-features

# Run benchmarks
bench:
    cargo bench --all-features

# Install CLI locally
install:
    cargo install --path cli

# Run REPL
repl:
    cargo run --release --bin synton -- repl

# Watch for changes
watch:
    cargo watch -x build

# Development workflow
dev: fmt-fix clippy build

# Run example
run-example name:
    cargo run --release --bin synton -- run "examples/{{name}}.synton"

# Parse example (check syntax only)
parse-example name:
    cargo run --release --bin synton -- parse "examples/{{name}}.synton"

# Type check example
check-example name:
    cargo run --release --bin synton -- check "examples/{{name}}.synton"

# Decompile example
decompile-example name lang="python":
    cargo run --release --bin synton -- decompile "examples/{{name}}.synton" --lang {{lang}} -o "examples/output/{{name}}.{{lang}}"

# Full CI check
ci: fmt clippy test
