use rust_barrier::event::Event;
use rust_barrier::network::NetworkConnection;
use tokio::net::{TcpListener, TcpStream};

#[tokio::test]
async fn test_rapid_mouse_movement() {
    let listener = TcpListener::bind("127.0.0.1:8086").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8086").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut client_conn = NetworkConnection::new(client_stream);
    let mut server_conn = NetworkConnection::new(server_stream);

    // Simulate rapid mouse movement
    for i in 0..100 {
        let event = Event::MouseMove { x: i, y: i };
        client_conn.send_event(event).await.unwrap();
    }
    
    // Verify all events received in order
    for i in 0..100 {
        let received = server_conn.receive_event().await.unwrap();
        assert!(matches!(received, Event::MouseMove { x, y } if x == i && y == i));
    }
}

#[tokio::test]
async fn test_keyboard_combinations() {
    let listener = TcpListener::bind("127.0.0.1:8087").await.unwrap();
    let client_stream = TcpStream::connect("127.0.0.1:8087").await.unwrap();
    let (server_stream, _) = listener.accept().await.unwrap();
    
    let mut client_conn = NetworkConnection::new(client_stream);
    let mut server_conn = NetworkConnection::new(server_stream);

    let ctrl_press = Event::KeyPress { code: 37, name: "Control_L".to_string() };
    let c_press = Event::KeyPress { code: 54, name: "c".to_string() };
    
    client_conn.send_event(ctrl_press.clone()).await.unwrap();
    client_conn.send_event(c_press.clone()).await.unwrap();
    
    let received1 = server_conn.receive_event().await.unwrap();
    let received2 = server_conn.receive_event().await.unwrap();
    
    assert_eq!(received1, ctrl_press);
    assert_eq!(received2, c_press);
} 