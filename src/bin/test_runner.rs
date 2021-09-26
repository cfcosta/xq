use std::fs;
use std::path::PathBuf;

use anyhow::{ Result, anyhow };
use structopt::StructOpt;
use xq::{parser, command::Command, storage::{MemStore,Storage}};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "FILE")]
    file: PathBuf
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.file)?;
    let mut storage = MemStore::new();

    let (_, commands) = parser::parse(&contents).map_err(|_| anyhow!("Failed to parse program."))?;

    for command in commands {
        match command {
            Command::Enqueue(key, value) => {
                println!("[ENQUEUE] {}: {}", &key, &value);
                storage.enqueue(key, value)?;
            },
            Command::Dequeue(key) => {
                let value = storage.dequeue(key.clone())?;
                println!("[DEQUEUE] {}: {}", key, value);
            }
        }
    }

    Ok(())
}
