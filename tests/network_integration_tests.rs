use rust_barrier::event::Event;
use rust_barrier::network::{NetworkConnection, NetworkError};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_basic_connectivity() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut client_conn = NetworkConnection::new(client_stream);
    let mut server_conn = NetworkConnection::new(server_stream);
    
    let test_event = Event::MouseMove { x: 100, y: 200 };
    client_conn.send_event(test_event.clone()).await.unwrap();
    let received = server_conn.receive_event().await.unwrap();
    assert_eq!(received, test_event);
}

#[tokio::test]
async fn test_event_transmission() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut client_conn = NetworkConnection::new(client_stream);
    let mut server_conn = NetworkConnection::new(server_stream);
    
    let events = vec![
        Event::MouseMove { x: 100, y: 200 },
        Event::MouseButton { button: 1, pressed: true },
        Event::KeyPress { code: 65, name: "A".to_string() },
        Event::Heartbeat,
    ];

    for event in events {
        client_conn.send_event(event.clone()).await.unwrap();
        let received = server_conn.receive_event().await.unwrap();
        assert_eq!(received, event);
    }
}

#[tokio::test]
async fn test_connection_closed() {
    let listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
    let client = TcpStream::connect("127.0.0.1:8085").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut server_conn = NetworkConnection::new(server_stream);
    
    // Drop client connection
    drop(client);
    
    let result = server_conn.receive_event().await;
    assert!(matches!(result, Err(NetworkError::Connection(_))));
}

#[tokio::test]
async fn test_invalid_data() {
    let listener = TcpListener::bind("127.0.0.1:8084").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8084").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut server_conn = NetworkConnection::new(server_stream);
    
    // Send invalid JSON with newline delimiter
    client.write_all(b"invalid json\n").await.unwrap();
    let result = server_conn.receive_event().await;
    assert!(matches!(result, Err(NetworkError::Serialization(_))));
}

#[tokio::test]
async fn test_screen_switch() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    let _client = TcpStream::connect("127.0.0.1:8082").await.unwrap();
    let (_server_conn, _) = listener.accept().await.unwrap();
    
    // ... rest of the test ...
}

#[tokio::test]
async fn test_event_throughput() {
    let listener = TcpListener::bind("127.0.0.1:8088").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8088").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let _client_conn = NetworkConnection::new(client_stream);
    let _server_conn = NetworkConnection::new(server_stream);
    
    // Test will be implemented later
    // This is just a placeholder
} 