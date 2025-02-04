use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
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

pub struct NetworkConnection {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl NetworkConnection {
    pub fn new(stream: TcpStream) -> Self {
        let (reader_half, writer_half) = stream.into_split();
        let reader = BufReader::new(reader_half);
        Self { reader, writer: writer_half }
    }

    pub async fn send_event(&mut self, event: Event) -> Result<()> {
        let mut data = serde_json::to_vec(&event)?;
        data.push(b'\n');
        self.writer.write_all(&data).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn receive_event(&mut self) -> Result<Event> {
        let mut line = String::new();
        let n = self.reader.read_line(&mut line).await?;
        if n == 0 {
            return Err(NetworkError::Connection("Connection closed".into()));
        }
        Ok(serde_json::from_str(line.trim())?)
    }
}