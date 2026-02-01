//! Type inference

use crate::{TResult, TypeChecker, TypeEnv, type_from_literal};
use synton_ast::{Expr, Type};

/// Type inference engine
pub struct TypeInfer {
    env: TypeEnv,
    next_ty_var: u32,
}

impl TypeInfer {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
            next_ty_var: 0,
        }
    }

    /// Fresh type variable
    pub fn fresh_var(&mut self) -> Type {
        let id = self.next_ty_var;
        self.next_ty_var += 1;
        Type::new(synton_ast::TypeKind::Inference(id), synton_ast::Span::new(
            synton_ast::Position::start(),
            synton_ast::Position::start(),
        ))
    }

    /// Infer type of expression
    pub fn infer(&mut self, expr: &Expr) -> TResult<Type> {
        match &expr.kind {
            synton_ast::ExprKind::Literal(lit) => {
                Ok(type_from_literal(lit, expr.span))
            }
            synton_ast::ExprKind::Var { name, .. } => {
                self.env.get(name)
                    .map(|b| b.ty.clone())
                    .ok_or_else(|| crate::error::TypeError::UndefinedVar {
                        name: name.clone(),
                    })
            }
            _ => Ok(self.fresh_var()),
        }
    }
}

impl Default for TypeInfer {
    fn default() -> Self {
        Self::new()
    }
}
