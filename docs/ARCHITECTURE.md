# Synton Language Architecture

This document describes the architecture of the Synton programming language implementation.

## Overview

Synton is a research programming language optimized for AI/LLM code generation. The implementation consists of multiple Rust crates organized as a Cargo workspace.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI (cli/)                          │
│  Build • Run • Verify • Decompile • REPL                   │
└────────────────────┬───────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
┌──────────────┐          ┌──────────────┐
│   Lexer      │          │  Decompiler  │
│ (Logos)      │          │  (Python/TS) │
└──────┬───────┘          └──────────────┘
       │
       ▼
┌──────────────┐
│   Parser     │
│  (Chumsky)   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│     AST      │
│  (synton-ast)│
└──────┬───────┘
       │
       ├─────────────┬──────────────┬─────────────┐
       │             │              │             │
       ▼             ▼              ▼             ▼
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│Type Check│  │ Contract │  │ Runtime  │  │   LSP    │
│          │  │ Verifier │  │    VM    │  │  Server  │
│(Z3 hints)│  │  (Z3)    │  │(Wasmtime)│  │(Tower)   │
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

## Crates

### 1. synton-ast

**Purpose**: Abstract Syntax Tree definitions

**Key Types**:
- `Expr`: Expressions (literals, variables, binary ops, calls, etc.)
- `Stmt`: Statements (let, if, while, loop, return, etc.)
- `Type`: Types (builtin, refinement, function, etc.)
- `Module`: Compilation unit

**File Structure**:
```
src/
├── lib.rs          # Public API
├── expr.rs         # Expression nodes
├── stmt.rs         # Statement nodes
├── types.rs        # Type definitions
├── op.rs           # Operators (BinaryOp, CompareOp)
└── span.rs         # Source locations
```

**Dependencies**: None (foundational crate)

### 2. synton-lexer

**Purpose**: Tokenization using Logos

**Key Function**:
```rust
pub fn tokenize(source: &str) -> LexResult<Vec<TokenKind>>
```

**Token Types**:
- Keywords: `fn`, `let`, `if`, `branch`, `while`, `loop`, etc.
- Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, etc.
- Delimiters: `(`, `)`, `[`, `]`, `{`, `}`, `;`, `,`, `:`
- Literals: Integers, Floats, Strings, Chars, Booleans
- Identifiers

**Dependencies**:
- `logos` = "0.14"

### 3. synton-parser

**Purpose**: Parse tokens to AST using Chumsky

**Key Functions**:
```rust
pub fn parse_module(source: &str) -> ParseResult<Module>
pub fn parse_expr(source: &str) -> ParseResult<Expr>
pub fn parse_stmt(source: &str) -> ParseResult<Stmt>
pub fn parse_type(source: &str) -> ParseResult<Type>
```

**Grammar**:
- Polish notation for expressions: `(+ 1 2)`
- Parenthesized statements: `(let x = 42)`
- Recursive definitions supported

**File Structure**:
```
src/
├── lib.rs          # Main parser interface
├── expr_parser.rs  # Expression parsing
├── stmt_parser.rs  # Statement parsing
├── grammar.rs      # Module-level grammar
├── error.rs        # Error types
└── ast_builder.rs  # AST construction helpers
```

**Dependencies**:
- `synton-lexer`
- `synton-ast`
- `chumsky` = "0.12"

### 4. synton-typeck

**Purpose**: Type checking and inference

**Key Features**:
- Hindley-Milner type inference
- Refinement types: `T where P`
- Maybe types: `maybe<T, confidence>`
- Tensor types: `tensor<T>[dims]`

**Key Function**:
```rust
pub fn type_check(module: &Module) -> Result<TypedModule, TypeError>
```

**Dependencies**:
- `synton-ast`
- `synton-parser`
- `rustc-hash`
- `indexmap`

### 5. synton-contract

**Purpose**: Contract verification using Z3 SMT solver

**Key Features**:
- Pre/post conditions: `@pre`, `@post`
- Constraint checking
- Counterexample generation
- Verification reports

**Key Function**:
```rust
pub fn verify_module(module: &Module) -> Result<VerifyReport, VerifyError>
```

**Dependencies**:
- `synton-ast`
- `z3` = "0.12"

### 6. synton-runtime

**Purpose**: Bytecode VM execution

**Key Features**:
- Stack-based VM
- WebAssembly support (Wasmtime/Wasmi)
- Structured error handling (DSO)
- Maybe value runtime

**Key Function**:
```rust
pub fn execute(bytecode: &[u8]) -> Result<Value, RuntimeError>
```

