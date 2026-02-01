//! SMT solver interface

use serde_json::Value;

/// Result of verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    /// Constraint is satisfiable
    Sat,
    /// Constraint is unsatisfiable (proof of violation)
    Unsat,
    /// Solver couldn't determine
    Unknown,
    /// Verification was skipped (solver disabled)
    Skipped,
}

/// Solver trait for SMT solver abstraction
pub trait Solver: Send + Sync {
    /// Assert a constraint
    fn assert(&self, constraint: &str) -> VerificationResult;

    /// Check with input values
    fn check_with_inputs(&self, constraint: &str, inputs: &Value) -> VerificationResult;

    /// Push a new scope
    fn push(&self);

    /// Pop current scope
    fn pop(&self);

    /// Reset the solver state
    fn reset(&self);
}

/// Result type for solver operations
pub type SolverResult<T> = Result<T, SolverError>;

/// Solver error
#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    #[error("Z3 error: {0}")]
    Z3(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("timeout")]
    Timeout,
}

/// Dummy solver for when Z3 is not available
pub struct DummySolver;

impl Solver for DummySolver {
    fn assert(&self, _constraint: &str) -> VerificationResult {
        VerificationResult::Skipped
    }

    fn check_with_inputs(&self, _constraint: &str, _inputs: &Value) -> VerificationResult {
        VerificationResult::Skipped
    }

    fn push(&self) {}

    fn pop(&self) {}

    fn reset(&self) {}
}

#[cfg(feature = "z3")]
/// Z3 solver implementation
pub struct Z3Solver {
    _inner: std::sync::Arc<()>,
    // TODO: Add actual Z3 integration when available
}

#[cfg(feature = "z3")]
impl Z3Solver {
    pub fn new() -> Self {
        Self { _inner: std::sync::Arc::new(()) }
    }
}

#[cfg(feature = "z3")]
impl Solver for Z3Solver {
    fn assert(&self, _constraint: &str) -> VerificationResult {
        VerificationResult::Unknown
    }

    fn check_with_inputs(&self, _constraint: &str, _inputs: &Value) -> VerificationResult {
        VerificationResult::Unknown
    }

    fn push(&self) {}

    fn pop(&self) {}

    fn reset(&self) {}
}
