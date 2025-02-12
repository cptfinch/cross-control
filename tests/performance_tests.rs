use rust_barrier::event::Event;
use rust_barrier::network::NetworkConnection;
use tokio::net::{TcpListener, TcpStream};
use std::time::Instant;

#[tokio::test]
async fn test_event_throughput() {
    let listener = TcpListener::bind("127.0.0.1:8088").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8088").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();

    let mut client_conn = NetworkConnection::new(client_stream);
    let mut server_conn = NetworkConnection::new(server_stream);

    let event_count = 1000;
    let start = Instant::now();

    // Send events
    for i in 0..event_count {
        let event = Event::MouseMove { x: i, y: i };
        client_conn.send_event(event).await.unwrap();
    }

    // Receive and verify events
    for i in 0..event_count {
        let received = server_conn.receive_event().await.unwrap();
        assert!(matches!(received, Event::MouseMove { x, y } if x == i && y == i));
    }

    let duration = start.elapsed();
    println!("Processed {} events in {:?}", event_count, duration);
    println!("Events per second: {}", event_count as f64 / duration.as_secs_f64());
} 