use std::{convert::From, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Null,
}

impl Value {
    pub fn kind(&self) -> QueueType {
        match self {
            Self::Integer(_) => QueueType::Integer,
            Self::Float(_) => QueueType::Float,
            Self::String(_) => QueueType::String,
            Self::Null => QueueType::Null,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Null => write!(f, "null"),
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

impl From<&str> for Identifier {
    fn from(v: &str) -> Self {
        Identifier(v.into())
    }
}

impl From<String> for Identifier {
    fn from(v: String) -> Self {
        Identifier(v.into())
    }
}

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
    AssertError(Box<Command>),
    Noop,
}

impl Command {
    pub fn enqueue<Id: Into<Identifier>, V: Into<Value>>(id: Id, v: V) -> Self {
        Self::Enqueue(id.into(), v.into())
    }

    pub fn dequeue<T: Into<Identifier>>(id: T) -> Self {
        Self::Dequeue(id.into())
    }

    pub fn peek<T: Into<Identifier>>(id: T) -> Self {
        Self::Peek(id.into())
    }

    pub fn length<T: Into<Identifier>>(id: T) -> Self {
        Self::Length(id.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueType {
    Integer,
    Float,
    String,
    Null
}
