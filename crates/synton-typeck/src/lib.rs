//! # Synton Type Checker
//!
//! Type checking with inference and refinement type support.

#![warn(missing_docs, unused_crate_dependencies)]

use thiserror::Error;
use synton_ast::{Module, Type, Expr, Stmt, BuiltinType, Span, Position, TypeKind};

pub mod error;
pub mod env;
pub mod infer;

pub use error::{TypeError, TResult};
pub use env::{TypeEnv, Binding};
pub use infer::TypeInfer;

/// Type checker configuration
#[derive(Debug, Clone)]
pub struct TypeCheckerConfig {
    /// Enable refinement type checking
    pub refinements: bool,
    /// Enable type inference
    pub inference: bool,
}

impl Default for TypeCheckerConfig {
    fn default() -> Self {
        Self {
            refinements: true,
            inference: true,
        }
    }
}

/// Main type checker
pub struct TypeChecker {
    config: TypeCheckerConfig,
    env: TypeEnv,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::with_config(TypeCheckerConfig::default())
    }

    pub fn with_config(config: TypeCheckerConfig) -> Self {
        Self {
            config,
            env: TypeEnv::new(),
        }
    }

    /// Check a module
    pub fn check_module(&mut self, module: &Module) -> TResult<()> {
        for stmt in &module.stmts {
            self.check_stmt(&stmt)?;
        }
        Ok(())
    }

    /// Check a statement
    pub fn check_stmt(&mut self, stmt: &Stmt) -> TResult<Type> {
        match &stmt.kind {
            synton_ast::StmtKind::Empty => Ok(unit_type(stmt.span)),
            synton_ast::StmtKind::Expr(expr) => self.check_expr(expr),
            synton_ast::StmtKind::Let { name, ty, init, .. } => {
                let init_ty = if let Some(init) = init {
                    self.check_expr(init)?
                } else {
                    return Err(TypeError::Uninitialized {
                        name: name.clone(),
                    });
                };

                if let Some(expected) = ty {
                    self.unify(&expected, &init_ty, stmt.span)?;
                }

                self.env.decl(name.clone(), init_ty);
                Ok(unit_type(stmt.span))
            }
            _ => Ok(unit_type(stmt.span)),
        }
    }

    /// Check an expression
    pub fn check_expr(&mut self, expr: &Expr) -> TResult<Type> {
        match &expr.kind {
            synton_ast::ExprKind::Error => Ok(unknown_type(expr.span)),
            synton_ast::ExprKind::Literal(lit) => Ok(type_from_literal(lit, expr.span)),
            _ => Ok(unknown_type(expr.span)),
        }
    }

    /// Unify two types
    pub fn unify(&mut self, expected: &Type, found: &Type, _span: Span) -> TResult<()> {
        if std::mem::discriminant(&expected.kind) != std::mem::discriminant(&found.kind) {
            return Err(TypeError::Mismatch {
                expected: format!("{:?}", expected.kind),
                found: format!("{:?}", found.kind),
            });
        }
        Ok(())
    }

    /// Infer the type of an expression
    pub fn infer_expr(&mut self, expr: &Expr) -> TResult<Type> {
        self.check_expr(expr)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to check a module
pub fn check(module: &Module) -> TResult<()> {
    TypeChecker::new().check_module(module)
}

/// Helper to create a unit type
pub fn unit_type(span: Span) -> Type {
    Type::new(TypeKind::Unit, span)
}

/// Helper to create an unknown/inference type
pub fn unknown_type(span: Span) -> Type {
    Type::new(TypeKind::Inference(0), span)
}

/// Helper to create a type from a literal
pub fn type_from_literal(lit: &synton_ast::Literal, span: Span) -> Type {
    let kind = match lit {
        synton_ast::Literal::Integer(_) => TypeKind::Builtin(BuiltinType::I32),
        synton_ast::Literal::Float(_) => TypeKind::Builtin(BuiltinType::F64),
        synton_ast::Literal::String(_) => TypeKind::Builtin(BuiltinType::String),
        synton_ast::Literal::Bool(_) => TypeKind::Builtin(BuiltinType::Bool),
        synton_ast::Literal::Char(_) => TypeKind::Builtin(BuiltinType::Char),
        _ => TypeKind::Inference(0),
    };
    Type::new(kind, span)
}
