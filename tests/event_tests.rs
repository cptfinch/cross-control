use rust_barrier::event::Event;
use serde_json;

#[test]
fn test_event_serialization() {
    let event = Event::MouseMove { x: 100, y: 200 };
    let serialized = serde_json::to_string(&event).unwrap();
    let deserialized: Event = serde_json::from_str(&serialized).unwrap();
    assert_eq!(event, deserialized);
} 