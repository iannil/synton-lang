//! Execution engine

use super::{RuntimeError, ExecutionResult, Bytecode, Stack, StackValue, StdLib};
use tracing::{debug, trace, warn};

/// Execution engine
pub struct Engine {
    stack: Stack,
    max_steps: Option<usize>,
}

impl Engine {
    pub fn new(config: super::RuntimeConfig) -> Self {
        Self {
            stack: Stack::new(1024),
            max_steps: config.max_steps,
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode, stdlib: &StdLib) -> ExecutionResult {
        self.run_with_inputs(bytecode, &[], stdlib)
    }

    pub fn run_with_inputs(
        &mut self,
        bytecode: &Bytecode,
        inputs: &[StackValue],
        stdlib: &StdLib,
    ) -> ExecutionResult {
        // Push inputs onto stack
        for val in inputs.iter().rev() {
            self.stack.push(val.clone());
        }

        let mut pc = 0;
        let mut steps = 0;

        while pc < bytecode.instructions().len() {
            // Check step limit
            if let Some(max) = self.max_steps {
                if steps >= max {
                    return ExecutionResult::Error {
                        code: "MAX_STEPS_EXCEEDED".to_string(),
                        message: format!("exceeded maximum steps: {}", max),
                        location: None,
                    };
                }
            }
            steps += 1;

            let instr = &bytecode.instructions()[pc];
            trace!("pc={}, instr={:?}", pc, instr.op);

            match self.execute_one(&instr.op, bytecode, stdlib, &mut pc) {
                Ok(ControlFlow::Continue) => {}
                Ok(ControlFlow::Halt(value)) => return ExecutionResult::Success(value),
                Err(e) => {
                    return ExecutionResult::Error {
                        code: error_code(&e),
                        message: e.to_string(),
                        location: Some(format!("pc={}", pc)),
                    };
                }
            }
        }

        ExecutionResult::Unit
    }

    fn execute_one(
        &mut self,
        op: &super::OpKind,
        bytecode: &Bytecode,
        stdlib: &StdLib,
        pc: &mut usize,
    ) -> Result<ControlFlow, RuntimeError> {
        match op {
            super::OpKind::Nop => {}
            super::OpKind::Const(idx) => {
                let c = bytecode.get_constant(*idx)
                    .ok_or_else(|| RuntimeError::InvalidOperation("constant index out of bounds".into()))?;
                self.stack.push(constant_to_value(c));
            }
            super::OpKind::Add => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.add(b)?);
            }
            super::OpKind::Sub => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.sub(b)?);
            }
            super::OpKind::Mul => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.mul(b)?);
            }
            super::OpKind::Div => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.div(b)?);
            }
            super::OpKind::Return => {
                let val = self.stack.pop().unwrap_or(StackValue::Unit);
                return Ok(ControlFlow::Halt(val));
            }
            super::OpKind::Print => {
                let val = self.stack.pop().unwrap_or(StackValue::Unit);
                println!("{}", val);
            }
            super::OpKind::Panic => {
                let msg = self.stack.pop()
                    .ok()
                    .and_then(|v| v.as_string().ok())
                    .unwrap_or_else(|| "panic".to_string());
                return Err(RuntimeError::Panic(msg));
            }
            _ => {
                warn!("unimplemented opcode: {:?}", op);
            }
        }
        *pc += 1;
        Ok(ControlFlow::Continue)
    }
}

enum ControlFlow {
    Continue,
    Halt(StackValue),
}

fn error_code(e: &RuntimeError) -> String {
    match e {
        RuntimeError::DivisionByZero => "DIVISION_BY_ZERO".to_string(),
        RuntimeError::IndexOutOfBounds { .. } => "INDEX_OUT_OF_BOUNDS".to_string(),
        RuntimeError::StackOverflow => "STACK_OVERFLOW".to_string(),
        RuntimeError::StackUnderflow => "STACK_UNDERFLOW".to_string(),
        RuntimeError::MaxStepsExceeded => "MAX_STEPS_EXCEEDED".to_string(),
        RuntimeError::ConstraintViolation(_) => "CONSTRAINT_VIOLATION".to_string(),
        _ => "RUNTIME_ERROR".to_string(),
    }
}

fn constant_to_value(c: &super::Constant) -> StackValue {
    match c {
        super::Constant::Integer(i) => StackValue::Integer(*i),
        super::Constant::Float(f) => StackValue::Float(*f),
        super::Constant::String(s) => StackValue::String(s.clone()),
        super::Constant::Bool(b) => StackValue::Bool(*b),
        super::Constant::Unit => StackValue::Unit,
    }
}
