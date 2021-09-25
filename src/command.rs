#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub enum Command {
    Enqueue(Identifier, Value),
    Dequeue(Identifier)
}
