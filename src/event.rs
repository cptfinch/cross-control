// src/event.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Event {
    MouseMove { x: i32, y: i32 },
    MouseButton { button: u8, pressed: bool },
    KeyPress { code: u16, name: String },  // Platform-independent key codes
    KeyRelease { code: u16, name: String },
    ScreenSwitch { to_screen: String },    // Screen identifier
    Heartbeat,
    Error(String),
}

// Platform detection
#[cfg(target_os = "linux")]
pub mod platform {
    pub use x11rb::protocol::xproto::{Keycode, Button};
}

#[cfg(target_os = "windows")]
pub mod platform {
    pub use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, MOUSE_EVENT};
}
