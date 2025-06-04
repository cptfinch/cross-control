#[cfg(target_os = "linux")]
mod linux_tests {
    use rust_barrier::event::{platform::*, Event};

    #[test]
    fn test_linux_key_mapping() {
        // Test X11 keycodes to our Event mapping
        let x11_key = Keycode::from(65); // 'A' in X11

        // Convert using the library helper
        let event = Event::new_key_press(x11_key.into(), "A".to_string()).unwrap();

        assert_eq!(event, Event::KeyPress { code: 65, name: "A".to_string() });
    }
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use rust_barrier::event::{platform::*, Event};

    #[test]
    fn test_windows_key_mapping() {
        let vk = VIRTUAL_KEY(0x41); // 'A' in Windows

        let event = Event::new_key_press(vk.0 as u16, "A".to_string()).unwrap();

        assert_eq!(event, Event::KeyPress { code: 0x41, name: "A".to_string() });
    }
}
