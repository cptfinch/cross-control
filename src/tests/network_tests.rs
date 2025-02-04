#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, EventType, TransportType};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_udp_send_receive() {
        let (tx, mut rx) = mpsc::channel(32);
        
        // Start UDP server
        let server_addr = "127.0.0.1:8082";
        tokio::spawn(async move {
            let _ = udp::run_udp_server(tx).await;
        });

        // Wait for server to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Send test event
        let test_event = Event {
            event_type: EventType::MouseMove { x: 100, y: 200 },
            transport: TransportType::Fast,
            sequence: 1,
        };

        let result = udp::send_event_udp(test_event.clone(), server_addr).await;
        assert!(result.is_ok());

        // Check received event
        if let Some(received_event) = rx.recv().await {
            assert_eq!(received_event.sequence, test_event.sequence);
            match received_event.event_type {
                EventType::MouseMove { x, y } => {
                    assert_eq!(x, 100);
                    assert_eq!(y, 200);
                }
                _ => panic!("Wrong event type received"),
            }
        } else {
            panic!("No event received");
        }
    }
} 