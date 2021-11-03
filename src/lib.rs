use std::fmt::Debug;

use anyhow::{bail, Result};

pub mod errors;
pub mod parser;
pub mod storage;
pub mod types;

use errors::*;
use storage::StorageBackend;
use types::*;

#[derive(Debug, Clone, Copy)]
pub struct Runtime;

impl Runtime {
    #[tracing::instrument]
    pub fn run_command<T: StorageBackend + Send + Sync + Debug>(
        &self,
        storage: &T,
        command: Command,
        ) -> Result<Option<Value>> {
        match command {
            Command::Open(key, kind) => {
                storage.open(&key, kind)?;
                Ok(None)
            }
            Command::Close(key) => {
                storage.close(&key)?;
                Ok(None)
            }
            Command::Enqueue(key, value) => {
                storage.enqueue(&key, value)?;
                Ok(None)
            }
            Command::Dequeue(key) => {
                let value = storage.dequeue(&key)?;
                Ok(Some(value))
            }
            Command::Length(key) => {
                let value = storage.length(&key)?;
                Ok(Some((value as i64).into()))
            }
            Command::Peek(key) => {
                let value = storage.peek(&key)?;
                Ok(Some(value))
            }
            Command::Assert(cmd, val) => {
                let cmd_desc = format!("{:?}", &cmd);

                match self.run_command(storage, *cmd)? {
                    Some(result) => {
                        if result == val {
                            return Ok(None);
                        }

                        bail!(DataError::FailedAssertion {
                            command: cmd_desc,
                            expected: format!("{:?}", val),
                            got: format!("{:?}", result),
                        })
                    }
                    None => {
                        bail!(DataError::FailedAssertion {
                            command: cmd_desc,
                            expected: format!("{:?}", val),
                            got: format!("{:?}", Value::Null)
                        })
                    }
                }
            }
            Command::AssertError(cmd) => {
                let cmd_desc = format!("{:?}", &cmd);

                match self.run_command(storage, *cmd) {
                    Ok(Some(result)) => bail!(DataError::FailedAssertion {
                        command: cmd_desc,
                        expected: String::from("Error"),
                        got: format!("{:?}", result),
                    }),
                    Ok(None) => bail!(DataError::FailedAssertion {
                        command: cmd_desc,
                        expected: String::from("Error"),
                        got: format!("{:?}", Value::Null)
                    }),
                    Err(_) => Ok(None),
                }
            }
            Command::Noop => Ok(None),
        }
    }
}
