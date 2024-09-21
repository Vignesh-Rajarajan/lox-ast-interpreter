use crate::error::*;
use crate::token::*;
use crate::object::*;
use std::rc::Rc;
pub enum Expr {
    Assign(Rc<AssignExpr>),
    Binary(Rc<BinaryExpr>),
    Call(Rc<CallExpr>),
    Grouping(Rc<GroupingExpr>),
    Literal(Rc<LiteralExpr>),
    Logical(Rc<LogicalExpr>),
    Unary(Rc<UnaryExpr>),
    Variable(Rc<VariableExpr>),
}
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Assign(expr1), Expr::Assign(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Binary(expr1), Expr::Binary(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Call(expr1), Expr::Call(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Grouping(expr1), Expr::Grouping(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Literal(expr1), Expr::Literal(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Logical(expr1), Expr::Logical(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Unary(expr1), Expr::Unary(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Expr::Variable(expr1), Expr::Variable(expr2)) => Rc::ptr_eq(expr1, expr2),
          _=> false,
      }
  }
}

impl Eq for Expr{}

use std::hash::*;
impl Hash for Expr {
    fn hash<H: Hasher>(&self, hasher: &mut H)
    where H: Hasher, 
       { match self {
            Expr::Assign(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Binary(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Call(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Grouping(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Literal(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Logical(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Unary(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Expr::Variable(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
        }
    }
}
impl Expr {
    pub fn accept<T>(&self, wrapper: Rc<Expr>, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Expr::Assign(expr) => expr_visitor.visit_assign_expr(wrapper,&expr),
            Expr::Binary(expr) => expr_visitor.visit_binary_expr(wrapper,&expr),
            Expr::Call(expr) => expr_visitor.visit_call_expr(wrapper,&expr),
            Expr::Grouping(expr) => expr_visitor.visit_grouping_expr(wrapper,&expr),
            Expr::Literal(expr) => expr_visitor.visit_literal_expr(wrapper,&expr),
            Expr::Logical(expr) => expr_visitor.visit_logical_expr(wrapper,&expr),
            Expr::Unary(expr) => expr_visitor.visit_unary_expr(wrapper,&expr),
            Expr::Variable(expr) => expr_visitor.visit_variable_expr(wrapper,&expr),
        }
    }
}
pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}
pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<Expr>>,
}
pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}
pub struct LiteralExpr {
    pub value: Option<Object>,
}
pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}
pub struct VariableExpr {
    pub name: Token,
}
pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<T,LoxResult>;
    fn visit_binary_expr(&self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> Result<T,LoxResult>;
    fn visit_call_expr(&self, wrapper: Rc<Expr>, expr: &CallExpr) -> Result<T,LoxResult>;
    fn visit_grouping_expr(&self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> Result<T,LoxResult>;
    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> Result<T,LoxResult>;
    fn visit_logical_expr(&self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> Result<T,LoxResult>;
    fn visit_unary_expr(&self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> Result<T,LoxResult>;
    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<T,LoxResult>;
}
