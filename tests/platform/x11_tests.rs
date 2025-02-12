#[cfg(target_os = "linux")]
mod tests {
    use rust_barrier::platform::x11::X11Platform;
    use rust_barrier::event::Event;

    #[test]
    fn test_x11_connection() {
        let platform = X11Platform::new();
        assert!(platform.is_ok());
    }

    #[test]
    fn test_input_grab() {
        let platform = X11Platform::new().unwrap();
        assert!(platform.grab_input().is_ok());
    }

    #[test]
    fn test_mouse_movement() {
        let platform = X11Platform::new().unwrap();
        let event = Event::MouseMove { x: 100, y: 100 };
        assert!(platform.simulate_event(&event).is_ok());
    }
} 