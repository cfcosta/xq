#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String)
}

#[derive(Debug)]
pub enum Command {
    Enqueue(String, Value),
    Dequeue(String)
}

#[derive(Debug)]
pub enum Operation {
    Enqueue,
    Dequeue
}
