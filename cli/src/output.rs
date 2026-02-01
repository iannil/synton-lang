//! Output formatting utilities

use synton_ast::{Module, Expr, Stmt, ImportDecl};

pub struct OutputFormatter {
    colored: bool,
    indent: usize,
}

impl OutputFormatter {
    pub fn new() -> Self {
        Self {
            colored: true,
            indent: 0,
        }
    }

    pub fn format_module(&mut self, module: &Module) -> String {
        let mut out = String::new();

        for import in &module.imports {
            out.push_str(&self.format_import(import));
        }

        for stmt in &module.stmts {
            out.push_str(&self.format_stmt(stmt));
        }

        out
    }

    fn format_import(&self, import: &ImportDecl) -> String {
        format!("import {}\n", import.name)
    }

    fn format_stmt(&mut self, stmt: &Stmt) -> String {
        match &stmt.kind {
            synton_ast::StmtKind::FnDecl(decl) => {
                self.format_fn_decl(decl)
            }
            synton_ast::StmtKind::Expr(expr) => {
                format!("{}\n", self.format_expr(expr))
            }
            _ => String::new(),
        }
    }

    fn format_fn_decl(&mut self, decl: &synton_ast::stmt::FnDecl) -> String {
        let params: Vec<String> = decl.params.iter()
            .map(|p| p.name.clone())
            .collect();

        format!(
            "!fn:{} [{}] -> {}\n{}\n",
            decl.name,
            params.join(" "),
            decl.ret_type.as_ref()
                .map(|t| format!("{:?}", t.kind))
                .unwrap_or_else(|| "?".to_string()),
            "  // TODO: implement body formatting"
        )
    }

    fn format_expr(&mut self, expr: &Expr) -> String {
        match &expr.kind {
            synton_ast::ExprKind::Literal(lit) => format!("{:?}", lit),
            synton_ast::ExprKind::Var { name, .. } => name.clone(),
            synton_ast::ExprKind::Binary { op, left, right } => {
                format!(
                    "({} {} {})",
                    op.token(),
                    self.format_expr(left),
                    self.format_expr(right)
                )
            }
            _ => "?".to_string(),
        }
    }
}

impl Default for OutputFormatter {
    fn default() -> Self {
        Self::new()
    }
}
