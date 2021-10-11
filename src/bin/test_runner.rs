use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use xq::{
    parser,
    storage::{Storage, StorageOptions},
    run_command
};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "FILE")]
    file: PathBuf,
    #[structopt(flatten)]
    storage: StorageOptions,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.file)?;

    #[cfg(feature = "memory-storage")]
    let mut storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let mut storage = Storage::init(&options.storage.database_path)?;

    let commands = parser::parse(&contents)?;

    for command in commands {
        let _ = run_command(&mut storage, command)?;
    }

    println!("OK");

    Ok(())
}
