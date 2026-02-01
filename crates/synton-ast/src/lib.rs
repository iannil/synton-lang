//! # Synton AST
//!
//! Abstract Syntax Tree definitions and operations for the Synton programming language.

#![warn(missing_docs, unused_crate_dependencies)]

pub mod id;
pub mod span;
pub mod types;
pub mod expr;
pub mod stmt;
pub mod op;

pub use id::{NodeId, VarId, FnId, ModuleId};
pub use span::{Span, Position};
pub use types::{Type, TypeKind, BuiltinType, RefinementType};
pub use expr::{Expr, ExprKind, Literal, Pattern, MatchArm, Param};
pub use stmt::{Stmt, StmtKind, FnDecl, StructDecl, EnumDecl, Contract, ContractKind, ContractLocation, StructField, EnumVariant, AssignTarget};
pub use op::{Op, UnaryOp, BinaryOp, CompareOp};

/// AST container for a complete Synton module
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Module {
    /// Module identifier (content-addressed hash)
    pub id: ModuleId,
    /// Top-level statements and declarations
    pub stmts: Vec<Stmt>,
    /// Import declarations
    pub imports: Vec<ImportDecl>,
    /// Export declarations
    pub exports: Vec<ExportDecl>,
}

impl Module {
    /// Create a new empty module
    pub fn new(id: ModuleId) -> Self {
        Self {
            id,
            stmts: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }

    /// Check if module is empty
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty() && self.imports.is_empty() && self.exports.is_empty()
    }
}

/// Import declaration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImportDecl {
    /// Import style (hash-based or semantic)
    pub kind: ImportKind,
    /// Local binding name
    pub name: String,
    /// Optional alias
    pub alias: Option<String>,
    pub span: Span,
}

/// Import style variants
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ImportKind {
    /// Content-addressed hash import: `import hash:sha256:abc123...`
    Hash {
        algorithm: String,
        hash: String,
    },
    /// Semantic import: `import sem:"sort algorithm fast"`
    Semantic {
        query: String,
    },
    /// Path import: `import "std/list"`
    Path {
        path: String,
    },
}

/// Export declaration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportDecl {
    /// Name being exported
    pub name: String,
    /// Optional export alias
    pub alias: Option<String>,
    pub span: Span,
}

/// Node kind identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NodeKind {
    // Declarations
    FnDecl,
    StructDecl,
    EnumDecl,
    TypeAlias,
    ConstDecl,
    LetDecl,

    // Statements
    ExprStmt,
    Assign,
    Block,
    If,
    Loop,
    While,
    For,
    Break,
    Continue,
    Return,

    // Expressions
    Literal,
    Var,
    Unary,
    Binary,
    Call,
    Index,
    Field,
    ArrayExpr,
    TupleExpr,
    StructExpr,
    Lambda,
    IfExpr,
    BlockExpr,

    // Types
    Builtin,
    Refinement,
    ArrayType,
    TupleType,
    StructType,
    EnumType,
    FnType,
}
