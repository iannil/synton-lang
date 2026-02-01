//! Operators in Synton

use serde::{Deserialize, Serialize};

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
    BitNot,
    Ref,
    Deref,
}

impl UnaryOp {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Not => "!",
            Self::Neg => "-",
            Self::BitNot => "~",
            Self::Ref => "&",
            Self::Deref => "*",
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    And,
    Or,
}

impl BinaryOp {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Pow => "**",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::Shl => "<<",
            Self::Shr => ">>",
            Self::And => "&&",
            Self::Or => "||",
        }
    }

    pub const fn precedence(self) -> u8 {
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::Add | Self::Sub => 5,
            Self::Mul | Self::Div | Self::Mod => 6,
            Self::Pow => 7,
            Self::Shl | Self::Shr => 8,
            Self::BitAnd => 9,
            Self::BitXor => 10,
            Self::BitOr => 11,
        }
    }
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompareOp {
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

impl CompareOp {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Less => "<",
            Self::LessEq => "<=",
            Self::Greater => ">",
            Self::GreaterEq => ">=",
        }
    }
}

/// Operation wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    Unary(UnaryOp),
    Binary(BinaryOp),
    Compare(CompareOp),
}
