//! Mouse state tracking: position, buttons, scrolling, and cursor lock.

pub use winit::event::MouseButton;
use std::collections::HashSet;
use glam::Vec2;

/// Tracks mouse input state for the current frame.
#[derive(Debug, Default)]
pub struct MouseState {
    /// Buttons currently held down.
    pressed: HashSet<MouseButton>,
    /// Buttons pressed this exact frame.
    just_pressed: HashSet<MouseButton>,
    /// Buttons released this exact frame.
    just_released: HashSet<MouseButton>,
    /// Current mouse position in logical pixels relative to the window.
    pub position: Vec2,
    /// Mouse movement delta this frame (raw device events).
    pub delta: Vec2,
    /// Mouse wheel scroll delta this frame (lines/pixels).
    pub scroll: Vec2,
}

impl MouseState {
    /// Roll over frame-edge state. Must be called at the start of the frame.
    pub(crate) fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.delta = Vec2::ZERO;
        self.scroll = Vec2::ZERO;
    }

    /// Process a button event.
    pub(crate) fn process_button(&mut self, button: MouseButton, is_pressed: bool) {
        if is_pressed {
            if self.pressed.insert(button) {
                self.just_pressed.insert(button);
            }
        } else {
            if self.pressed.remove(&button) {
                self.just_released.insert(button);
            }
        }
    }

    /// Process cursor movement event.
    pub(crate) fn process_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    /// Process raw cursor movement delta for FPS lock.
    pub(crate) fn process_delta(&mut self, delta: Vec2) {
        self.delta += delta;
    }

    /// Process wheel scroll.
    pub(crate) fn process_scroll(&mut self, delta: Vec2) {
        self.scroll += delta;
    }

    /// Returns `true` if the button is currently held down.
    pub fn is_pressed(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }

    /// Returns `true` if the button was pressed this exact frame.
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed.contains(&button)
    }

    /// Returns `true` if the button was released this exact frame.
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.just_released.contains(&button)
    }
}
