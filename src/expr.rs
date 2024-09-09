use crate::error::*;
use crate::token::*;
use crate::object::*;
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}
impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Assign(expr) => expr.accept(visitor),
            Expr::Binary(expr) => expr.accept(visitor),
            Expr::Grouping(expr) => expr.accept(visitor),
            Expr::Literal(expr) => expr.accept(visitor),
            Expr::Logical(expr) => expr.accept(visitor),
            Expr::Unary(expr) => expr.accept(visitor),
            Expr::Variable(expr) => expr.accept(visitor),
        }
    }
}
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}
pub struct LiteralExpr {
    pub value: Option<Object>,
}
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct VariableExpr {
    pub name: Token,
}
pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<T,LoxError>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T,LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T,LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T,LoxError>;
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<T,LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T,LoxError>;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<T,LoxError>;
}
impl AssignExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_assign_expr(self)
    }
}
impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}
impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}
impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}
impl LogicalExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_logical_expr(self)
    }
}
impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}
impl VariableExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_variable_expr(self)
    }
}
