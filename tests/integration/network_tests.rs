use rust_barrier::event::Event;
use rust_barrier::network::{self, NetworkError};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_basic_connectivity() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    let test_event = Event::MouseMove { x: 100, y: 200 };
    network::send_event(&mut client, test_event.clone()).await.unwrap();
    let received = network::receive_event(&mut server_conn).await.unwrap();
    assert_eq!(received, test_event);
}

#[tokio::test]
async fn test_event_transmission() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    let events = vec![
        Event::MouseMove { x: 100, y: 200 },
        Event::MouseButton { button: 1, pressed: true },
        Event::KeyPress { code: 65, name: "A".to_string() },
        Event::Heartbeat,
    ];

    for event in events {
        network::send_event(&mut client, event.clone()).await.unwrap();
        let received = network::receive_event(&mut server_conn).await.unwrap();
        assert_eq!(received, event);
    }
}

#[tokio::test]
async fn test_connection_errors() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8082").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Test connection closure
    drop(client);
    let result = network::receive_event(&mut server_conn).await;
    assert!(matches!(result, Err(NetworkError::Connection(_))));
}

#[tokio::test]
async fn test_invalid_data() {
    let listener = TcpListener::bind("127.0.0.1:8083").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8083").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Send invalid JSON
    client.write_all(b"{invalid}").await.unwrap();
    let result = network::receive_event(&mut server_conn).await;
    assert!(matches!(result, Err(NetworkError::Serialization(_))));
} 