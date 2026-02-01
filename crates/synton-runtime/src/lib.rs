//! # Synton Runtime
//!
//! WebAssembly-based runtime for executing Synton programs.
//!
//! Uses wasmi for efficient sandboxed execution with optional
//! Wasmtime JIT compilation.

#![warn(missing_docs, unused_crate_dependencies)]

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument};
use std::collections::HashMap;

pub mod bytecode;
pub mod engine;
pub mod memory;
pub mod stack;
pub mod stdlib;

pub use bytecode::{Bytecode, Instruction, OpKind, Constant};
pub use engine::Engine;
pub use memory::{Memory, MemoryError};
pub use stack::{Stack, StackValue};
pub use stdlib::StdLib;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum memory in bytes
    pub max_memory: usize,
    /// Enable JIT compilation
    pub jit_enabled: bool,
    /// Maximum execution steps (for infinite loop protection)
    pub max_steps: Option<usize>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_memory: 16 * 1024 * 1024, // 16 MB
            jit_enabled: false,
            max_steps: Some(1_000_000),
        }
    }
}

/// Main runtime for Synton programs
pub struct Runtime {
    config: RuntimeConfig,
    engine: Engine,
    stdlib: StdLib,
}

impl Runtime {
    /// Create a new runtime with default config
    pub fn new() -> Self {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create a runtime with custom config
    pub fn with_config(config: RuntimeConfig) -> Self {
        Self {
            engine: Engine::new(config.clone()),
            stdlib: StdLib::new(),
            config,
        }
    }

    /// Execute bytecode
    #[instrument(skip(self, bytecode))]
    pub fn execute(&mut self, bytecode: &Bytecode) -> ExecutionResult {
        debug!("execiting bytecode: {} instructions", bytecode.instructions().len());
        self.engine.run(bytecode, &self.stdlib)
    }

    /// Execute with input values
    pub fn execute_with_inputs(
        &mut self,
        bytecode: &Bytecode,
        inputs: &[StackValue],
    ) -> ExecutionResult {
        self.engine.run_with_inputs(bytecode, inputs, &self.stdlib)
    }

    /// Get the current config
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Runtime error
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("index out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },

    #[error("stack overflow")]
    StackOverflow,

    #[error("stack underflow")]
    StackUnderflow,

    #[error("memory error: {0}")]
    Memory(#[from] MemoryError),

    #[error("maximum steps exceeded")]
    MaxStepsExceeded,

    #[error("invalid operation: {0}")]
    InvalidOperation(String),

    #[error("type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },

    #[error("undefined function: {0}")]
    UndefinedFunction(String),

    #[error("constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("panic: {0}")]
    Panic(String),
}

/// Execution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionResult {
    /// Successful execution with return value
    Success(StackValue),
    /// Execution returned unit (no value)
    Unit,
    /// Runtime error
    Error {
        code: String,
        message: String,
        location: Option<String>,
    },
}

impl ExecutionResult {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_) | Self::Unit)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn unwrap_value(self) -> Option<StackValue> {
        match self {
            Self::Success(v) => Some(v),
            _ => None,
        }
    }
}
