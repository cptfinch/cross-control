use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event as X11Event;
use x11rb::NONE;
use crate::event::Event;
use thiserror::Error;
use x11rb::rust_connection::RustConnection;
use xkbcommon::xkb;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Error, Debug)]
pub enum X11Error {
    #[error("X11 connection error: {0}")]
    ConnectionError(String),
    #[error("Failed to grab input: {0}")]
    GrabError(String),
    #[error("Event conversion error: {0}")]
    EventError(String),
}

pub struct X11Platform {
    conn: std::sync::Arc<RustConnection>,
    root: Window,
    screen_num: i32,
    keymap: xkb::Keymap,
}

impl X11Platform {
    pub fn new() -> Result<Self, X11Error> {
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| X11Error::ConnectionError(e.to_string()))?;
        
        let conn = std::sync::Arc::new(conn);
        let setup = conn.setup();
        let screen = setup.roots[screen_num].root;
        
        // Load keymap with proper parameters
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_names(
            &ctx,
            Some("evdev".to_string()),  // rules: Option<String>
            None,                       // model: Option<String>
            Some("us".to_string()),     // layout: Option<String>
            None,                       // variant: Option<String>
            None,                       // options: Option<String>
            xkb::KEYMAP_COMPILE_NO_FLAGS
        ).ok_or_else(|| X11Error::ConnectionError("Failed to load keymap".into()))?;

