use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Not a valid identifier: {0}")]
    InvalidIdentifier(String),
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Tried to dequeue from an empty queue: {0}")]
    EmptyQueue(String),
}
