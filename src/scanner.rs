use crate::error::LoxError;
use crate::token::{Object, Token};
use crate::token_type::TokenType;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_err: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(err) => {
                    err.report("".to_string());
                    had_err = Some(err);
                }
            }
        }
        if let Some(err) = had_err {
            return Err(err);
        }
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_match('*') {
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            _ => {
                if self.is_alpha_numeric(c) {
                    self.identifier()?;
                } else {
                    return Err(LoxError::new(
                        self.line,
                        "Unexpected character.".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::new(self.line, "Unterminated string.".to_string()));
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1]; // +1 and -1 to remove the quotes
        self.add_token_with_literal(TokenType::String, Some(Object::String(value.to_string())));
        Ok(())
    }

    fn advance(&mut self) -> char {
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    //In essence, this code is creating a new token based on the current state of the scanner (the type of token, the text it represents, any literal value, and its location in the source code) and adding it to the list of tokens
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Object>) {
        // This line is creating a substring (called a lexeme) from the source code.
        // It starts at the index self.start and ends at self.current.
        let lexeme = &self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal,
            self.line,
        ));
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token_with_literal(
            TokenType::Number,
            Some(Object::Number(
                self.source[self.start..self.current].parse().unwrap(),
            )),
        );
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = Scanner::keywords(text);
        if let Some(token_type) = token_type {
            self.add_token(token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
        Ok(())
    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || self.is_digit(c)
    }

    fn keywords(identifier: &str) -> Option<TokenType> {
        match identifier {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    fn scan_comment(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                '*' => {
                    self.advance();
                    if self.is_match('/') {
                        return Ok(());
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    self.advance();
                    if self.is_match('*') {
                        self.scan_comment()?; // we are calling the function recursively for the nested comment /* /*   */  */
                    }
                }
                '\0' => {
                    return Err(LoxError::new(
                        self.line,
                        "Unterminated comment.".to_string(),
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
