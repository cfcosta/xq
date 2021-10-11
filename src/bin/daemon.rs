use std::str;

use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{ TcpListener, TcpStream };
use structopt::StructOpt;

use xq::{
    parser,
    storage::{ Storage, StorageBackend, StorageOptions },
    run_command, CommandResult
};

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(name = "ADDRESS", default_value = "127.0.0.1:8080")]
    addr: String,
    #[structopt(flatten)]
    storage: StorageOptions,
}

async fn run_server<T: StorageBackend + Send + Sync>(mut socket: TcpStream, storage: T) -> Result<()> {
        let mut buf = vec![0;1024];

        loop {
            let _ = socket.read(&mut buf).await?;
            let commands = parser::parse(&str::from_utf8(&buf)?)?;

            for command in commands {
                let res = run_command(&storage, command).await?;

                match res {
                    CommandResult::Empty => socket.write_all(b"OK\n").await?,
                    CommandResult::Val(v) => socket.write_all(format!("{}\n", v).as_bytes()).await?
                }
            }
        }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&options.storage.database_path)?;

    let listener = TcpListener::bind(&options.addr).await?;

    println!("Listening on {}", &options.addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(run_server(socket, storage.clone()));
    }
}
