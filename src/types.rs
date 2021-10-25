use std::{ fmt, convert::From };

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}

impl From<i64> for Value {
    fn from(item: i64) -> Self {
        Value::Integer(item)
    }
}

impl From<String> for Value {
    fn from(item: String) -> Self {
        Value::String(item)
    }
}

impl From<f64> for Value {
    fn from(item: f64) -> Self {
        Value::Float(item)
    }
}

impl From<f32> for Value {
    fn from(item: f32) -> Self {
        Value::Float(item as f64)
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Hash)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Enqueue(Identifier, Value),
    Dequeue(Identifier),
    Length(Identifier),
    Peek(Identifier),
    Assert(Box<Command>, Value),
    Noop
}
