use crate::error::*;
use crate::expr::*;
use crate::token::*;
use std::rc::Rc;
pub enum Stmt {
    Block(Rc<BlockStmt>),
    If(Rc<IfStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    Break(Rc<BreakStmt>),
    Print(Rc<PrintStmt>),
    Return(Rc<ReturnStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
}
impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Stmt::Block(expr1), Stmt::Block(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::If(expr1), Stmt::If(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Expression(expr1), Stmt::Expression(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Function(expr1), Stmt::Function(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Break(expr1), Stmt::Break(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Print(expr1), Stmt::Print(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Return(expr1), Stmt::Return(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::Var(expr1), Stmt::Var(expr2)) => Rc::ptr_eq(expr1, expr2),
            (Stmt::While(expr1), Stmt::While(expr2)) => Rc::ptr_eq(expr1, expr2),
          _=> false,
      }
  }
}

impl Eq for Stmt{}

use std::hash::*;
impl Hash for Stmt {
    fn hash<H: Hasher>(&self, hasher: &mut H)
    where H: Hasher, 
       { match self {
            Stmt::Block(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::If(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Expression(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Function(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Break(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Print(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Return(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::Var(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
            Stmt::While(expr) => {hasher.write_usize(Rc::as_ptr(expr) as usize);}
        }
    }
}
impl Stmt {
    pub fn accept<T>(&self, wrapper: Rc<Stmt>, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(expr) => stmt_visitor.visit_block_stmt(wrapper,&expr),
            Stmt::If(expr) => stmt_visitor.visit_if_stmt(wrapper,&expr),
            Stmt::Expression(expr) => stmt_visitor.visit_expression_stmt(wrapper,&expr),
            Stmt::Function(expr) => stmt_visitor.visit_function_stmt(wrapper,&expr),
            Stmt::Break(expr) => stmt_visitor.visit_break_stmt(wrapper,&expr),
            Stmt::Print(expr) => stmt_visitor.visit_print_stmt(wrapper,&expr),
            Stmt::Return(expr) => stmt_visitor.visit_return_stmt(wrapper,&expr),
            Stmt::Var(expr) => stmt_visitor.visit_var_stmt(wrapper,&expr),
            Stmt::While(expr) => stmt_visitor.visit_while_stmt(wrapper,&expr),
        }
    }
}
pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}
pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}
pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}
pub struct BreakStmt {
    pub token: Token,
}
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}
pub struct ReturnStmt {
    pub token: Token,
    pub value: Option<Rc<Expr>>,
}
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}
pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<T,LoxResult>;
    fn visit_if_stmt(&self, wrapper: Rc<Stmt>, stmt: &IfStmt) -> Result<T,LoxResult>;
    fn visit_expression_stmt(&self, wrapper: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T,LoxResult>;
    fn visit_function_stmt(&self, wrapper: Rc<Stmt>, stmt: &FunctionStmt) -> Result<T,LoxResult>;
    fn visit_break_stmt(&self, wrapper: Rc<Stmt>, stmt: &BreakStmt) -> Result<T,LoxResult>;
    fn visit_print_stmt(&self, wrapper: Rc<Stmt>, stmt: &PrintStmt) -> Result<T,LoxResult>;
    fn visit_return_stmt(&self, wrapper: Rc<Stmt>, stmt: &ReturnStmt) -> Result<T,LoxResult>;
    fn visit_var_stmt(&self, wrapper: Rc<Stmt>, stmt: &VarStmt) -> Result<T,LoxResult>;
    fn visit_while_stmt(&self, wrapper: Rc<Stmt>, stmt: &WhileStmt) -> Result<T,LoxResult>;
}
