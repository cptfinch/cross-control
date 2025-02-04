pub mod tcp;
pub mod udp;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Channel send error")]
    ChannelSend,
}

pub type Result<T> = std::result::Result<T, NetworkError>; 