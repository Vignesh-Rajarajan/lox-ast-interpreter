use crate::error::LoxResult;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use crate::token_type::TokenType;

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }


    pub fn get_at(&self, distance: usize, name: &str) -> Result<Object, LoxResult> {
        if distance == 0 {
            Ok(self.values.get(name).unwrap().clone())
        }else{
           self.enclosing.as_ref().unwrap().borrow().get_at(distance - 1, name)
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> Result<(), LoxResult> {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }else{
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value)
        }
    }

pub fn get(&self, name: &Token) -> Result<Object, LoxResult> {
        if let Some(value) = self.values.get(name.lexeme.as_str()) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(LoxResult::runtime_error(
                name,
                format!("Undefined variable '{}'.", name).as_str(),
            ))
        }
    }
    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxResult> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::runtime_error(
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

    #[test]
    fn can_enclose_an_environment() {
        let env = Environment::new();
        let env2 = Environment::new_with_enclosing(Rc::new(RefCell::new(env)));
        assert!(env2.enclosing.is_some());
    }

    #[test]
    fn can_read_from_enclosing_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        let env2 = Environment::new_with_enclosing(Rc::clone(&env));
        assert!(env2.enclosing.is_some());
        let result = env2.get(&Token::new(TokenType::Identifier, "a".to_string(), None, 0));
        assert_eq!(result.unwrap(), Object::Number(1.0));
    }
    #[test]
    fn can_assign_to_enclosing_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        let mut env2 = Environment::new_with_enclosing(Rc::clone(&env));
        let token = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        let assign_result = env2.assign(&token, Object::Number(92.0));
        assert!(assign_result.is_ok());
        let result = env2.get(&token);
        assert_eq!(result.unwrap(), Object::Number(92.0));
    }
}
