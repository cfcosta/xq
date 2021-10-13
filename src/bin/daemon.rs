use std::{fmt::Debug, str};

use anyhow::Result;
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, trace};

use xq::{
    parser, run_command,
    storage::{Storage, StorageBackend, StorageOptions},
    CommandResult,
};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "ADDRESS", default_value = "0.0.0.0:8080")]
    addr: String,
    #[structopt(flatten)]
    storage: StorageOptions,
}

#[tracing::instrument]
async fn run_server<T: StorageBackend + Send + Sync + Debug>(
    mut socket: TcpStream,
    storage: T,
) -> Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        let _ = socket.read(&mut buf).await?;
        match parser::parse(&str::from_utf8(&buf)?) {
            Ok(commands) => {
                for command in commands {
                    debug!(command = ?&command, "Running command");

                    match run_command(&storage, command).await {
                        Ok(res) => match res {
                            CommandResult::Empty => socket.write_all(b"OK\n").await?,
                            CommandResult::Val(v) => {
                                socket.write_all(format!("{}\n", v).as_bytes()).await?
                            }
                        },
                        Err(e) => {
                            socket
                                .write_all(format!("ERROR: {}\n", e).as_bytes())
                                .await?
                        }
                    }
                }
            }
            Err(e) => {
                socket
                    .write_all(format!("ERROR: {}\n", e).as_bytes())
                    .await?
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    trace!("Started subscriber for tracing");

    let options = Options::from_args();

    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&options.storage.database_path)?;

    let listener = TcpListener::bind(&options.addr).await?;

    info!(address = %&options.addr, "Daemon started");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(run_server(socket, storage.clone()));
    }
}
