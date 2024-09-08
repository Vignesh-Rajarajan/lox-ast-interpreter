use crate::error::LoxError;
use crate::expr::Expr::Unary;
use crate::expr::{
    AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr,
};
use crate::object::Object;
use crate::stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt};
use crate::token::Token;
use crate::token_type::TokenType;
use Expr::Binary;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    had_error: bool,
}

// Example: !(2 + 3) * 4 == 5 - 6 / 3
//         ==
//        /  \
//       /    \
//      *      -
//     / \    / \
//    !   4  5   /
//    |         / \
//    ()       6   3
//    |
//    +
//   / \
//  2   3

// Here's how the parser would process this expression, step by step:
// 1. expression() is called, which immediately calls equality().
// 2. equality() calls comparison() for the left side.
// 3. comparison() calls term() as there are no comparison operators.
// 4. term() calls factor() for the left side.
// 5. factor() calls unary().
// 6. unary() sees the ! operator and creates a unary expression. It then recursively calls unary() for the right side of the !.
// 7. The next unary() call doesn't see a unary operator, so it calls primary().
// 8. primary() sees the left parenthesis and creates a grouping expression. It then calls expression() for the contents of the parentheses.
// 9. This new expression() call goes through equality(), comparison(), and term(), finally reaching factor().
// 10. factor() parses 2 + 3 as a binary expression.
// 11. The grouping is completed, and we return to the original unary() call, which now has !(2 + 3) as its right side.
// 12. We return to factor(), which sees the * operator and creates a binary expression with !(2 + 3) as its left side.
// 13. factor() calls unary() for the right side of *, which simply returns the literal 4.
// 14. We now have !(2 + 3) * 4 parsed, and we return up to term().
// 15. term() doesn't see + or -, so it returns to comparison().
// 16. comparison() doesn't see any comparison operators, so it returns to equality().
// 17. equality() sees the == operator and creates a binary expression with !(2 + 3) * 4 as its left side.
// 18. For the right side of ==, equality() calls comparison() again.
// 19. This goes down to term(), which sees the - operator and creates a binary expression.
// 20. The left side of - is the literal 5, and for the right side, term() calls factor().
// 21. factor() sees the / operator and creates another binary expression with 6 and 3.

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0, // it's index into the vec tokens
            had_error: false,
        }
    }

    // This is the entry point for the parser.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    //It simply calls equality(), which is the highest precedence level in the expression grammar.
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronise();
        }
        result
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: var.name,
                    value: Box::new(value),
                }));
            }
            return Err(self.error(equals, "Invalid assignment target."));
        }
        Ok(expr)
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let initializer = if self.is_match(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: expr }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    // This method handles comparison operators (>, >=, <, <=). It works similarly to equality() but for comparison operators.
    //Example: a > b <= c would be parsed as ((a > b) <= c).
    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    // These methods handle addition/subtraction and multiplication/division respectively. They work similarly to comparison() but for their specific operators.
    //Example for term(): a + b - c would be parsed as ((a + b) - c).
    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }
    // Example for factor(): a * b / c would be parsed as ((a * b) / c).
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }
    // This method handles unary operators (! and -). If it finds a unary operator, it creates a unary expression. Otherwise, it falls through to primary().
    //Example: !-a would be parsed as (!(-a)).
    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }
    //This method handles the most basic expressions: literals (numbers, strings, booleans, nil) and parenthesized expressions.
    //Example: For a number like 123, it would create a Literal expression.
    //For a parenthesized expression like (1 + 2), it would create a Grouping expression containing the inner expression.
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }));
        }

        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }));
        }

        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(self.previous().literal.clone().unwrap()),
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().clone(),
            }));
        }
        Err(LoxError::new(self.peek().line, "Expect expression."))
    }
    // consume checks if the current token matches the given token type
    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek().clone(), message))
    }
    // This method handles equality comparisons (== and !=).
    //It first parses a comparison expression, then checks for equality operators. If found, it creates a binary expression.
    //Example: a == b != c would be parsed as ((a == b) != c).
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == ttype
    }

    /// Advances the parser to the next token and returns the previous token.
    /// This method is used to consume the current token and move the parser forward.
    /// If the parser is already at the end of the token stream, this method will not advance further.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&mut self, token: Token, message: &str) -> LoxError {
        self.had_error = true;
        LoxError::pares_error(token, message)
    }
    pub fn success(&self) -> bool {
        !self.had_error
    }

    /// Synchronizes the parser by advancing to the next token and skipping tokens until a valid statement is found.
    /// This is used to recover from parse errors by skipping tokens until a known statement is encountered,
    /// allowing the parser to continue processing the rest of the program.
    fn synchronise(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            if matches!(
                self.peek().ttype,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }
}
