//! Type system definitions

use serde::{Deserialize, Serialize};
use super::Span;

/// A type in Synton
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub fn new(kind: TypeKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a builtin type
    pub fn builtin(ty: BuiltinType, span: Span) -> Self {
        Self::new(TypeKind::Builtin(ty), span)
    }

    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self.kind, TypeKind::Builtin(BuiltinType::I32 | BuiltinType::I64 | BuiltinType::F32 | BuiltinType::F64))
    }

    /// Check if type is signed integer
    pub fn is_signed_int(&self) -> bool {
        matches!(self.kind, TypeKind::Builtin(BuiltinType::I32 | BuiltinType::I64))
    }
}

/// Type kinds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    /// Builtin types: i32, f32, bool, string, etc.
    Builtin(BuiltinType),
    /// Refinement type with constraint: `int { val > 0 }`
    Refinement(Box<RefinementType>),
    /// Array type: `[T; n]` or `list<T>`
    List(Box<Type>),
    /// Tuple type: `(T1, T2, ...)`
    Tuple(Vec<Type>),
    /// Struct type
    Struct(Vec<StructField>),
    /// Enum type
    Enum(Vec<EnumVariant>),
    /// Function type: `(T1, T2) -> T3`
    Fn {
        params: Vec<Type>,
        ret: Box<Type>,
    },
    /// Optional type: `maybe<T>` or `?T`
    Maybe(Box<Type>),
    /// Result type: `result<T, E>`
    Result {
        ok: Box<Type>,
        err: Box<Type>,
    },
    /// Reference type: `&T`
    Ref(Box<Type>),
    /// Inference variable
    Inference(u32),
    /// Type variable (generic)
    Var(String),
    /// Never type
    Never,
    /// Unit type
    Unit,
}

/// Builtin types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuiltinType {
    /// Signed 32-bit integer
    I32,
    /// Signed 64-bit integer
    I64,
    /// Unsigned 32-bit integer
    U32,
    /// Unsigned 64-bit integer
    U64,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,
    /// Boolean
    Bool,
    /// String
    String,
    /// Byte
    U8,
    /// Character
    Char,
    /// Dynamic (any type)
    Dyn,
}

impl BuiltinType {
    pub const fn name(self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Bool => "bool",
            Self::String => "string",
            Self::U8 => "u8",
            Self::Char => "char",
            Self::Dyn => "dyn",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "i32" => Some(Self::I32),
            "i64" => Some(Self::I64),
            "u32" => Some(Self::U32),
            "u64" => Some(Self::U64),
            "f32" => Some(Self::F32),
            "f64" => Some(Self::F64),
            "bool" => Some(Self::Bool),
            "string" | "str" => Some(Self::String),
            "u8" | "byte" => Some(Self::U8),
            "char" => Some(Self::Char),
            "dyn" | "any" => Some(Self::Dyn),
            _ => None,
        }
    }

    pub const fn is_integer(self) -> bool {
        matches!(self, Self::I32 | Self::I64 | Self::U32 | Self::U64 | Self::U8)
    }

    pub const fn is_float(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }
}

/// Refinement type with contract constraint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RefinementType {
    /// Base type
    pub base: Box<Type>,
    /// Constraint expression
    pub constraint: String,
    /// Variable name in constraint (usually "val", "self", or "result")
    pub var_name: String,
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Enum variant definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub types: Vec<Type>,
    pub span: Span,
}
