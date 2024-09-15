use crate::error::LoxResult;
use crate::interpreter::Interpreter;
use crate::object::Object;
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}
impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<callable>")
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}
impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}
pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, args)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn to_string(&self) -> String {
        self.func.to_string()
    }
}

pub struct NativeClock;
impl LoxCallable for NativeClock {
    fn call(&self, _interpreter: &Interpreter, _args: Vec<Object>) -> Result<Object, LoxResult> {
        Ok(Object::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as f64,
        ))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "Native:clock".to_string()
    }
}
