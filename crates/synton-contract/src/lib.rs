//! # Synton Contract Verification
//!
//! Contract-based verification using Z3 SMT solver.
//!
//! Provides `@pre`, `@post`, and `@invariant` contract verification
//! to prevent AI hallucinations at compile time.

#![warn(missing_docs, unused_crate_dependencies)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use std::fmt;

pub mod dso;
pub mod solver;

pub use dso::{DebugStateObject, DsoBuilder};
pub use solver::{Solver, SolverResult, VerificationResult, DummySolver};

#[cfg(feature = "z3")]
pub use solver::Z3Solver;

/// Contract kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractKind {
    /// Precondition: `@pre(condition)`
    Pre,
    /// Postcondition: `@post(condition)`
    Post,
    /// Loop invariant: `@inv(condition)`
    Invariant,
    /// Assertion: `@assert(condition)`
    Assert,
}

/// A contract with constraint expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub kind: ContractKind,
    pub constraint: String,
    pub location: ContractLocation,
}

/// Location where a contract is applied
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractLocation {
    /// Function contract
    Fn { name: String },
    /// Loop invariant
    Loop,
    /// Assertion
    Inline,
}

/// Contract verifier
pub struct ContractVerifier {
    solver: Option<Box<dyn Solver>>,
    enabled: bool,
}

impl ContractVerifier {
    /// Create a new verifier (requires Z3 feature)
    #[cfg(feature = "z3")]
    pub fn new() -> Self {
        Self {
            solver: Some(Box::new(Z3Solver::new())),
            enabled: true,
        }
    }

    /// Create a disabled verifier (without Z3)
    #[cfg(not(feature = "z3"))]
    pub fn new() -> Self {
        Self::disabled()
    }

    /// Create a disabled verifier (without Z3)
    pub fn disabled() -> Self {
        Self {
            solver: Some(Box::new(DummySolver)),
            enabled: false,
        }
    }

    /// Check if verification is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Verify a precondition
    pub fn verify_pre(&self, contract: &Contract) -> VerificationResult {
        if let Some(solver) = &self.solver {
            solver.assert(&contract.constraint)
        } else {
            VerificationResult::Skipped
        }
    }

    /// Verify a postcondition
    pub fn verify_post(&self, contract: &Contract, inputs: &Value) -> VerificationResult {
        if let Some(solver) = &self.solver {
            solver.check_with_inputs(&contract.constraint, inputs)
        } else {
            VerificationResult::Skipped
        }
    }

    /// Generate a Debug State Object for contract violation
    pub fn violation_dso(
        &self,
        contract: &Contract,
        context: &Value,
    ) -> DebugStateObject {
        DsoBuilder::new()
            .status("ContractViolation")
            .error_code(match contract.kind {
                ContractKind::Pre => "PRECONDITION_VIOLATION",
                ContractKind::Post => "POSTCONDITION_VIOLATION",
                ContractKind::Invariant => "INVARIANT_VIOLATION",
                ContractKind::Assert => "ASSERTION_FAILED",
            }.to_string())
            .expected(contract.constraint.clone())
            .context(context.clone())
            .build()
    }
}

impl Default for ContractVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for contract verification
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("solver error: {0}")]
    SolverError(String),

    #[error("invalid constraint expression: {0}")]
    InvalidConstraint(String),

    #[error("Z3 not available (compile with 'z3' feature)")]
    Z3NotAvailable,
}
