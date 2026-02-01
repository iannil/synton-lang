//! Statement definitions

use serde::{Deserialize, Serialize};
use super::{Expr, Type, Span};

/// Statement in Synton
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Statement kinds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StmtKind {
    /// Expression statement
    Expr(Box<Expr>),

    /// Let binding: `(let x: type = expr)`
    Let {
        name: String,
        id: Option<super::VarId>,
        ty: Option<Type>,
        init: Option<Box<Expr>>,
        mutable: bool,
    },

    /// Assignment: `(set x expr)`
    Assign {
        target: AssignTarget,
        value: Box<Expr>,
    },

    /// Block statement: `{ stmts }`
    Block(Vec<Stmt>),

    /// If statement: `(branch cond then else)`
    If {
        cond: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    /// Loop statement
    Loop {
        body: Box<Stmt>,
    },

    /// While loop: `(while cond body)`
    While {
        cond: Box<Expr>,
        body: Box<Stmt>,
    },

    /// For loop: `(for var in iter body)`
    For {
        var: String,
        iter: Box<Expr>,
        body: Box<Stmt>,
    },

    /// Break statement
    Break(Option<Box<Expr>>),

    /// Continue statement
    Continue,

    /// Return statement
    Return(Option<Box<Expr>>),

    /// Function declaration
    FnDecl(FnDecl),

    /// Struct declaration
    StructDecl(StructDecl),

    /// Enum declaration
    EnumDecl(EnumDecl),

    /// Type alias
    TypeAlias {
        name: String,
        params: Vec<String>,
        ty: Type,
    },

    /// Const declaration
    Const {
        name: String,
        ty: Type,
        value: Box<Expr>,
    },

    /// Contract annotation
    Contract(Contract),

    /// Empty statement
    Empty,

    /// Error sentinel
    Error,
}

/// Assignment target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignTarget {
    Var(String),
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        base: Box<Expr>,
        name: String,
    },
    Deref(Box<Expr>),
}

/// Function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnDecl {
    pub name: String,
    pub id: Option<super::FnId>,
    pub params: Vec<super::expr::Param>,
    pub ret_type: Option<Type>,
    pub body: Option<Box<Expr>>,
    pub contracts: Vec<Contract>,
    pub span: Span,
}

/// Contract for function verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub kind: ContractKind,
    pub expr: Expr,
}

/// Contract kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractKind {
    /// Precondition: `@pre(n >= 0)`
    Pre,
    /// Postcondition: `@post($ret >= n)`
    Post,
    /// Invariant: `@inv(len > 0)`
    Invariant,
    /// Assert
    Assert,
}

/// Contract location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractLocation {
    Fn { name: String },
    Loop,
    Inline,
}

/// Struct declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDecl {
    pub name: String,
    pub params: Vec<String>,
    pub fields: Vec<StructField>,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub types: Vec<Type>,
    pub span: Span,
}
