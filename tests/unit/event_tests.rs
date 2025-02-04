use rust_barrier::event::Event;

#[test]
fn test_mouse_events() {
    let move_event = Event::MouseMove { x: -100, y: 500 };
    assert!(matches!(move_event, Event::MouseMove { x: -100, y: 500 }));

    // Test the new validation
    let valid_button = Event::new_mouse_button(1, true);
    assert!(valid_button.is_some());
    
    let invalid_button = Event::new_mouse_button(6, true);
    assert!(invalid_button.is_none());
}

#[test]
fn test_keyboard_events() {
    let valid_key = Event::new_key_press(65, "A".to_string());
    assert!(valid_key.is_some());
    
    let invalid_key = Event::new_key_press(65, "".to_string());
    assert!(invalid_key.is_none());
}

#[test]
fn test_screen_switch() {
    let switch = Event::ScreenSwitch { 
        to_screen: "linux-1".to_string() 
    };
    assert!(matches!(switch, Event::ScreenSwitch { to_screen } if to_screen == "linux-1"));
}

#[test]
fn test_control_events() {
    let heartbeat = Event::Heartbeat;
    assert!(matches!(heartbeat, Event::Heartbeat));

    let error = Event::Error("connection lost".to_string());
    assert!(matches!(error, Event::Error(msg) if msg == "connection lost"));
}

#[test]
fn test_event_constructors() {
    // Test mouse button validation
    assert!(Event::new_mouse_button(1, true).is_some());
    assert!(Event::new_mouse_button(6, true).is_none());
    
    // Test key press validation
    assert!(Event::new_key_press(65, "A".to_string()).is_some());
    assert!(Event::new_key_press(65, "".to_string()).is_none());
}