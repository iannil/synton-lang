//! Bytecode representation and instruction set

use serde::{Deserialize, Serialize};
use std::fmt;

/// A compiled bytecode program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bytecode {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    metadata: Metadata,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            instructions: Vec::with_capacity(capacity),
            constants: Vec::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn add_constant(&mut self, c: Constant) -> u32 {
        let idx = self.constants.len() as u32;
        self.constants.push(c);
        idx
    }

    pub fn get_constant(&self, idx: u32) -> Option<&Constant> {
        self.constants.get(idx as usize)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction in the bytecode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub op: OpKind,
    pub span: (u32, u32),
}

impl Instruction {
    pub fn new(op: OpKind) -> Self {
        Self { op, span: (0, 0) }
    }

    pub fn with_span(mut self, start: u32, end: u32) -> Self {
        self.span = (start, end);
        self
    }
}

/// Operation kinds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpKind {
    // No-ops
    Nop,

    // Constants
    Const(u32),

    // Stack operations
    Drop,
    Dup,
    Swap,
    Rot,

    // Local variables
    LoadLocal(u32),
    StoreLocal(u32),

    // Binary operations
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

    // Comparison
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,

    // Logical
    And,
    Or,
    Not,

    // Control flow
    Branch,
    BranchIf,
    Jump(u32),
    Loop(u32),
    Return,

    // Function calls
    Call(u32),
    CallIndirect,
    TailCall(u32),

    // Memory
    Load,
    Store,
    Alloc,

    // Arrays
    ArrayNew,
    ArrayGet,
    ArraySet,
    ArrayLen,

    // Contract checks
    CheckPre,
    CheckPost,
    Assert,

    // Builtins
    Print,
    Panic,
}

/// Constant values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Unit,
}

/// Bytecode metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub module_name: Option<String>,
    pub source_hash: Option<String>,
    pub debug_info: DebugInfo,
}

/// Debug information for bytecode
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugInfo {
    pub source_map: Vec<(u32, u32, u32)>, // (instr_offset, line, col)
    pub local_names: Vec<(u32, String)>,
    pub function_names: Vec<(u32, String)>,
}
