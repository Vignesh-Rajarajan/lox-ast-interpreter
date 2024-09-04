use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::token_type::TokenType;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 - n2)),
                _ => Err(LoxError::new(expr.operator.line, "invalid expression: operands must be two numbers")),
            },
            TokenType::Plus => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 + n2)),
                (Object::String(s1), Object::String(s2)) => Ok(Object::String(format!("{}{}", s1, s2))),
                _ => Err(LoxError::new(expr.operator.line, "invalid expression:operands must be two numbers or two strings")),
            },
            TokenType::Slash => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => {
                    if n2 == 0.0 {
                        Err(LoxError::new(expr.operator.line, "division by zero"))
                    } else {
                        Ok(Object::Number(n1 / n2))
                    }
                }
                _ => Err(LoxError::new(expr.operator.line, "invalid expression:operands must be numbers")),
            },
            TokenType::Star => match (left, right) {
                (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 * n2)),
                _ => Err(LoxError::new(expr.operator.line, "invalid expression:operands must be numbers")),
            }

            TokenType::Greater => Ok(Object::Bool(left > right)),
            TokenType::Less => Ok(Object::Bool(left < right)),
            TokenType::GreaterEqual => Ok(Object::Bool(left >= right)),
            TokenType::LessEqual => Ok(Object::Bool(left <= right)),
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => Err(LoxError::new(expr.operator.line, "invalid operator")),
        }
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone().unwrap())
    }
    // for example: -1
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.ttype {
            TokenType::Minus => {
                match right {
                    Object::Number(n) => Ok(Object::Number(-n)),
                    _ => Err(LoxError::new(
                        expr.operator.line,
                        "Operand must be a number",
                    )),
                }
            }
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => Err(LoxError::new(expr.operator.line, "unreachable")),
        }
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }
    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object,Object::Nil | Object::Bool(false))
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use super::*;

    fn make_literal(o: Object) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(o),
        }))
    }

    #[test]
    fn test_unary_minus() {
        let expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal(Object::Number(4.0)),
        };
        let interpreter = Interpreter {};
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
        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
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

        let interpreter = Interpreter {};
        let result = interpreter.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Bool(true));
    }
}