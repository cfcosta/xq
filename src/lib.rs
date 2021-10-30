use std::fmt::Debug;

use anyhow::{bail, Result};
use async_recursion::async_recursion;

pub mod errors;
pub mod parser;
pub mod storage;
pub mod types;

use errors::*;
use storage::StorageBackend;
use types::*;

pub enum CommandResult {
    Empty,
    Val(Value),
}

#[tracing::instrument]
#[async_recursion]
pub async fn run_command<T: StorageBackend + Send + Sync + Debug>(
    storage: &T,
    command: Command,
) -> Result<CommandResult> {
    match command {
        Command::Enqueue(key, value) => {
            storage.enqueue(&key, value).await?;
            Ok(CommandResult::Empty)
        }
        Command::Dequeue(key) => {
            let value = storage.dequeue(&key).await?;
            Ok(CommandResult::Val(value))
        }
        Command::Length(key) => {
            let value = storage.length(&key).await?;
            Ok(CommandResult::Val(Value::Integer(value as i64)))
        }
        Command::Peek(key) => {
            let value = storage.peek(&key).await?;
            Ok(CommandResult::Val(value))
        }
        Command::Assert(cmd, val) => {
            let cmd_desc = format!("{:?}", &cmd);

            match run_command(storage, *cmd).await? {
                CommandResult::Val(result) => {
                    if result == val {
                        return Ok(CommandResult::Empty);
                    }

                    bail!(DataError::FailedAssertion {
                        command: cmd_desc,
                        expected: format!("{:?}", val),
                        got: Some(format!("{:?}", result)),
                    })
                }
                CommandResult::Empty => {
                    bail!(DataError::FailedAssertion {
                        command: cmd_desc,
                        expected: format!("{:?}", val),
                        got: None
                    })
                }
            }
        }
        Command::AssertError(cmd) => {
            let cmd_desc = format!("{:?}", &cmd);

            match run_command(storage, *cmd).await {
                Ok(CommandResult::Val(result)) => bail!(DataError::FailedAssertion {
                    command: cmd_desc,
                    expected: String::from("Error"),
                    got: Some(format!("{:?}", result)),
                }),
                Ok(CommandResult::Empty) => bail!(DataError::FailedAssertion {
                    command: cmd_desc,
                    expected: String::from("Error"),
                    got: None,
                }),
                Err(_) => Ok(CommandResult::Empty)
            }
        }
        Command::Noop => Ok(CommandResult::Empty),
    }
}
