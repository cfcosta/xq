use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    #[error("Failed to parse input: {0}")]
    ParseError(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    #[error("Failed assertion: {command}\n  expected: {expected}\n  got: {got:?}")]
    FailedAssertion {
        command: String,
        expected: String,
        got: Option<String>,
    },
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    #[error("Failed to initialize storage")]
    FailedInitialize,
    #[error("Failed to get lock on the storage")]
    FailedLock,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    #[error("Connection error with the server")]
    ConnectionError,
}
