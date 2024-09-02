use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        let err= LoxError { token: None, line, message };
        err.report("".to_string());
        err
    }
    pub fn pares_error(token: Token, message: String) -> Self {
        let line = token.line;
        let err = LoxError { token: Some(token), line, message };
        err.report("".to_string());
        err
    }
    pub fn report(&self, loc: String) {
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
