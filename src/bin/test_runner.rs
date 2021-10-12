use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use tracing::{ info, debug };
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

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    debug!("Started subscriber for tracing");

    let options = Options::from_args();
    let contents = fs::read_to_string(options.file)?;

    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&options.storage.database_path)?;

    debug!("Initialized storage");

    debug!("Running program: {}", &contents);
    let commands = parser::parse(&contents)?;

    for command in commands {
        debug!("Running command: {:?}", &command);
        let _ = run_command(&storage, command).await?;
    }

    info!("Test finished successfully");

    Ok(())
}
