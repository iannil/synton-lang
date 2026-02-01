//! Memory management

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Memory error
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("out of memory: requested {requested}, available {available}")]
    OutOfMemory { requested: usize, available: usize },

    #[error("invalid address: {0}")]
    InvalidAddress(u32),

    #[error("access violation: address {address}, size {size}")]
    AccessViolation { address: u32, size: usize },
}

/// Memory cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryCell {
    Integer(i64),
    Float(f64),
    Bytes(Vec<u8>),
    String(String),
}

/// Runtime memory
pub struct Memory {
    cells: Vec<Option<MemoryCell>>,
    max_size: usize,
}

impl Memory {
    pub fn new(max_size: usize) -> Self {
        Self {
            cells: Vec::with_capacity(64),
            max_size,
        }
    }

    /// Allocate a new cell
    pub fn alloc(&mut self, cell: MemoryCell) -> Result<u32, MemoryError> {
        if self.cells.len() >= self.max_size {
            return Err(MemoryError::OutOfMemory {
                requested: 1,
                available: self.max_size - self.cells.len(),
            });
        }
        let addr = self.cells.len() as u32;
        self.cells.push(Some(cell));
        Ok(addr)
    }

    /// Read from memory
    pub fn read(&self, addr: u32) -> Result<&MemoryCell, MemoryError> {
        self.cells.get(addr as usize)
            .and_then(|c| c.as_ref())
            .ok_or_else(|| MemoryError::InvalidAddress(addr))
    }

    /// Write to memory
    pub fn write(&mut self, addr: u32, cell: MemoryCell) -> Result<(), MemoryError> {
        let slot = self.cells.get_mut(addr as usize)
            .ok_or_else(|| MemoryError::InvalidAddress(addr))?;
        *slot = Some(cell);
        Ok(())
    }

    /// Free a cell
    pub fn free(&mut self, addr: u32) -> Result<(), MemoryError> {
        let slot = self.cells.get_mut(addr as usize)
            .ok_or_else(|| MemoryError::InvalidAddress(addr))?;
        *slot = None;
        Ok(())
    }

    /// Collect garbage
    pub fn gc(&mut self, _roots: &[u32]) {
        // Simple mark-and-sweep could be implemented here
        // For now, just drop None cells
        self.cells.retain(|c| c.is_some());
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(1024)
    }
}
