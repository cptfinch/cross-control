#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, EventType, TransportType};

    #[test]
    fn test_event_serialization() {
        let event = Event {
            event_type: EventType::MouseMove { x: 100, y: 200 },
            transport: TransportType::Fast,
            sequence: 1,
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.sequence, deserialized.sequence);
        match deserialized.event_type {
            EventType::MouseMove { x, y } => {
                assert_eq!(x, 100);
                assert_eq!(y, 200);
            }
            _ => panic!("Wrong event type"),
        }
    }
} 