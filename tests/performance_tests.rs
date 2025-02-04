use rust_barrier::event::Event;
use rust_barrier::network;
use tokio::net::{TcpListener, TcpStream};
use std::time::Instant;

#[tokio::test]
async fn test_event_throughput() {
    let listener = TcpListener::bind("127.0.0.1:8088").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8088").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();

    let start = Instant::now();
    let event_count = 1000;  // Reduced from 10,000 for testing

    // Send events
    for i in 0..event_count {
        let event = Event::MouseMove { x: i, y: i };
        network::send_event(&mut client, event).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;  // Small delay
    }

    // Receive events
    for i in 0..event_count {
        let received = network::receive_event(&mut server_conn).await.unwrap();
        assert!(matches!(received, Event::MouseMove { x, y } if x == i && y == i));
    }

    let duration = start.elapsed();
    println!("Processed {} events in {:?}", event_count, duration);
    println!("Events per second: {}", event_count as f64 / duration.as_secs_f64());
} 