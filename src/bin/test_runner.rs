use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use xq::{
    parser,
    storage::{MemStore, Storage},
    types::Command,
};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "FILE")]
    file: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.file)?;
    let mut storage = MemStore::new();

    let commands = parser::parse(&contents)?;

    for command in commands {
        match command {
            Command::Enqueue(key, value) => {
                println!("[ENQUEUE] {}: {}", &key, &value);
                storage.enqueue(key, value)?;
            }
            Command::Dequeue(key) => {
                let value = storage.dequeue(key.clone())?;
                println!("[DEQUEUE] {}: {}", key, value);
            }
            Command::Length(key) => {
                let value = storage.length(key.clone())?;
                println!("[LENGTH] {}: {}", key, value);
            }
            Command::Peek(key) => {
                let value = storage.peek(key.clone())?;
                println!("[PEEK] {}: {}", key, value);
            }
        }
    }

    Ok(())
}
