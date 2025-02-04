use tokio::sync::mpsc;
use std::time::Duration;

pub async fn setup_test_server() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        udp::run_udp_server(tx).await
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    rx
}

pub fn create_test_event() -> Event {
    Event {
        event_type: EventType::MouseMove { x: 100, y: 200 },
        transport: TransportType::Fast,
        sequence: 1,
    }
} 