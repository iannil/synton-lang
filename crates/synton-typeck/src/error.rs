//! Type checker errors

use thiserror::Error;

/// Result type for type checker
pub type TResult<T> = Result<T, TypeError>;

/// Type error
#[derive(Error, Debug)]
pub enum TypeError {
    /// Type mismatch
    #[error("type mismatch: expected {expected}, found {found}")]
    Mismatch {
        expected: String,
        found: String,
    },

    /// Uninitialized variable
    #[error("uninitialized variable: '{name}'")]
    Uninitialized {
        name: String,
    },

    /// Undefined variable
    #[error("undefined variable: '{name}'")]
    UndefinedVar {
        name: String,
    },

    /// Undefined type
    #[error("undefined type: '{name}'")]
    UndefinedType {
        name: String,
    },

    /// Undefined function
    #[error("undefined function: '{name}'")]
    UndefinedFn {
        name: String,
    },

    /// Wrong number of arguments
    #[error("wrong number of arguments: expected {expected}, found {found}")]
    ArgCount {
        expected: usize,
        found: usize,
    },

    /// Cannot infer type
    #[error("cannot infer type for this expression")]
    CannotInfer,

    /// Refinement type violation
    #[error("refinement constraint violated: {constraint}")]
    RefinementViolation {
        constraint: String,
    },

    /// Cyclic type dependency
    #[error("cyclic type dependency")]
    Cycle,
}
