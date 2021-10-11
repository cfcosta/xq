use std::{ env, str };

use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{ TcpListener, TcpStream };

use xq::{
    parser,
    storage::{ Storage, StorageBackend },
    run_command, CommandResult
};

async fn run_server<T: StorageBackend + Clone>(mut socket: TcpStream, mut storage: T) -> Result<()> {
        let mut buf = vec![0;1024];

        loop {
            let _ = socket.read(&mut buf).await?;
            let commands = parser::parse(&str::from_utf8(&buf)?)?;

            for command in commands {
                let res = run_command(&mut storage, command)?;

                match res {
                    CommandResult::Empty => socket.write_all(b"OK\n").await?,
                    CommandResult::Val(v) => socket.write_all(format!("{}\n", v).as_bytes()).await?
                }
            }
        }
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&options.storage.database_path)?;

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(run_server(socket, storage.clone()));
    }
}
