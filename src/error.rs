use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, message: &str) -> Self {
        let err= LoxError { token: None, line, message:message.to_string() };
        err.report("");
        err
    }
    pub fn pares_error(token: Token, message: &str) -> Self {
        let line = token.line;
        let err = LoxError { token: Some(token), line, message:message.to_string() };
        err.report("");
        err
    }
    pub fn runtime_error(token: &Token, message: &str) -> Self {
        let line = token.line;
        let err = LoxError { token: Some(token.clone()), line, message:message.to_string() };
        err.report("");
        err
    }
    pub fn report(&self, loc: &str) {
        if let Some(token) = &self.token {
            if token.ttype == TokenType::EOF {
                eprintln!("[line {}] Error at end: {}", self.line, self.message);
            }else{
                eprintln!("[line {}] Error at '{}': {}", self.line, token.lexeme, self.message);
            }
        }else{
            eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
        }
    }
}
