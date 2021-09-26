use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxError {
    #[error("Parse Error")]
    ParseError,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    #[error("Tried to dequeue from an empty queue: {0}")]
    EmptyQueue(String),
}
