use std::rc::Rc;
use crate::callable::LoxCallable;
use crate::environment::Environment;
use crate::error::LoxResult;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt::{FunctionStmt, Stmt};
use crate::token::Token;

pub struct LoxFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt) -> Self {
        LoxFunction {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, LoxResult> {
        let mut env = Environment::new_with_enclosing(Rc::clone(&interpreter.globals));
        for (param, arg) in self.params.iter().zip(args) {
            env.define(param.lexeme.clone(), arg);
        }
        match interpreter.execute_block(&self.body, env) {
            Err(LoxResult::ReturnValue { value: val }) => Ok(val),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil),
        }
    }
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }
}