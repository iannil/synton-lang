//! Python code generation

use super::{Codegen, CodegenConfig, IndentStyle};
use crate::codegen::CodeBuffer;
use synton_ast::{Module, Expr, Stmt, Type, ImportDecl};
use crate::DecompilerResult;

/// Python code generator
pub struct PythonCodegen {
    config: CodegenConfig,
}

impl PythonCodegen {
    pub fn new(config: CodegenConfig) -> Self {
        Self { config }
    }
}

impl Codegen for PythonCodegen {
    fn config(&self) -> &CodegenConfig {
        &self.config
    }

    fn gen_module(&self, module: &Module) -> DecompilerResult<String> {
        let mut buf = CodeBuffer::new(self.config.indent);

        for import in &module.imports {
            self.gen_import(&mut buf, import)?;
        }

        for stmt in &module.stmts {
            self.gen_stmt_buf(&mut buf, stmt)?;
        }

        Ok(buf.finish())
    }

    fn gen_expr(&self, expr: &Expr) -> DecompilerResult<String> {
        let mut buf = String::new();
        self.gen_expr_inner(&mut buf, expr)?;
        Ok(buf)
    }

    fn gen_stmt(&self, stmt: &Stmt) -> DecompilerResult<String> {
        let mut buf = CodeBuffer::new(self.config.indent);
        self.gen_stmt_buf(&mut buf, stmt)?;
        Ok(buf.finish())
    }

    fn gen_type(&self, ty: &Type) -> DecompilerResult<String> {
        Ok(match &ty.kind {
            synton_ast::TypeKind::Builtin(b) => match b {
                synton_ast::BuiltinType::I32 => "int",
                synton_ast::BuiltinType::I64 => "int",
                synton_ast::BuiltinType::F32 => "float",
                synton_ast::BuiltinType::F64 => "float",
                synton_ast::BuiltinType::Bool => "bool",
                synton_ast::BuiltinType::String => "str",
                _ => "Any",
            }.to_string(),
            _ => "Any".to_string(),
        })
    }
}

impl PythonCodegen {
    fn gen_import(&self, buf: &mut CodeBuffer, import: &ImportDecl) -> DecompilerResult<()> {
        buf.push_line(&format!("import {}", import.name));
        Ok(())
    }

    fn gen_stmt_buf(&self, buf: &mut CodeBuffer, stmt: &Stmt) -> DecompilerResult<()> {
        match &stmt.kind {
            synton_ast::StmtKind::FnDecl(decl) => {
                self.gen_fn_decl(buf, decl)?;
            }
            synton_ast::StmtKind::Let { name, ty, init, .. } => {
                let init_str = if let Some(e) = init {
                    self.gen_expr(e)?
                } else {
                    "None".to_string()
                };
                buf.push_line(&format!("{} = {}", name, init_str));
            }
            synton_ast::StmtKind::Expr(e) => {
                buf.push_line(&self.gen_expr(e)?);
            }
            _ => {}
        }
        Ok(())
    }

    fn gen_fn_decl(&self, buf: &mut CodeBuffer, decl: &synton_ast::stmt::FnDecl) -> DecompilerResult<()> {
        let params: Vec<String> = decl.params.iter()
            .map(|p| {
                if self.config.types {
                    let ty_str = p.ty.as_ref()
                        .map(|t| self.gen_type(t).unwrap_or_else(|_| "Any".to_string()))
                        .unwrap_or_else(|| "Any".to_string());
                    format!("{}: {}", p.name, ty_str)
                } else {
                    p.name.clone()
                }
            })
            .collect();

        let ret_str = if self.config.types {
            decl.ret_type.as_ref()
                .map(|t| format!(" -> {}", self.gen_type(t).unwrap_or_else(|_| "Any".to_string())))
                .unwrap_or_else(|| "".to_string())
        } else {
            String::new()
        };

        buf.push_line(&format!(
            "def {}({}){}:",
            decl.name,
            params.join(", "),
            ret_str
        ));

        buf.indent();

        if let Some(body) = &decl.body {
            buf.push_line(&self.gen_expr(body)?);
        } else {
            buf.push_line("pass");
        }

        buf.dedent();
        Ok(())
    }

    fn gen_expr_inner(&self, buf: &mut String, expr: &Expr) -> DecompilerResult<()> {
        match &expr.kind {
            synton_ast::ExprKind::Literal(lit) => {
                self.gen_literal(buf, lit);
            }
            synton_ast::ExprKind::Var { name, .. } => {
                buf.push_str(name);
            }
            synton_ast::ExprKind::Binary { op, left, right } => {
                buf.push_str("(");
                self.gen_expr_inner(buf, left)?;
                buf.push_str(" ");
                buf.push_str(op.token());
                buf.push_str(" ");
                self.gen_expr_inner(buf, right)?;
                buf.push_str(")");
            }
            synton_ast::ExprKind::Call { callee, args } => {
                self.gen_expr_inner(buf, callee)?;
                buf.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { buf.push_str(", "); }
                    self.gen_expr_inner(buf, arg)?;
                }
                buf.push_str(")");
            }
            synton_ast::ExprKind::If { cond, then_branch, else_branch } => {
                buf.push_str("(");
                self.gen_expr_inner(buf, cond)?;
                buf.push_str(" if ");
                self.gen_expr_inner(buf, then_branch)?;
                if let Some(else_br) = else_branch {
                    buf.push_str(" else ");
                    self.gen_expr_inner(buf, else_br)?;
                }
                buf.push_str(")");
            }
            _ => {
                buf.push_str("None");
            }
        }
        Ok(())
    }

    fn gen_literal(&self, buf: &mut String, lit: &synton_ast::expr::Literal) {
        match lit {
            synton_ast::expr::Literal::Integer(i) => buf.push_str(&i.to_string()),
            synton_ast::expr::Literal::Float(f) => buf.push_str(&f.to_string()),
            synton_ast::expr::Literal::String(s) => {
                buf.push_str("\"");
                buf.push_str(s);
                buf.push_str("\"");
            }
            synton_ast::expr::Literal::Bool(b) => buf.push_str(if *b { "True" } else { "False" }),
            synton_ast::expr::Literal::Unit => buf.push_str("None"),
            _ => {}
        }
    }
}
