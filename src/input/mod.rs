//! Input handling: keyboard, mouse, gamepad, and action mapping.

pub mod action_map;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;

pub use action_map::ActionMap;
pub use keyboard::KeyboardState;
pub use mouse::MouseState;

use crate::math::Vec2;

/// Central input manager — owns keyboard, mouse, and gamepad state.
#[derive(Debug, Default)]
pub struct InputManager {
    /// Keyboard state for this frame.
    pub keyboard: KeyboardState,
    /// Mouse state for this frame.
    pub mouse: MouseState,
    /// Named action bindings.
    pub actions: ActionMap,
}

impl InputManager {
    /// Creates a new input manager with empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Call once at the start of each frame to roll over frame-edge state.
    pub fn begin_frame(&mut self) {
        self.keyboard.begin_frame();
        self.mouse.begin_frame();
    }

    /// Returns `true` if the named action was triggered this frame.
    pub fn action_pressed(&self, name: &str) -> bool {
        self.actions.is_pressed(name, &self.keyboard, &self.mouse)
    }
}
