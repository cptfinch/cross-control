use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::event::Event;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Connection error: {0}")]
    Connection(String),
}

pub type Result<T> = std::result::Result<T, NetworkError>;

pub async fn send_event(stream: &mut TcpStream, event: Event) -> Result<()> {
    let data = serde_json::to_vec(&event)?;
    stream.write_all(&data).await?;
    Ok(())
}

pub async fn receive_event(stream: &mut TcpStream) -> Result<Event> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Err(NetworkError::Connection("Connection closed".into()));
    }
    Ok(serde_json::from_slice(&buf[..n])?)
}