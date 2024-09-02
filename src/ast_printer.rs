use std::ops::BitXorAssign;

use crate::error::LoxError;
use crate::expr::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr};
pub(crate) struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: &[&Box<Expr>]) -> Result<String, LoxError> {
        let mut builder = format!("({} ", name);
        for expr in exprs {
            builder = format!("{} {}", builder, self.print(expr)?);
        }
        Ok(format!("{} )", builder))
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.lexeme.clone(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize("group".to_string(), &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        match &expr.value {
            Some(value) => Ok(value.to_string()),
            None => Ok("nil".to_string()),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.lexeme.clone(), &[&expr.right])
    }
}
