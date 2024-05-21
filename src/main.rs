mod event;
mod event_queue;
mod network;
mod client;

use crate::event::{Event, EventType};
use crate::event_queue::EventQueue;
use crate::network::run_server;
use crate::client::send_event;
use tokio::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

#[tokio::main]
async fn main() {
    let buffer_size = 32;
    let mut event_queue = EventQueue::new(buffer_size);

    // Spawn the server task
    let server_event_queue = event_queue.sender.clone();
    tokio::spawn(async move {
        if let Err(e) = run_server(server_event_queue).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Add a Quit event after 5 seconds for demonstration
    let timer_event_queue = event_queue.sender.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        timer_event_queue.send(Event { event_type: EventType::Quit }).await.unwrap();
    });

    // Capture events and send them to the server
    let event_queue_sender = event_queue.sender.clone();
    let server_addr = "127.0.0.1:8080".to_string();
    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            let event = Event {
                event_type: EventType::KeyPress { key_code: key as u8 },
            };
            if let Err(e) = send_event(event.clone(), &server_addr).await {
                eprintln!("Failed to send event: {}", e);
            }
            event_queue_sender.send(event).await.unwrap();
        }

        let mouse = device_state.get_mouse();
        let event = Event {
            event_type: EventType::MouseMove { x: mouse.coords.0, y: mouse.coords.1 },
        };
        if let Err(e) = send_event(event.clone(), &server_addr).await {
            eprintln!("Failed to send event: {}", e);
        }
        event_queue_sender.send(event).await.unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Process events
    event_queue.process_events().await;
}

