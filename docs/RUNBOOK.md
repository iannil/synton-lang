# Synton Development Runbook

This runbook covers deployment, monitoring, and common operational procedures for the Synton language toolchain.

## Table of Contents

1. [Building and Installation](#building-and-installation)
2. [Deployment Procedures](#deployment-procedures)
3. [Monitoring and Logging](#monitoring-and-logging)
4. [Common Issues and Fixes](#common-issues-and-fixes)
5. [Rollback Procedures](#rollback-procedures)

---

## Building and Installation

### Local Development Build

```bash
# Debug build
cargo build --all-features

# Release build (optimized)
cargo build --release --bin synton

# Install locally
cargo install --path cli
```

### Release Build

```bash
# Build release binary
make release

# The binary will be at:
# target/release/synton
```

---

## Deployment Procedures

### Publishing to crates.io

Each crate can be published independently:

```bash
# Publish AST crate (no dependencies)
cargo publish -p synton-ast

# Publish dependent crates in order
cargo publish -p synton-lexer
cargo publish -p synton-parser
cargo publish -p synton-typeck
cargo publish -p synton-contract
cargo publish -p synton-runtime
cargo publish -p synton-decompiler
cargo publish -p synton-lsp
```

### CI/CD Pipeline

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs on every push:

1. **Format Check**: Verifies code formatting
2. **Clippy**: Runs linter with `-D warnings`
3. **Tests**: Runs all tests with `--all-features`
4. **MSRV Check**: Verifies minimum supported Rust version

### Docker Deployment (Future)

```dockerfile
FROM rust:1.70 as builder
WORKDIR /usr/src/synton
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/synton/target/release/synton /usr/local/bin/
```

---

## Monitoring and Logging

### Tracing

Synton uses the `tracing` crate for structured logging:

```bash
# Set log level via environment variable
SYNTON_LOG=debug cargo run --bin synton -- check file.synton

# Log levels: error, warn, info, debug, trace
```

### LSP Server Logging

When running the LSP server, logs go to stderr:

```bash
synton lsp --stdio 2>&1 | tee lsp.log
```

### Performance Monitoring

```bash
# Run with timing information
time synton check examples/fib.synton

# Profile with flamegraph (requires flamegraph tool)
cargo flamegraph --bin synton -- check examples/fib.synton
```

---

## Common Issues and Fixes

### Issue: Z3 Build Failure

**Symptom**: Compilation fails with `z3-sys v0.8.1 build failed`

**Solution**: Z3 is an optional feature. Build without it:

```bash
cargo build --no-default-features
```

Or install Z3 development headers:
```bash
# macOS
brew install z3

# Ubuntu/Debian
apt-get install libz3-dev
```

### Issue: Parser Returns Empty AST

**Symptom**: `synton parse file.synton` returns empty module

**Cause**: Parser implementation is currently a stub

**Solution**: Use the lexer to verify tokenization:
```bash
echo '(+ 1 2)' | synton parse --format text
```

### Issue: Type Checker Reports Unknown Variable

**Symptom**: `error: undefined variable: 'x'`

**Solution**: Ensure variables are declared before use:
```synton
(let x:i32 = 42)  ; Declare first
(+ x 1)           ; Then use
```

### Issue: LSP Server Not Responding

**Symptoms**: Editor shows "LSP server not responding"

**Solutions**:
1. Ensure LSP feature is enabled: `cargo install --path cli --features lsp`
2. Check LSP logs in editor output
3. Restart language server in editor

---

## Rollback Procedures

### Reverting to Previous Version

```bash
# Uninstall current version
cargo uninstall synton

# Install from specific git tag
cargo install --git https://github.com/synton-lang/synton --tag v0.1.0
```

### Workspace Rollback

```bash
# Discard uncommitted changes
git reset --hard HEAD

# Revert to specific commit
git checkout <commit-hash>

# Clean build artifacts
make clean
cargo build --all-features
```

### Dependency Downgrade

If a dependency update causes issues:

```bash
# Update Cargo.toml to pin specific version
# Then update lockfile
cargo update -p <package-name>
```

---

## Health Checks

### Self-Test

```bash
# Run all checks
make check

# Run individual checks
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

### Example Program Verification

```bash
# Parse example
synton parse examples/fib.synton

# Type check example
synton check examples/fib.synton

# Run example
synton run examples/fib.synton
```

---

## Emergency Procedures

### Build Failure After Merge

1. Revert the commit: `git revert HEAD`
2. Force push to main: `git push --force`
3. Notify team via issue

### Dependency Security Vulnerability

1. Check for vulnerabilities: `cargo audit`
2. Update dependency: `cargo update -p <crate>`
3. Run tests: `cargo test --all-features`
4. Publish new patch release
