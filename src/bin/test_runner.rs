use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use tracing::{debug, info, trace};

use xq::{
    parser, run_command,
    storage::{Storage, StorageOptions},
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

    trace!("Initialized storage");

    trace!(program = ?&contents, "Running program");
    let commands = parser::parse(&contents)?;

    for command in commands {
        debug!(command = ?&command, "Running command");
        let _ = run_command(&storage, command)?;
    }

    info!("Test finished successfully");

    Ok(())
}
