use rust_barrier::event::Event;
use rust_barrier::network;
use tokio::net::{TcpListener, TcpStream};

#[tokio::test]
async fn test_rapid_mouse_movement() {
    let listener = TcpListener::bind("127.0.0.1:8086").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8086").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();

    // Simulate rapid mouse movement
    for i in 0..100 {
        let event = Event::MouseMove { x: i, y: i };
        network::send_event(&mut client, event).await.unwrap();
    }
    
    // Verify all events received in order
    for i in 0..100 {
        let received = network::receive_event(&mut server_conn).await.unwrap();
        assert!(matches!(received, Event::MouseMove { x, y } if x == i && y == i));
    }
}

#[tokio::test]
async fn test_keyboard_combinations() {
    let listener = TcpListener::bind("127.0.0.1:8087").await.unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8087").await.unwrap();
    let (mut server_conn, _) = listener.accept().await.unwrap();

    // Test Ctrl+C combination
    let ctrl_press = Event::KeyPress { code: 17, name: "Ctrl".to_string() };
    let c_press = Event::KeyPress { code: 67, name: "C".to_string() };
    
    network::send_event(&mut client, ctrl_press.clone()).await.unwrap();
    network::send_event(&mut client, c_press.clone()).await.unwrap();
    
    let received1 = network::receive_event(&mut server_conn).await.unwrap();
    let received2 = network::receive_event(&mut server_conn).await.unwrap();
    
    assert_eq!(received1, ctrl_press);
    assert_eq!(received2, c_press);
} 