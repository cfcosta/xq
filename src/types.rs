use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
            Value::String(v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Hash)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Command {
    Enqueue(Identifier, Value),
    Dequeue(Identifier),
    Length(Identifier),
    Peek(Identifier),
    Assert(Box<Command>, Value)
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Enqueue(id, value) => write!(f, "enqueue {} {}", &id.0, &value),
            Command::Dequeue(id) => write!(f, "dequeue {}", &id.0),
            Command::Length(id) => write!(f, "length {}", &id.0),
            Command::Peek(id) => write!(f, "peek {}", &id.0),
            Command::Assert(cmd, value) => write!(f, "assert ({}) {}", cmd, &value),
        }
    }
}
