use rust_barrier::event::Event;
use rust_barrier::network::{self, NetworkError};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use std::time::Duration;

#[tokio::test]
async fn test_basic_connectivity() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Test mouse movement
    let test_event = Event::MouseMove { x: 100, y: 200 };
    network::send_event(&mut client, test_event.clone()).await.unwrap();
    let received = network::receive_event(&mut server_conn).await.unwrap();
    assert_eq!(received, test_event);
}

#[tokio::test]
async fn test_key_events() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Test key press/release
    let key_press = Event::KeyPress { 
        code: 65, // 'A' key
        name: "A".to_string() 
    };
    network::send_event(&mut client, key_press.clone()).await.unwrap();
    let received = network::receive_event(&mut server_conn).await.unwrap();
    assert_eq!(received, key_press);
}

#[tokio::test]
async fn test_screen_switch() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8082").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Test screen transition
    let switch = Event::ScreenSwitch { 
        to_screen: "windows-pc".to_string() 
    };
    network::send_event(&mut client, switch.clone()).await.unwrap();
    let received = network::receive_event(&mut server_conn).await.unwrap();
    assert_eq!(received, switch);
}

#[tokio::test]
async fn test_connection_closed() {
    let listener = TcpListener::bind("127.0.0.1:8083").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8083").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Close the connection
    drop(client);
    
    // Try to receive - should error
    let result = network::receive_event(&mut server_conn).await;
    assert!(matches!(result, Err(NetworkError::Connection(_))));
}

#[tokio::test]
async fn test_invalid_json() {
    let listener = TcpListener::bind("127.0.0.1:8084").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8084").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    // Send invalid JSON - now using async write_all
    client.write_all(b"{invalid}").await.unwrap();
    
    let result = network::receive_event(&mut server_conn).await;
    assert!(matches!(result, Err(NetworkError::Serialization(_))));
}

#[tokio::test]
async fn test_all_event_types() {
    let listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8085").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();
    
    let events = vec![
        Event::MouseMove { x: 100, y: 200 },
        Event::MouseButton { button: 1, pressed: true },
        Event::KeyPress { code: 65, name: "A".to_string() },
        Event::KeyRelease { code: 65, name: "A".to_string() },
        Event::ScreenSwitch { to_screen: "windows-pc".to_string() },
        Event::Heartbeat,
        Event::Error("test error".to_string()),
    ];

    for event in events {
        network::send_event(&mut client, event.clone()).await.unwrap();
        let received = network::receive_event(&mut server_conn).await.unwrap();
        assert_eq!(received, event);
    }
} 