use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, GrabMode, EventMask};
use x11rb::protocol::xproto::Window;
use x11rb::rust_connection::RustConnection;
use thiserror::Error;
use crate::event::Event;
use xkbcommon::xkb;
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum X11Error {
    #[error("X11 connection error: {0}")]
    ConnectionError(String),
    #[error("X11 reply error: {0}")]
    ReplyError(String),
    #[error("Failed to grab input: {0}")]
    GrabError(String),
    #[error("Keymap error: {0}")]
    KeymapError(String),
    #[error("XKB error: {0}")]
    XkbError(String),
}

pub struct X11Platform {
    conn: Arc<RustConnection>,
    root: Window,
    keymap: xkb::Keymap,
    state: xkb::State,
}

impl X11Platform {
    pub fn new() -> Result<Self, X11Error> {
        // Connect to X server
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| X11Error::ConnectionError(e.to_string()))?;
        let conn = Arc::new(conn);
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;

        // Initialize XKB
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

        // Create keymap using Keymap::new_from_names
        let keymap = xkb::Keymap::new_from_names(
            &context,
            None,     // rules
            None,     // model
            None,     // layout
            None,     // variant
            None,     // options
            xkb::KEYMAP_COMPILE_NO_FLAGS
        ).ok_or_else(|| X11Error::KeymapError("Failed to create keymap".to_string()))?;

        let state = xkb::State::new(&keymap);

        Ok(Self {
            conn,
            root,
            keymap,
            state,
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
            self.root,
            x11rb::CURRENT_TIME,
        ).map_err(|e| X11Error::GrabError(e.to_string()))?
        .reply()
        .map_err(|e| X11Error::GrabError(e.to_string()))?;

        Ok(())
    }

    pub fn run_event_loop<F>(&self, mut callback: F) -> Result<(), X11Error> 
    where
        F: FnMut(Event)
    {
        loop {
            let event = self.conn.wait_for_event()?;
            if let Some(our_event) = self.convert_to_event(event)? {
                callback(our_event);
            }
        }
    }

    pub fn simulate_event(&self, event: &Event) -> Result<(), X11Error> {
        match event {
            Event::MouseMove { x, y } => {
                self.conn.warp_pointer(
                    self.root,
                    self.root,
                    0, 0,
                    0, 0,
                    *x as i16, *y as i16,
                ).map_err(|e| X11Error::GrabError(e.to_string()))?;
            }
            Event::MouseButton { button: _, pressed: _ } => {
                // TODO: Implement mouse button simulation
            }
            Event::KeyPress { code: _, .. } | Event::KeyRelease { code: _, .. } => {
                // TODO: Implement keyboard event simulation
            }
            _ => {}
        }
        self.conn.flush().map_err(|e| X11Error::GrabError(e.to_string()))?;
        Ok(())
    }

    fn convert_to_event(&self, x_event: x11rb::protocol::Event) -> Result<Option<Event>, X11Error> {
        match x_event {
            x11rb::protocol::Event::MotionNotify(motion) => {
                Ok(Some(Event::MouseMove {
                    x: motion.event_x.into(),
                    y: motion.event_y.into(),
                }))
            }
            x11rb::protocol::Event::ButtonPress(button) => {
                Ok(Some(Event::MouseButton {
                    button: button.detail,
                    pressed: true,
                }))
            }
            x11rb::protocol::Event::ButtonRelease(button) => {
                Ok(Some(Event::MouseButton {
                    button: button.detail,
                    pressed: false,
                }))
            }
            x11rb::protocol::Event::KeyPress(key) => {
                let keycode = key.detail as u32;
                let key_name = self.keymap.key_get_name(keycode)
                    .ok_or(X11Error::KeymapError("Invalid keycode".to_string()))?
                    .to_string();
                
                Ok(Some(Event::KeyPress {
                    code: key.detail as u16,
                    name: key_name,
                }))
            }
            x11rb::protocol::Event::KeyRelease(key) => {
                let keycode = key.detail as u32;
                let key_name = self.keymap.key_get_name(keycode)
                    .ok_or(X11Error::KeymapError("Invalid keycode".to_string()))?
                    .to_string();
                
                Ok(Some(Event::KeyRelease {
                    code: key.detail as u16,
                    name: key_name,
                }))
            }
            _ => Ok(None),
        }
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
        X11Error::ReplyError(e.to_string())
    }
} 