**Dependencies**:
- `synton-ast`
- `wasmtime` = "26.0"
- `wasmi` = "0.35"

### 7. synton-decompiler

**Purpose**: Generate Python/TypeScript from Synton AST

**Key Functions**:
```rust
pub fn to_python(ast: &Module) -> String
pub fn to_typescript(ast: &Module) -> String
```

**Dependencies**:
- `synton-ast`

### 8. synton-lsp

**Purpose**: Language Server Protocol implementation

**Key Features**:
- Code completion
- Go to definition
- Find references
- Diagnostics
- Document symbols

**Dependencies**:
- `synton-parser`
- `synton-typeck`
- `tower-lsp` = "0.20"
- `tokio`
- `async-trait`

### 9. cli

**Purpose**: Command-line interface

**Commands**:
```bash
synton build <file>          # Compile to binary AST
synton run <file>            # Execute compiled program
synton verify <file>         # Verify contracts only
synton decompile <file>      # Convert to Python/TS
synton repl                  # Interactive REPL
synton parse <file>          # Check syntax
synton check <file>          # Type check
```

**Dependencies**:
- `synton-parser`
- `synton-typeck`
- `synton-contract`
- `synton-runtime`
- `synton-decompiler`
- `clap` = { version = "4.5", features = ["derive"] }

## Data Flow

### Compilation Pipeline

```
Source Code (.synton)
    │
    ▼
┌─────────────┐
│   Lexer     │ → Vec<TokenKind>
└─────────────┘
    │
    ▼
┌─────────────┐
│   Parser    │ → Module (AST)
└─────────────┘
    │
    ├─────────────────┬────────────────┬──────────────┐
    │                 │                │              │
    ▼                 ▼                ▼              ▼
┌─────────┐    ┌──────────┐    ┌─────────┐    ┌──────────┐
│Type Check│    │ Contract │    │ Serialize│    │Decompile │
│         │    │ Verifier │    │ to Binary│    │ to Py/TS │
└────┬────┘    └──────────┘    └─────────┘    └──────────┘
     │
     ▼
┌─────────┐
│ Runtime │ → Output / DSO Error
└─────────┘
```

### Binary Format

```
[4 bytes] Magic: "SYTN"
[1 byte]  Version
[varint]   Node count
[varint]   Node table offset
[...]      Node table (types + children)
[...]      Literal table (compressed)
[...]      Metadata (spans, contracts)
```

## Language Features

### Expression Syntax (Polish Notation)

```synton
(+ 1 2)              # Binary: 1 + 2
(- x 5)              # Binary: x - 5
(* 2 3)              # Binary: 2 * 3
((+ 1 2) 3)          # Nested: (1 + 2) 3
```

### Statement Syntax

```synton
(let x = 42)                  # Variable binding
(branch cond then else)       # Conditional
(while cond body)             # Loop
(loop body)                   # Infinite loop
(break)                       # Break
(return expr)                 # Return
```

### Type System

```synton
i32, i64, f32, f64            # Builtin types
bool, string, char            # Builtin types
maybe<T, confidence>          # Probabilistic types
T where predicate             # Refinement types
tensor<T>[dims]              # Tensor types
```

### Contracts

```synton
!fn:divide [a:i32, b:i32] -> i32
  @pre(b != 0)
  @post($ret != inf)
  (/ a b)
```

## Error Handling

### Debug State Objects (DSO)

```json
{
  "status": "RuntimeError",
  "error_code": "CONSTRAINT_VIOLATION",
  "location": "node_id:45a",
  "context": {
    "variable_values": {"%1": 0, "%2": -5},
    "call_stack": [...],
    "memory_snapshot": "hash:..."
  },
  "suggestions": [
    {
      "description": "Check divisor is not zero",
      "code": "(if (!= b 0) (/ a b) 0)",
      "confidence": 0.95
    }
  ]
}
```

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Token Efficiency (vs Python) | >40% reduction | TBD |
| Parse Speed | >1 MB/s | TBD |
| Verification Time | <100ms (10 constraints) | TBD |
| Binary Compression | <30% of source | TBD |
| Test Coverage | >95% | ~60% |

## Future Enhancements

1. **Incremental Compilation**: Cache parsed ASTs
2. **Parallel Type Checking**: Use rayon for type inference
3. **JIT Compilation**: LLVM backend for hot paths
4. **REPL Improvements**: Auto-completion, syntax highlighting
5. **Package Manager**: Import management, versioning
6. **Standard Library**: Core data structures, algorithms

## Contributing

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed contribution guidelines.

## License

MIT OR Apache-2.0
