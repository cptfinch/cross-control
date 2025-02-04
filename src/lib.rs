pub mod event;
pub mod network;

// Re-export main types for convenience
pub use event::{Event, EventType, TransportType};

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[test]
    fn test_event_creation() {
        let event = event::Event {
            event_type: event::EventType::MouseMove { x: 100, y: 200 },
            transport: event::TransportType::Fast,
            sequence: 1,
        };
        assert_matches!(event.event_type, event::EventType::MouseMove { x: 100, y: 200 });
    }
} 