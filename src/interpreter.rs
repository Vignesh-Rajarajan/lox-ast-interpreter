use crate::callable::{Callable, LoxCallable, NativeClock};
use crate::environment::Environment;
use crate::error::LoxResult;
use crate::expr::*;
use crate::function::LoxFunction;
use crate::object::Object;
use crate::stmt::{
    BlockStmt, BreakStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt,
    StmtVisitor, VarStmt, WhileStmt,
};
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::collections::HashMap;

use std::ops::Deref;
use std::rc::Rc;
use crate::token::Token;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    //An environment typically stores variables and their values during program execution
    environment: RefCell<Rc<RefCell<Environment>>>,
    nesting_level: RefCell<usize>,
    locals: RefCell<HashMap<Rc<Expr>, usize>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Func(Callable {
                func: Rc::new(NativeClock {}),
            }),
        );
        Self {
            environment: RefCell::new(Rc::clone(&globals)),
            nesting_level: RefCell::new(0),
            globals: Rc::clone(&globals),
            locals: RefCell::new(HashMap::new()),
        }
    }
    pub fn evaluate(&self, expr: Rc<Expr>) -> Result<Object, LoxResult> {
        expr.accept(expr.clone(), self)
    }
    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }
    pub fn interpret(&self, stmt: &[Rc<Stmt>]) -> bool {
        let mut had_error = false;
        for statement in stmt {
            if self.execute(statement.clone()).is_err() {
                had_error = true;
                break;
            }
        }
        had_error
    }

    fn execute(&self, stmt: Rc<Stmt>) -> Result<(), LoxResult> {
        stmt.accept(stmt.clone(), self)
    }

    pub fn execute_block(
        &self,
        statements: &Rc<Vec<Rc<Stmt>>>,
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement.clone()));

        self.environment.replace(previous);

        result
    }

    pub fn resolve(&self, expr: Rc<Expr>, depth: usize) {
        self.locals.borrow_mut().insert(expr, depth);
    }

    fn lookup_variable(&self, name: &Token, expr: Rc<Expr>) -> Result<Object, LoxResult> {
        if let Some(distance) = self.locals.borrow().get(&expr) {
            self.environment
                .borrow()
                .borrow()
                .get_at(*distance, &name.lexeme)
        }else{
            self.globals.borrow().get(name)
        }
    }
}
impl StmtVisitor<()> for Interpreter {
    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let new_env = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, new_env)
    }

    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch.clone())
        } else {
            Ok(())
        }
    }

    fn visit_expression_stmt(
        &self,
        wrapper: Rc<Stmt>,
        stmt: &ExpressionStmt,
    ) -> Result<(), LoxResult> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }

    fn visit_function_stmt(&self, wrapper: Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        let func = LoxFunction::new(stmt, self.environment.borrow().deref());
        self.environment.borrow().borrow_mut().define(
            stmt.name.lexeme.to_string(),
            Object::Func(Callable {
                func: Rc::new(func),
            }),
        );
        Ok(())
    }

    fn visit_break_stmt(&self, wrapper: Rc<Stmt>, stmt: &BreakStmt) -> Result<(), LoxResult> {
        if *self.nesting_level.borrow() == 0 {
            return Err(LoxResult::runtime_error(
                &stmt.token.clone(),
                "Can't break outside of loop",
            ));
        }
        Err(LoxResult::Break)
    }

    fn visit_print_stmt(&self, wrapper: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{:?}", value.to_string());
        Ok(())
    }

    fn visit_return_stmt(&self, wrapper: Rc<Stmt>, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        if let Some(value) = &stmt.value {
            Err(LoxResult::return_value(self.evaluate(value.clone())?))
        } else {
            Err(LoxResult::return_value(Object::Nil))
        }
    }

    fn visit_var_stmt(&self, _: Rc<Stmt>, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value = if let Some(initializer) = stmt.initializer.clone() {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.to_string(), value);
        Ok(())
    }

    fn visit_while_stmt(&self, wrapper: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), LoxResult> {
        *self.nesting_level.borrow_mut() += 1;
        while self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.body.clone())?;
        }
        *self.nesting_level.borrow_mut() -= 1;
        Ok(())
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<Object, LoxResult> {
        let value = self.evaluate(expr.value.clone())?;
        if let Some(distance) = self.locals.borrow().get(&wrapper) {
            self.environment
                .borrow()
                .borrow_mut()
                .assign_at(*distance, &expr.name, value.clone())?;
        }else{
            self.globals.borrow_mut().assign(&expr.name, value.clone())?;
        }
        Ok(value)
    }

    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;
        match expr.operator.ttype {
            TokenType::Minus => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 - n2)),
                _ => Err(LoxResult::new(
                    expr.operator.line,
                    "invalid expression: operands must be two numbers",
                )),
            },
            TokenType::Plus => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 + n2)),
                (Object::String(s1), Object::Number(n2)) => {
                    Ok(Object::String(format!("{}{}", s1, n2)))
                }
                (Object::Number(n1), Object::String(s2)) => {
                    Ok(Object::String(format!("{}{}", n1, s2)))
                }
                (Object::String(s1), Object::String(s2)) => {
                    Ok(Object::String(format!("{}{}", s1, s2)))
                }
                _ => Err(LoxResult::new(
                    expr.operator.line,
                    "invalid expression:operands must be two numbers or two strings",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => {
                    if n2 == 0.0 {
                        Err(LoxResult::new(expr.operator.line, "division by zero"))
                    } else {
                        Ok(Object::Number(n1 / n2))
                    }
                }
                _ => Err(LoxResult::new(
                    expr.operator.line,
                    "invalid expression:operands must be numbers",
                )),
            },
            TokenType::Star => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 * n2)),
                _ => Err(LoxResult::new(
                    expr.operator.line,
                    "invalid expression:operands must be numbers",
                )),
            },

            TokenType::Greater => {
                // if object are not of equal type return err
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left > right))
            }
            TokenType::Less => {
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left < right))
            }
            TokenType::GreaterEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left >= right))
            }
            TokenType::LessEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left <= right))
            }
            TokenType::BangEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left != right))
            }
            TokenType::EqualEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxResult::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left == right))
            }
            _ => Err(LoxResult::runtime_error(&expr.operator, "invalid operator")),
        }
    }

    fn visit_call_expr(&self, _: Rc<Expr>, expr: &CallExpr) -> Result<Object, LoxResult> {
        let callee = self.evaluate(expr.callee.clone())?;
        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            arguments.push(self.evaluate(arg.clone())?);
        }
        if let Object::Func(function) = callee {
            if arguments.len() != function.arity() {
                return Err(LoxResult::runtime_error(
                    &expr.paren,
                    &format!(
                        "expected {} arguments but got {}",
                        function.arity(),
                        arguments.len()
                    ),
                ));
            }
            function.func.call(self, arguments)
        } else {
            Err(LoxResult::runtime_error(
                &expr.paren,
                "can only call functions and classes",
            ))
        }
    }

    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<Object, LoxResult> {
        self.evaluate(expr.expression.clone())
    }
    fn visit_literal_expr(&self, _: Rc<Expr>, expr: &LiteralExpr) -> Result<Object, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;
        if expr.operator.ttype == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }
        self.evaluate(expr.right.clone())
    }

    // for example: -1
    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<Object, LoxResult> {
        let right = self.evaluate(expr.right.clone())?;
        match expr.operator.ttype {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(LoxResult::new(
                    expr.operator.line,
                    "Operand must be a number",
                )),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxResult::new(expr.operator.line, "unreachable")),
        }
    }

    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<Object, LoxResult> {
        self.lookup_variable(&expr.name, wrapper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn make_literal(o: Object) -> Rc<Expr> {
        Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: Some(o) })))
    }

    #[test]
    fn test_unary_minus() {
        let expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal(Object::Number(4.0)),
        };
        let interpreter = Interpreter::new();
        let result = interpreter.visit_unary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(-4.0));
    }
    #[test]
    fn test_unary_bang() {
        let expr = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 1),
            right: make_literal(Object::Bool(true)),
        };
        let interpreter = Interpreter::new();
        let result = interpreter.visit_unary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_binary_subtraction() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal(Object::Number(3.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(1.0));
    }
    #[test]
    fn test_binary_addition() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: make_literal(Object::Number(3.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(7.0));
    }

    #[test]
    fn test_binary_addition_string() {
        let binary_expr = BinaryExpr {
            left: Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::String("hello".to_string())),
            }))),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::String("world".to_string())),
            }))),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("helloworld".to_string()));
    }

    #[test]
    fn test_binary_slash() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 1),
            right: make_literal(Object::Number(2.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(2.0));
    }
    #[test]
    fn test_binary_slash_zero() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 1),
            right: make_literal(Object::Number(0.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_err());
        println!("{:?}", result.err().unwrap());
    }
    #[test]
    fn test_binary_star() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: make_literal(Object::Number(2.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(8.0));
    }

    #[test]
    fn test_binary_greater_lesser_greater_equal() {
        let mut binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 1),
            right: make_literal(Object::Number(2.0)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

        let result1 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

        let result2 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

        let result3 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

        let result4 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result4.is_ok());
        assert_eq!(result4.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

        let result5 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result5.is_ok());
        assert_eq!(result5.unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_binary_greater_lesser_greater_equal_string() {
        let mut binary_expr = BinaryExpr {
            left: Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::String("def".to_string())),
            }))),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 1),
            right: Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::String("abc".to_string())),
            }))),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

        let result1 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

        let result2 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

        let result3 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

        let result4 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result4.is_ok());
        assert_eq!(result4.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

        let result5 = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result5.is_ok());
        assert_eq!(result5.unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_binary_nil() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Nil),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            right: make_literal(Object::Nil),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_binary_error_case() {
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Number(4.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 1),
            right: make_literal(Object::Bool(true)),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &binary_expr,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_var_statement() {
        let interpreter = Interpreter::new();
        let var_stmt = VarStmt {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            initializer: Some(make_literal(Object::Number(4.0))),
        };
        let result = interpreter.visit_var_stmt(Rc::new(Stmt::Block(Rc::new(BlockStmt { statements: Rc::new(vec![]) }))), &var_stmt);
        assert!(result.is_ok());
        let val = interpreter
            .environment
            .borrow()
            .borrow()
            .get(&var_stmt.name);
        assert_eq!(val.unwrap(), Object::Number(4.0));
    }
    #[test]
    fn test_var_expr_undefined() {
        let interpreter = Interpreter::new();
        let var_expr = VariableExpr {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
        };
        let val = interpreter.visit_variable_expr(
            Rc::new(Expr::Literal(Rc::new(LiteralExpr { value: None }))),
            &var_expr,
        );
        assert!(val.is_err())
    }
}
