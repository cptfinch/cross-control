use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use crate::event::{Event};
use serde_json;
use std::error::Error;

pub type BoxError = Box<dyn Error + Send + Sync>;

pub async fn handle_client(mut stream: TcpStream, event_queue: mpsc::Sender<Event>) -> Result<(), BoxError> {
    let mut buffer = [0; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        let event: Event = serde_json::from_slice(&buffer[..n])?;
        event_queue.send(event).await?;
    }

    Ok(())
}

pub async fn run_server(event_queue: mpsc::Sender<Event>) -> Result<(), BoxError> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        let event_queue = event_queue.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, event_queue).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}

