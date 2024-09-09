use crate::error::*;
use crate::expr::*;
use crate::token::*;
pub enum Stmt {
    Block(BlockStmt),
    If(IfStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    While(WhileStmt),
}
impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(expr) => expr.accept(visitor),
            Stmt::If(expr) => expr.accept(visitor),
            Stmt::Expression(expr) => expr.accept(visitor),
            Stmt::Print(expr) => expr.accept(visitor),
            Stmt::Var(expr) => expr.accept(visitor),
            Stmt::While(expr) => expr.accept(visitor),
        }
    }
}
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
pub struct ExpressionStmt {
    pub expression: Expr,
}
pub struct PrintStmt {
    pub expression: Expr,
}
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}
pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, expr: &BlockStmt) -> Result<T,LoxError>;
    fn visit_if_stmt(&self, expr: &IfStmt) -> Result<T,LoxError>;
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T,LoxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T,LoxError>;
    fn visit_var_stmt(&self, expr: &VarStmt) -> Result<T,LoxError>;
    fn visit_while_stmt(&self, expr: &WhileStmt) -> Result<T,LoxError>;
}
impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}
impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_if_stmt(self)
    }
}
impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}
impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}
impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}
impl WhileStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_while_stmt(self)
    }
}
