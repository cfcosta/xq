use std::fs;
use std::path::PathBuf;

use anyhow::{ Result, bail };
use structopt::StructOpt;
use xq::{
    parser,
    storage::{DbStorage, Storage},
    types::*,
    errors::*
};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "FILE")]
    file: PathBuf,

    #[structopt(short = "d", long = "database-path")]
    db_path: String,
}

enum CommandResult {
    Empty,
    Val(Value),
}

fn run_command(storage: &mut dyn Storage, command: Command) -> Result<CommandResult> {
    println!("{:?}", &command);
    match command {
        Command::Enqueue(key, value) => {
            storage.enqueue(key, value)?;
            Ok(CommandResult::Empty)
        }
        Command::Dequeue(key) => {
            let value = storage.dequeue(key.clone())?;
            Ok(CommandResult::Val(value))
        }
        Command::Length(key) => {
            let value = storage.length(key.clone())?;
            Ok(CommandResult::Val(Value::Integer(value as i64)))
        }
        Command::Peek(key) => {
            let value = storage.peek(key.clone())?;
            Ok(CommandResult::Val(value.clone()))
        }
        Command::Assert(cmd, val) => {
            let cmd_desc = format!("{:?}", &cmd);

            match run_command(storage, *cmd)? {
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
    }
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.file)?;
    let mut storage = DbStorage::init(&options.db_path)?;

    let commands = parser::parse(&contents)?;

    for command in commands {
        let res = run_command(&mut storage, command);

        if res.is_err() {
            DbStorage::cleanup(&format!("{}", &options.db_path))?;
            res?;

            break;
        }
    }

    DbStorage::cleanup(&format!("{}", &options.db_path))?;

    println!("OK");

    Ok(())
}
