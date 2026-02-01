//! Expression definitions

use serde::{Deserialize, Serialize};
use super::{Type, Span};

/// Expression in Synton
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub id: Option<super::NodeId>,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span, id: None }
    }

    pub fn with_id(mut self, id: super::NodeId) -> Self {
        self.id = Some(id);
        self
    }
}

/// Expression kinds (using prefix notation / Polish notation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExprKind {
    /// Literal value
    Literal(Literal),

    /// Variable reference
    Var {
        id: Option<super::VarId>,
        name: String,
    },

    /// Unary operator: `(! x)`, `(- x)`
    Unary {
        op: super::UnaryOp,
        arg: Box<Expr>,
    },

    /// Binary operator: `(+ a b)`, `(* x y)`
    Binary {
        op: super::BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Comparison: `(<= a b)`
    Compare {
        op: super::CompareOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Function call: `(call:fn_name arg1 arg2)`
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    /// Method call: `(.call object method arg1)`
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    /// Array/slice indexing: `(.get array index)`
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },

    /// Field access: `(.field struct name)`
    Field {
        base: Box<Expr>,
        name: String,
    },

    /// Array literal: `[a, b, c]`
    Array(Vec<Expr>),

    /// Tuple literal: `(a, b, c)`
    Tuple(Vec<Expr>),

    /// Struct literal
    Struct {
        ty: String,
        fields: Vec<(String, Expr)>,
    },

    /// Lambda/anonymous function
    Lambda {
        params: Vec<Param>,
        body: Box<Expr>,
    },

    /// Block expression: `{ stmts; expr }`
    Block(Vec<super::Stmt>, Option<Box<Expr>>),

    /// If expression: `(branch cond then else)`
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },

    /// Loop expression
    Loop {
        body: Box<Expr>,
    },

    /// Break expression
    Break(Option<Box<Expr>>),

    /// Continue expression
    Continue,

    /// Return expression
    Return(Option<Box<Expr>>),

    /// Match expression
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
    },

    /// Maybe type constructor: `(some value)`, `(none)`
    Some(Box<Expr>),
    None,

    /// Type cast: `(as expr type)`
    As {
        expr: Box<Expr>,
        ty: Type,
    },

    /// Size of type: `(size_of type)`
    SizeOf(Type),

    /// Error sentinel for parse errors
    Error,
}

/// Parameter in function/lambda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
    pub span: Span,
}

/// Match arm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

/// Pattern for match/if-let
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard: `_`
    Wildcard,
    /// Literal pattern
    Literal(Literal),
    /// Variable binding
    Bind(String),
    /// Or pattern
    Or(Vec<Pattern>),
    /// Struct pattern
    Struct {
        ty: String,
        fields: Vec<(String, Pattern)>,
    },
    /// Tuple pattern
    Tuple(Vec<Pattern>),
    /// Slice pattern
    Slice(Vec<Pattern>),
    /// Some/None patterns
    Some(Box<Pattern>),
    None,
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// Integer: `42`, `-7`
    Integer(i64),
    /// Float: `3.14`, `-0.5`
    Float(f64),
    /// String: `"hello"`
    String(String),
    /// Character: `'a'`
    Char(char),
    /// Boolean: `true`, `false`
    Bool(bool),
    /// Byte: `b'A'`
    Byte(u8),
    /// Array of bytes: `b"hello"`
    Bytes(Vec<u8>),
    /// Unit: `()`
    Unit,
}

impl Literal {
    pub fn ty(&self) -> super::types::BuiltinType {
        match self {
            Self::Integer(_) => super::types::BuiltinType::I32,
            Self::Float(_) => super::types::BuiltinType::F64,
            Self::String(_) => super::types::BuiltinType::String,
            Self::Char(_) => super::types::BuiltinType::Char,
            Self::Bool(_) => super::types::BuiltinType::Bool,
            Self::Byte(_) => super::types::BuiltinType::U8,
            Self::Bytes(_) => super::types::BuiltinType::Dyn,
            Self::Unit => super::types::BuiltinType::Dyn,
        }
    }
}
