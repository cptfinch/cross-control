use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use crate::event::Event;
use serde_json;
use std::error::Error;
use std::net::SocketAddr;
use super::{Result, NetworkError};

pub type BoxError = Box<dyn Error + Send + Sync>;

pub async fn run_udp_server(event_queue: mpsc::Sender<Event>) -> Result<(), BoxError> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    println!("UDP Server running on 127.0.0.1:8081");
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let event: Event = serde_json::from_slice(&buf[..len])?;
        event_queue.send(event).await?;
    }
}

pub async fn send_event_udp(event: Event, server_addr: &str) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let event_data = serde_json::to_vec(&event)
        .map_err(NetworkError::Serialization)?;
    socket.send_to(&event_data, server_addr).await?;
    Ok(())
} 