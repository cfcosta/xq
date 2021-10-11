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

    #[error("Failed assertion: {command}\n  expected: {expected}\n  got: {got:?}")]
    FailedAssertion {
        command: String,
        expected: String,
        got: Option<String>
    },
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    #[error("Failed to initialize storage")]
    FailedInitialize
}
