//! Value stack

use serde::{Deserialize, Serialize};
use super::RuntimeError;

/// Stack value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StackValue {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Unit,
    // Reference to heap
    Ref(u32),
}

impl StackValue {
    pub fn as_integer(&self) -> Result<i64, RuntimeError> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "integer".to_string(),
                found: format!("{:?}", self),
            }),
        }
    }

    pub fn as_float(&self) -> Result<f64, RuntimeError> {
        match self {
            Self::Float(f) => Ok(*f),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "float".to_string(),
                found: format!("{:?}", self),
            }),
        }
    }

    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Self::Bool(b) => Ok(*b),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "bool".to_string(),
                found: format!("{:?}", self),
            }),
        }
    }

    pub fn as_string(&self) -> Result<String, RuntimeError> {
        match self {
            Self::String(s) => Ok(s.clone()),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "string".to_string(),
                found: format!("{:?}", self),
            }),
        }
    }

    pub fn add(self, other: StackValue) -> Result<StackValue, RuntimeError> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::String(a), Self::String(b)) => Ok(Self::String(format!("{}{}", a, b))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "compatible types".to_string(),
                found: "incompatible types".to_string(),
            }),
        }
    }

    pub fn sub(self, other: StackValue) -> Result<StackValue, RuntimeError> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                found: "non-numeric".to_string(),
            }),
        }
    }

    pub fn mul(self, other: StackValue) -> Result<StackValue, RuntimeError> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                found: "non-numeric".to_string(),
            }),
        }
    }

    pub fn div(self, other: StackValue) -> Result<StackValue, RuntimeError> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Self::Integer(a / b))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Self::Float(a / b))
                }
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                found: "non-numeric".to_string(),
            }),
        }
    }
}

impl std::fmt::Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Unit => write!(f, "()"),
            Self::Ref(r) => write!(f, "ref({})", r),
        }
    }
}

/// Execution stack
pub struct Stack {
    values: Vec<StackValue>,
    max_size: usize,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Self {
            values: Vec::with_capacity(64),
            max_size,
        }
    }

    pub fn push(&mut self, value: StackValue) -> Result<(), RuntimeError> {
        if self.values.len() >= self.max_size {
            return Err(RuntimeError::StackOverflow);
        }
        self.values.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<StackValue, RuntimeError> {
        self.values.pop()
            .ok_or_else(|| RuntimeError::StackUnderflow)
    }

    pub fn peek(&self, offset: usize) -> Result<&StackValue, RuntimeError> {
        self.values.get(self.values.len() - 1 - offset)
            .ok_or_else(|| RuntimeError::StackUnderflow)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}
