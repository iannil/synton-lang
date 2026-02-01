# Synton Code Map

This document provides a high-level overview of the Synton codebase structure and key files.

## Project Structure

```
synton-lang/
├── crates/                      # Core library crates
│   ├── synton-ast/             # Abstract Syntax Tree definitions
│   │   ├── src/lib.rs          # Main exports
│   │   ├── src/id.rs           # Node identifiers (NodeId, VarId, etc.)
│   │   ├── src/span.rs         # Source location tracking
│   │   ├── src/types.rs        # Type system definitions
│   │   ├── src/expr.rs         # Expression nodes
│   │   ├── src/stmt.rs         # Statement nodes
│   │   └── src/op.rs           # Operator definitions
│   │
│   ├── synton-lexer/           # Lexical analysis (tokenization)
│   │   └── src/lib.rs          # Logos-based lexer implementation
│   │
│   ├── synton-parser/          # Syntax analysis (parsing)
│   │   ├── src/lib.rs          # Main parser interface
│   │   ├── src/error.rs        # Parse error types
│   │   └── src/ast_builder.rs  # AST construction helpers
│   │
│   ├── synton-typeck/          # Type checking
│   │   ├── src/lib.rs          # Main type checker
│   │   ├── src/error.rs        # Type error definitions
│   │   ├── src/env.rs          # Type environment (scope tracking)
│   │   └── src/infer.rs        # Type inference engine
│   │
│   ├── synton-contract/        # Contract verification
│   │   ├── src/lib.rs          # Contract types and verifier
│   │   ├── src/solver.rs       # Z3 SMT solver interface
│   │   └── src/dso.rs          # Debug State Objects
│   │
│   ├── synton-runtime/         # Execution runtime
│   │   ├── src/lib.rs          # Main runtime API
│   │   ├── src/bytecode.rs     # Bytecode representation
│   │   ├── src/engine.rs       # Execution engine
│   │   ├── src/stack.rs        # Value stack
│   │   ├── src/memory.rs       # Memory management
│   │   └── src/stdlib.rs       # Standard library functions
│   │
│   ├── synton-decompiler/      # Code transpilation
│   │   ├── src/lib.rs          # Main decompiler API
│   │   ├── src/python.rs       # Python code generation
│   │   ├── src/typescript.rs   # TypeScript code generation
│   │   └── src/codegen.rs      # Code generation utilities
│   │
│   └── synton-lsp/             # Language Server Protocol
│       ├── src/lib.rs          # LSP server implementation
│       ├── src/diagnostics.rs  # Diagnostic generation
│       ├── src/completion.rs   # Auto-completion
│       ├── src/hover.rs        # Hover information
│       └── src/symbols.rs      # Document symbols
│
├── cli/                        # Command-line interface
│   ├── src/main.rs             # CLI entry point
│   ├── src/commands.rs         # Subcommand implementations
│   ├── src/repl.rs             # REPL implementation
│   └── src/output.rs           # Output formatting
│
├── runtime-wasm/               # WebAssembly runtime (future)
│
├── examples/                   # Example Synton programs
│   └── fib.synton              # Fibonacci example
│
├── tests/                      # Integration tests
│
├── .github/workflows/          # CI/CD workflows
│   └── ci.yml                  # Main CI workflow
│
├── docs/                       # Documentation
│   ├── CONTRIBUTING.md         # Contribution guidelines
│   ├── RUNBOOK.md              # Operational procedures
│   └── CODEMAP.md              # This file
│
├── Cargo.toml                  # Workspace configuration
├── Cargo.lock                  # Dependency lock file
├── Makefile                    # Build commands (make)
├── justfile                    # Build commands (just)
├── rustfmt.toml                # Formatting configuration
├── clippy.toml                 # Lint configuration
├── .pre-commit-config.yaml     # Pre-commit hooks
└── README.md                   # Project overview
```

## Key Modules

### Type System Flow

```
synton-ast (types.rs)
    ↓ defines
Type, TypeKind, BuiltinType
    ↓ used by
synton-typeck (lib.rs, env.rs)
    ↓ performs checking on
Module, Stmt, Expr (from AST)
    ↓ produces
TResult<Type>
```

### Compilation Pipeline

```
Source Code
    ↓ lex (synton-lexer)
Tokens
    ↓ parse (synton-parser)
AST (Module)
    ↓ type check (synton-typeck)
Typed AST
    ↓ verify (synton-contract)
Verified AST
    ↓ compile (future)
Bytecode
    ↓ execute (synton-runtime)
Result / DSO
```

### LSP Architecture

```
Client Request
    ↓
synton-lsp (lib.rs)
    ├── diagnostics.rs → Generate diagnostics
    ├── completion.rs → Provide completions
    ├── hover.rs → Hover info
    └── symbols.rs → Document symbols
```

## Important File Locations

| Purpose | File |
|---------|------|
| Workspace config | `/Cargo.toml` |
| AST definitions | `/crates/synton-ast/src/` |
| Token definitions | `/crates/synton-lexer/src/lib.rs` |
| Parser entry | `/crates/synton-parser/src/lib.rs` |
| Type checker | `/crates/synton-typeck/src/lib.rs` |
| CLI entry | `/cli/src/main.rs` |
| CI workflow | `/.github/workflows/ci.yml` |

## Crate Dependencies

```
synton-ast (no workspace deps)
    ↑
    ├── synton-lexer
    ├── synton-parser ──→ synton-lexer, synton-ast
    ├── synton-typeck ──→ synton-ast, synton-parser
    ├── synton-contract ──→ synton-ast
    ├── synton-runtime ──→ synton-ast
    ├── synton-decompiler ──→ synton-ast
    ├── synton-lsp ──→ synton-ast, synton-parser, synton-typeck
    └── cli ──→ all of the above
```
