//! Type environment for tracking bindings

use rustc_hash::FxHashMap;
use synton_ast::{Type, VarId, FnId};
use std::collections::hash_map::Entry;

/// A binding in the type environment
#[derive(Debug, Clone)]
pub struct Binding {
    pub name: String,
    pub ty: Type,
    pub id: Option<VarId>,
}

/// Type environment
#[derive(Debug, Clone)]
pub struct TypeEnv {
    vars: Vec<FxHashMap<String, Binding>>,
    next_var_id: u32,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: vec![FxHashMap::default()],
            next_var_id: 0,
        }
    }

    /// Push a new scope
    pub fn push(&mut self) {
        self.vars.push(FxHashMap::default());
    }

    /// Pop the current scope
    pub fn pop(&mut self) {
        if self.vars.len() > 1 {
            self.vars.pop();
        }
    }

    /// Declare a variable in the current scope
    pub fn decl(&mut self, name: String, ty: Type) {
        let id = VarId(self.next_var_id);
        self.next_var_id += 1;

        if let Some(scope) = self.vars.last_mut() {
            scope.insert(name.clone(), Binding { name, ty, id: Some(id) });
        }
    }

    /// Look up a variable
    pub fn get(&self, name: &str) -> Option<&Binding> {
        for scope in self.vars.iter().rev() {
            if let Some(binding) = scope.get(name) {
                return Some(binding);
            }
        }
        None
    }

    /// Check if a name exists in any scope
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Update an existing variable's type
    pub fn update(&mut self, name: &str, ty: Type) -> bool {
        for scope in self.vars.iter_mut().rev() {
            if let Some(binding) = scope.get_mut(name) {
                binding.ty = ty;
                return true;
            }
        }
        false
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}
