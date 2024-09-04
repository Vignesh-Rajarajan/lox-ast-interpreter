use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Nil,
    Bool(bool),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Nil => write!(f, "nil"),
            Object::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => n1.partial_cmp(n2),
            (Object::String(s1), Object::String(s2)) => s1.partial_cmp(s2),
            (Object::Bool(b1), Object::Bool(b2)) => b1.partial_cmp(b2),
            (Object::Nil, Object::Nil) => Some(std::cmp::Ordering::Equal),
            (Object::Nil, _) => Some(std::cmp::Ordering::Less),
            (_, Object::Nil) => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    }
}