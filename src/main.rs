use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
mod command;
use command::Command;
mod store;
mod persistence;
use store::KeyValueStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server listening on port 6379");

    let store = Arc::new(KeyValueStore::new());
    store.load("data.json").await?;

    let store_clone = store.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        store_clone.save("data.json").await.unwrap();
        std::process::exit(0);
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {:?}", addr);
        let store = store.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, store).await {
                eprintln!("Error handling client {}; {:?}", addr, e);
            }
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream, store: Arc<KeyValueStore>) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            // Connection closed
            break;
        }

        println!("Received: {}", line.trim());
        let command = Command::parse(&line);

        match command {
            Command::Get(key) => {
                // Handle GET
                if let Some(value) = store.get(&key).await {
                    writer.write_all(format!("VALUE {}\n", value).as_bytes()).await?;
                } else {
                    writer.write_all(b"ERROR Key not found\n").await?;
                }
                
            }
            Command::Set(key, value) => {
                // Handle SET
                store.set(key, value).await;
                writer.write_all(b"OK\n").await?;
            }
            Command::Delete(key) => match store.delete(&key).await {
                Ok(true) => writer.write_all(b"OK\n").await?,
                Ok(false) => writer.write_all(b"ERROR Key not found\n").await?,
                Err(e) => {
                    writer.write_all(format!("ERROR {}\n", e).as_bytes()).await?
                }
            
            }
            Command::Unknown => {
                writer.write_all(b"ERROR Unknown Command\n").await?;
            }
        }
    }

    Ok(())
}
