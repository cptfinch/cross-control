use device_query::{DeviceQuery, DeviceState, Keycode};
use tokio::time::Duration;

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

    // Capture events
    let event_queue_sender = event_queue.sender.clone();
    tokio::spawn(async move {
        let device_state = DeviceState::new();
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            for key in keys {
                let event = Event {
                    event_type: EventType::KeyPress { key_code: key as u8 },
                };
                event_queue_sender.send(event).await.unwrap();
            }

            let mouse = device_state.get_mouse();
            let event = Event {
                event_type: EventType::MouseMove { x: mouse.coords.0, y: mouse.coords.1 },
            };
            event_queue_sender.send(event).await.unwrap();

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // Add a Quit event after 5 seconds for demonstration
    let timer_event_queue = event_queue.sender.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        timer_event_queue.send(Event { event_type: EventType::Quit }).await.unwrap();
    });

    // Process events
    event_queue.process_events().await;
}

