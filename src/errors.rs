use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Not a valid identifier: {0}")]
    InvalidIdentifier(String)
}
