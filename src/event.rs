// src/event.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    MouseMove { x: i32, y: i32 },
    KeyPress { key_code: u8 },
    Quit,
    SyncRequest, // For state synchronization
    SyncResponse { mouse_x: i32, mouse_y: i32, keys: Vec<u8> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Reliable, // Use TCP
    Fast,     // Use UDP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub transport: TransportType,
    pub sequence: u32, // For ordering and loss detection
}