        Ok(Self {
            conn,
            root: screen,
            screen_num: screen_num.try_into().unwrap(),
            keymap,
        })
    }

    pub fn grab_input(&self) -> Result<(), X11Error> {
        // Grab keyboard
        self.conn.grab_keyboard(
            false,
            self.root,
            x11rb::CURRENT_TIME,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        ).map_err(|e| X11Error::GrabError(e.to_string()))?
        .reply()
        .map_err(|e| X11Error::GrabError(e.to_string()))?;

        // Grab pointer (mouse)
        self.conn.grab_pointer(
            false,
            self.root,
            EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE | 
            EventMask::POINTER_MOTION | EventMask::BUTTON_MOTION,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
            self.root,
            NONE,
            x11rb::CURRENT_TIME,
        ).map_err(|e| X11Error::GrabError(e.to_string()))?
        .reply()
        .map_err(|e| X11Error::GrabError(e.to_string()))?;

        Ok(())
    }

    pub fn convert_to_event(&self, x_event: X11Event) -> Result<Option<Event>, X11Error> {
        match x_event {
            X11Event::MotionNotify(motion) => {
                Ok(Some(Event::MouseMove {
                    x: motion.event_x.into(),
                    y: motion.event_y.into(),
                }))
            },
            X11Event::ButtonPress(button) => {
                Ok(Some(Event::MouseButton {
                    button: button.detail,
                    pressed: true,
                }))
            },
            X11Event::ButtonRelease(button) => {
                Ok(Some(Event::MouseButton {
                    button: button.detail,
                    pressed: false,
                }))
            },
            X11Event::KeyPress(key) => {
                Ok(Some(Event::KeyPress {
                    code: key.detail as u16,
                    name: String::new(), // TODO: Implement key name lookup
                }))
            },
            X11Event::KeyRelease(key) => {
                Ok(Some(Event::KeyRelease {
                    code: key.detail as u16,
                    name: String::new(), // TODO: Implement key name lookup
                }))
            },
            _ => Ok(None),
        }
    }

    pub fn simulate_event(&self, event: &Event) -> Result<(), X11Error> {
        match event {
            Event::MouseMove { x, y } => {
                self.conn.warp_pointer(
                    NONE,
                    self.root,
                    0, 0,
                    0, 0,
                    *x as i16, *y as i16,
                ).map_err(|e| X11Error::EventError(e.to_string()))?;
            },
            Event::MouseButton { button, pressed } => {
                let event = if *pressed {
                    ButtonPressEvent {
                        response_type: 4,
                        detail: *button,
                        sequence: 0,
                        time: x11rb::CURRENT_TIME,
                        root: self.root,
                        event: self.root,
                        child: NONE,
                        root_x: 0,
                        root_y: 0,
                        event_x: 0,
                        event_y: 0,
                        state: 0u16.into(),
                        same_screen: true,
                    }
                } else {
                    ButtonPressEvent {
                        response_type: 5,
                        detail: *button,
                        sequence: 0,
                        time: x11rb::CURRENT_TIME,
                        root: self.root,
                        event: self.root,
                        child: NONE,
                        root_x: 0,
                        root_y: 0,
                        event_x: 0,
                        event_y: 0,
                        state: 0u16.into(),
                        same_screen: true,
                    }
                };

                self.conn.send_event(
                    false,
                    self.root,
                    EventMask::BUTTON_PRESS,
                    &event,
                ).map_err(|e| X11Error::EventError(e.to_string()))?;
            },
            _ => {},
        }
        self.conn.flush().map_err(|e| X11Error::EventError(e.to_string()))?;
        Ok(())
    }

    pub fn run_event_loop(&self) -> Result<(), X11Error> {
        let event_mask = EventMask::POINTER_MOTION | 
            EventMask::BUTTON_PRESS | 
            EventMask::BUTTON_RELEASE |
            EventMask::KEY_PRESS |
            EventMask::KEY_RELEASE;

        self.conn.change_window_attributes(
            self.root,
            &ChangeWindowAttributesAux::default().event_mask(event_mask)
        )?.check()?;

        loop {
            let event = self.conn.wait_for_event()?;
            match event {
                X11Event::MotionNotify(ev) => {
                    let translated = Event::MouseMove { 
                        x: ev.event_x.into(), 
                        y: ev.event_y.into() 
                    };
                    self.send_to_network(translated);
                },
                X11Event::ButtonPress(ev) => {
                    let translated = Event::MouseButton { 
                        button: ev.detail, 
                        pressed: true 
                    };
                    self.send_to_network(translated);
                },
                X11Event::KeyPress(ev) => {
                    let code: u16 = ev.detail.into();
                    if let Some(key) = self.keycode_to_name(code) {
                        let translated = Event::KeyPress { 
                            code, 
                            name: key 
                        };
                        self.send_to_network(translated);
                    }
                },
                _ => {}
            }
        }
    }

    fn keycode_to_name(&self, code: u16) -> Option<String> {
        let keycode = xkb::Keycode::from(code as u32 + 8);  // X11 offset
        let state = xkb::State::new(&self.keymap);
        
        // Get keysyms
        let keysyms = state.key_get_syms(keycode);
        if keysyms.is_empty() {
            return None;
        }

        // Prepare buffer for keysym name
        let mut buffer = [0 as c_char; 64];
        let len = unsafe {
            xkb::ffi::xkb_keysym_get_name(
                keysyms[0],
                buffer.as_mut_ptr(),
                buffer.len()
            )
        };

        if len <= 0 {
            return None;
        }

        // Convert buffer to CStr and then to String
        unsafe {
            CStr::from_ptr(buffer.as_ptr())
                .to_str()
                .ok()
                .map(|s| s.to_string())
        }
    }

    fn send_to_network(&self, event: Event) {
        // Placeholder implementation
        println!("[DEBUG] Sending event: {:?}", event);
    }
}

impl Drop for X11Platform {
    fn drop(&mut self) {
        // Cleanup: ungrab keyboard and pointer
        let _ = self.conn.ungrab_keyboard(x11rb::CURRENT_TIME);
        let _ = self.conn.ungrab_pointer(x11rb::CURRENT_TIME);
        let _ = self.conn.flush();
    }
}

impl From<x11rb::errors::ConnectionError> for X11Error {
    fn from(e: x11rb::errors::ConnectionError) -> Self {
        X11Error::ConnectionError(e.to_string())
    }
}

impl From<x11rb::errors::ReplyError> for X11Error {
    fn from(e: x11rb::errors::ReplyError) -> Self {
        X11Error::GrabError(e.to_string())
    }
} 