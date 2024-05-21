use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::event::Event;
use serde_json;
use std::error::Error;

pub async fn send_event(event: Event, server_addr: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stream = TcpStream::connect(server_addr).await?;
    let event_data = serde_json::to_vec(&event)?;
    stream.write_all(&event_data).await?;
    Ok(())
}
