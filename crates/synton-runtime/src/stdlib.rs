//! Standard library functions

use super::{StackValue, RuntimeError};
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// Standard library
#[derive(Clone)]
pub struct StdLib {
    functions: FxHashMap<String, StdLibFn>,
}

type StdLibFn = Arc<dyn Fn(&[StackValue]) -> Result<StackValue, RuntimeError> + Send + Sync>;

impl StdLib {
    pub fn new() -> Self {
        let mut functions: FxHashMap<String, StdLibFn> = FxHashMap::default();

        // Register built-in functions
        functions.insert(
            "print".to_string(),
            Arc::new(stdlib_print) as StdLibFn,
        );

        functions.insert(
            "len".to_string(),
            Arc::new(stdlib_len) as StdLibFn,
        );

        functions.insert(
            "abs".to_string(),
            Arc::new(stdlib_abs) as StdLibFn,
        );

        Self { functions }
    }

    /// Call a standard library function
    pub fn call(&self, name: &str, args: &[StackValue]) -> Result<StackValue, RuntimeError> {
        self.functions.get(name)
            .ok_or_else(|| RuntimeError::UndefinedFunction(name.to_string()))
            .and_then(|f| f(args))
    }

    /// Check if a function exists
    pub fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// List all registered functions
    pub fn functions(&self) -> impl Iterator<Item = &str> {
        self.functions.keys().map(|s| s.as_str())
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}

fn stdlib_print(args: &[StackValue]) -> Result<StackValue, RuntimeError> {
    for arg in args {
        print!("{}", arg);
    }
    println!();
    Ok(StackValue::Unit)
}

fn stdlib_len(args: &[StackValue]) -> Result<StackValue, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::InvalidOperation("len requires at least one argument".to_string()));
    }
    match &args[0] {
        StackValue::String(s) => Ok(StackValue::Integer(s.len() as i64)),
        _ => Err(RuntimeError::TypeMismatch {
            expected: "string".to_string(),
            found: "other".to_string(),
        }),
    }
}

fn stdlib_abs(args: &[StackValue]) -> Result<StackValue, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::InvalidOperation("abs requires an argument".to_string()));
    }
    match args[0] {
        StackValue::Integer(i) => Ok(StackValue::Integer(i.abs())),
        StackValue::Float(f) => Ok(StackValue::Float(f.abs())),
        _ => Err(RuntimeError::TypeMismatch {
            expected: "numeric".to_string(),
            found: "non-numeric".to_string(),
        }),
    }
}
