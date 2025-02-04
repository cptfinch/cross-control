mod event;
mod event_queue;
mod network;
mod client;

use crate::event::{Event, EventType, TransportType};
use crate::event_queue::EventQueue;
use crate::network::{tcp, udp};
use tokio::time::{self, Duration};
use device_query::{DeviceQuery, DeviceState, Keycode};
use tokio::sync::mpsc;
use tracing::{info, error, warn};

fn setup_logging() {
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() {
    setup_logging();
    info!("Starting rust-barrier...");
    
    let buffer_size = 32;
    let mut event_queue = EventQueue::new(buffer_size);

    // Start both servers
    let server_event_queue = event_queue.sender.clone();
    tokio::spawn(async move {
        if let Err(e) = tcp::run_tcp_server(server_event_queue.clone()).await {
            eprintln!("TCP Server error: {}", e);
        }
    });

    let udp_event_queue = event_queue.sender.clone();
    tokio::spawn(async move {
        if let Err(e) = udp::run_udp_server(udp_event_queue).await {
            eprintln!("UDP Server error: {}", e);
        }
    });

    // Event capture and sending
    let server_addr_tcp = "127.0.0.1:8080".to_string();
    let server_addr_udp = "127.0.0.1:8081".to_string();
    let device_state = DeviceState::new();
    let mut sequence_number = 0;

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            let event = Event {
                event_type: EventType::KeyPress { key_code: key as u8 },
                transport: TransportType::Fast,
                sequence: sequence_number,
            };
            sequence_number += 1;

            if let Err(e) = udp::send_event_udp(event.clone(), &server_addr_udp).await {
                eprintln!("Failed to send UDP event: {}", e);
                // Fallback to TCP
                if let Err(e) = tcp::send_event_tcp(event.clone(), &server_addr_tcp).await {
                    eprintln!("Failed to send TCP event: {}", e);
                }
            }
        }

        let mouse = device_state.get_mouse();
        if mouse.coords.0 != 640 || mouse.coords.1 != 400 {
            let event = Event {
                event_type: EventType::MouseMove { x: mouse.coords.0, y: mouse.coords.1 },
                transport: TransportType::Fast,
                sequence: sequence_number,
            };
            sequence_number += 1;

            if let Err(e) = udp::send_event_udp(event.clone(), &server_addr_udp).await {
                eprintln!("Failed to send UDP event: {}", e);
                // Fallback to TCP
                if let Err(e) = tcp::send_event_tcp(event.clone(), &server_addr_tcp).await {
                    eprintln!("Failed to send TCP event: {}", e);
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
