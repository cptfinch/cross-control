use rust_barrier::network::{udp, tcp, NetworkError};
use rust_barrier::event::{Event, EventType, TransportType};
use tokio::sync::mpsc;
use std::time::Duration;
use serde_json;

#[tokio::test]
async fn test_udp_server_startup() {
    let (tx, _rx) = mpsc::channel(32);
    let server = tokio::spawn(async move {
        udp::run_udp_server(tx).await
    });

    // Give the server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Try to bind to the same port - should fail
    let result = tokio::net::UdpSocket::bind("127.0.0.1:8081").await;
    assert!(result.is_err());

    server.abort(); // Clean up
}

#[tokio::test]
async fn test_udp_event_roundtrip() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // Start server
    let server = tokio::spawn(async move {
        udp::run_udp_server(tx).await
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Create and send test event
    let test_event = Event {
        event_type: EventType::MouseMove { x: 100, y: 200 },
        transport: TransportType::Fast,
        sequence: 1,
    };

    let send_result = udp::send_event_udp(
        test_event.clone(),
        "127.0.0.1:8081"
    ).await;
    assert!(send_result.is_ok());

    // Verify received event
    match tokio::time::timeout(Duration::from_secs(1), rx.recv()).await {
        Ok(Some(received)) => {
            assert_eq!(received.sequence, test_event.sequence);
            assert_matches!(received.event_type, 
                EventType::MouseMove { x: 100, y: 200 }
            );
        },
        Ok(None) => panic!("Channel closed unexpectedly"),
        Err(_) => panic!("Timeout waiting for event"),
    }

    server.abort();
}

#[tokio::test]
async fn test_error_handling() {
    // Test invalid address
    let test_event = Event {
        event_type: EventType::MouseMove { x: 0, y: 0 },
        transport: TransportType::Fast,
        sequence: 1,
    };

    let result = udp::send_event_udp(
        test_event,
        "invalid-address"
    ).await;
    
    assert!(matches!(result, Err(NetworkError::Io(_))));
}

#[tokio::test]
async fn test_basic_event_serialization() {
    let event = Event {
        event_type: EventType::Ping,  // We'll add this new event type
        transport: TransportType::Fast,
        sequence: 1,
    };

    let serialized = serde_json::to_string(&event).unwrap();
    let deserialized: Event = serde_json::from_str(&serialized).unwrap();

    assert_eq!(event.sequence, deserialized.sequence);
    assert!(matches!(deserialized.event_type, EventType::Ping));
} 