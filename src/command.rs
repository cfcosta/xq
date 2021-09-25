#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String)
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub enum Command {
    Enqueue(String, Value),
    Dequeue(String)
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Enqueue,
    Dequeue
}
