use crate::error::LoxError;
use crate::object::Object;
use crate::token::Token;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(value) = self.values.get(name.lexeme.as_str()) {
            Ok(value.clone())
        } else {
            Err(LoxError::runtime_error(
                name,
                format!("Undefined variable '{}'.", name).as_str(),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else {
            Err(LoxError::runtime_error(
                name,
                format!("Undefined variable '{}'.", name).as_str(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::TokenType;

    #[test]
    fn test_define() {
        let mut env = Environment::new();
        env.define("a".to_string(), Object::Number(1.0));
        assert_eq!(env.values.get("a"), Some(&Object::Number(1.0)));
    }

    #[test]
    fn test_define_multiple() {
        let mut env = Environment::new();
        env.define("a".to_string(), Object::Number(1.0));
        env.define("a".to_string(), Object::Bool(true));
        let result = env.get(&Token::new(TokenType::Identifier, "a".to_string(), None, 0));
        assert_eq!(result.unwrap(), Object::Bool(true));
    }

    #[test]
    fn error_when_getting_undefined_variable() {
        let env = Environment::new();
        let result = env.get(&Token::new(TokenType::Identifier, "a".to_string(), None, 0));
        assert!(result.is_err());
    }
    #[test]
    fn error_when_assigning_undefined_variable() {
        let mut env = Environment::new();
        let tok = &Token::new(TokenType::Identifier, "b".to_string(), None, 0);
        assert!(env.assign(&tok, Object::Number(1.0)).is_err());
    }

    #[test]
    fn test_re_assign() {
        let mut env = Environment::new();
        env.define("a".to_string(), Object::Number(1.0));
        assert!(env
            .assign(
                &Token::new(TokenType::Identifier, "a".to_string(), None, 0),
                Object::Bool(true)
            )
            .is_ok());
        let result = env.get(&Token::new(TokenType::Identifier, "a".to_string(), None, 0));
        assert_eq!(result.unwrap(), Object::Bool(true));
    }
}
