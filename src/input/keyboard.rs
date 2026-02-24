//! Keyboard state tracking.

pub use winit::keyboard::KeyCode;
use std::collections::HashSet;

/// Tracks keyboard input state for the current frame.
#[derive(Debug, Default)]
pub struct KeyboardState {
    /// Keys currently held down.
    pressed: HashSet<KeyCode>,
    /// Keys pressed this exact frame.
    just_pressed: HashSet<KeyCode>,
    /// Keys released this exact frame.
    just_released: HashSet<KeyCode>,
}

impl KeyboardState {
    /// Roll over frame-edge state. Must be called at the start of the frame.
    pub(crate) fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Process a raw winit keyboard input event.
    pub(crate) fn process_event(&mut self, keycode: KeyCode, is_pressed: bool) {
        if is_pressed {
            if self.pressed.insert(keycode) {
                self.just_pressed.insert(keycode);
            }
        } else {
            if self.pressed.remove(&keycode) {
                self.just_released.insert(keycode);
            }
        }
    }

    /// Returns `true` if the key is currently held down.
    pub fn is_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns `true` if the key was pressed this exact frame.
    pub fn just_pressed(&self, keycode: KeyCode) -> bool {
        self.just_pressed.contains(&keycode)
    }

    /// Returns `true` if the key was released this exact frame.
    pub fn just_released(&self, keycode: KeyCode) -> bool {
        self.just_released.contains(&keycode)
    }
}
