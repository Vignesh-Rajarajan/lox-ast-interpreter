use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;


#[derive(Debug)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    GenericError { line: usize, message: String },
    SystemError { message: String },
    Break,
    ReturnValue { value: Object },
}
impl LoxResult {
    pub fn new(line: usize, message: &str) -> Self {
        let err = LoxResult::GenericError {
            line,
            message: message.to_string(),
        };
        err.report("");
        err
    }
    pub fn pares_error(token: Token, message: &str) -> Self {
        let err = LoxResult::ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report("");
        err
    }
    pub fn runtime_error(token: &Token, message: &str) -> Self {
        let err = LoxResult::RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn return_value(value: Object) -> Self {
        let err = LoxResult::ReturnValue { value };
        err.report("");
        err
    }

    pub fn system_error(message: &str) -> Self {
        let err = LoxResult::SystemError {
            message: message.to_string(),
        };
        err.report("");
        err
    }
    pub fn report(&self, loc: &str) {
        match self {
            LoxResult::ParseError { token, message } | LoxResult::RuntimeError { token, message } => {
                if token.ttype == TokenType::Eof {
                    eprintln!("[line {}] Error at end: {}", token.line, message);
                } else {
                    eprintln!(
                        "[line {}] Error at '{}': {}",
                        token.line, token.lexeme, message
                    );
                }
            }
            LoxResult::GenericError { line, message } => {
                eprintln!("[line {}] Error {}: {}", line, loc, message);
            }
            LoxResult::Break | LoxResult::ReturnValue { .. } => {}
            LoxResult::SystemError { message } => {
                eprintln!("Error: {}", message);
            }
        }
    }
}
