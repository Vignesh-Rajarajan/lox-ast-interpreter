use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::stmt::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, StmtVisitor, VarStmt};
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    //An environment typically stores variables and their values during program execution

    environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            // 1. The outermost RefCell allows for interior mutability,
            //    meaning we can change the contents of this structure even if we only have a shared reference to it.
            //    This is useful in situations where we need to modify the environment from different parts of the program.
            // 2. Inner Rc allows multiple parts of the program to share ownership of the same environment.
            //    When all references to this Rc are dropped, the memory it holds is automatically freed
            // The outer doll (RefCell) can be opened by anyone holding it.
            // Inside is a deed (Rc) that can be photocopied and shared.
            // The deed protects another openable doll (inner RefCell).
            // At the very center is your precious environment.
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }
    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }
    pub fn interpret(&self, stmt: &[Stmt]) -> bool {
        let mut had_error = false;
        for statement in stmt {
            if self.execute(statement).is_err() {
                had_error = true;
                break;
            }
        }
        had_error
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    fn execute_block(&self, stmts: &[Stmt], environment: Environment) -> Result<(), LoxError> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        let mut result = Ok(());
        for stmt in stmts {
            result = self.execute(stmt);
            if result.is_err() {
                break;
            }
        }
        self.environment.replace(previous);
        result
    }
}
impl StmtVisitor<()> for Interpreter {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxError> {
        let new_env = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, new_env)
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{:?}", value.to_string());
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            Object::Nil
        };
        self.environment.borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.to_string(), value);
        Ok(())
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 - n2)),
                _ => Err(LoxError::new(
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
                _ => Err(LoxError::new(
                    expr.operator.line,
                    "invalid expression:operands must be two numbers or two strings",
                )),
            },
            TokenType::Slash => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => {
                    if n2 == 0.0 {
                        Err(LoxError::new(expr.operator.line, "division by zero"))
                    } else {
                        Ok(Object::Number(n1 / n2))
                    }
                }
                _ => Err(LoxError::new(
                    expr.operator.line,
                    "invalid expression:operands must be numbers",
                )),
            },
            TokenType::Star => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 * n2)),
                _ => Err(LoxError::new(
                    expr.operator.line,
                    "invalid expression:operands must be numbers",
                )),
            },

            TokenType::Greater => {
                // if object are not of equal type return err
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left > right))
            }
            TokenType::Less => {
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left < right))
            }
            TokenType::GreaterEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left >= right))
            }
            TokenType::LessEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left <= right))
            }
            TokenType::BangEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left != right))
            }
            TokenType::EqualEqual => {
                if left.get_type() != right.get_type() {
                    return Err(LoxError::new(
                        expr.operator.line,
                        "invalid expression:operands are different types",
                    ));
                }
                Ok(Object::Bool(left == right))
            }
            _ => Err(LoxError::runtime_error(&expr.operator, "invalid operator")),
        }
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone().unwrap())
    }
    // for example: -1
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(LoxError::new(
                    expr.operator.line,
                    "Operand must be a number",
                )),
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxError::new(expr.operator.line, "unreachable")),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError> {
        self.environment.borrow().borrow().get(&expr.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn make_literal(o: Object) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    #[test]
    fn test_unary_minus() {
        let expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal(Object::Number(4.0)),
        };
        let interpreter = Interpreter::new();
        let result = interpreter.visit_unary_expr(&expr);
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
        let result = interpreter.visit_unary_expr(&expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(7.0));
    }

    #[test]
    fn test_binary_addition_string() {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::String("hello".to_string())),
            })),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::String("world".to_string())),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

        let result1 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

        let result2 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

        let result3 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

        let result4 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result4.is_ok());
        assert_eq!(result4.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

        let result5 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result5.is_ok());
        assert_eq!(result5.unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_binary_greater_lesser_greater_equal_string() {
        let mut binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::String("def".to_string())),
            })),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::String("abc".to_string())),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::Less, "<".to_string(), None, 1);

        let result1 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1);

        let result2 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Object::Bool(true));

        binary_expr.operator = Token::new(TokenType::LessEqual, "<=".to_string(), None, 1);

        let result3 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::EqualEqual, "==".to_string(), None, 1);

        let result4 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result4.is_ok());
        assert_eq!(result4.unwrap(), Object::Bool(false));

        binary_expr.operator = Token::new(TokenType::BangEqual, "!=".to_string(), None, 1);

        let result5 = interpreter.visit_binary_expr(&binary_expr);
        assert!(result5.is_ok());
        assert_eq!(result5.unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_binary_nil() {
        let mut binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&binary_expr);
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
        let result = interpreter.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_statement() {
        let interpreter = Interpreter::new();
        let var_stmt = VarStmt {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            initializer: Some(*make_literal(Object::Number(4.0))),
        };
        let result = interpreter.visit_var_stmt(&var_stmt);
        assert!(result.is_ok());
        let val = interpreter.environment.borrow().borrow().get(&var_stmt.name);
        assert_eq!(val.unwrap(), Object::Number(4.0));
    }
    #[test]
    fn test_var_statement_undefined() {
        let interpreter = Interpreter::new();
        let var_stmt = VarStmt {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            initializer: None,
        };
        let result = interpreter.visit_var_stmt(&var_stmt);
        assert!(result.is_ok());
        let val = interpreter.environment.borrow().borrow().get(&var_stmt.name);
        assert_eq!(val.unwrap(), Object::Nil);
    }
    #[test]
    fn test_var_expr() {
        let interpreter = Interpreter::new();
        let var_stmt = VarStmt {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            initializer: Some(*make_literal(Object::Number(4.0))),
        };
        let result = interpreter.visit_var_stmt(&var_stmt);
        assert!(result.is_ok());
        let var_expr = VariableExpr {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
        };
        let val = interpreter.visit_variable_expr(&var_expr);
        assert_eq!(val.unwrap(), Object::Number(4.0));
    }
    #[test]
    fn test_var_expr_undefined() {
        let interpreter = Interpreter::new();
        let var_expr = VariableExpr {
            name: Token::new(TokenType::Identifier, "a".to_string(), None, 1),
        };
        let val = interpreter.visit_variable_expr(&var_expr);
        assert!(val.is_err())
    }
}
