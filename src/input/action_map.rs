//! Named action binding (e.g. mapping "jump" to Spacebar or gamepad A).

use std::collections::{HashMap, HashSet};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use super::{keyboard::KeyboardState, mouse::MouseState};

/// Represents a single input binding that can trigger an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
    /// A keyboard key.
    Key(KeyCode),
    /// A mouse button.
    Mouse(MouseButton),
    // (Gamepad buttons can be added here once we integrate gilrs deeply)
}

/// Maps human-readable action names to one or more physical bindings.
#[derive(Debug, Default)]
pub struct ActionMap {
    /// Maps an action name to a set of physical bounds.
    bindings: HashMap<String, HashSet<Binding>>,
}

impl ActionMap {
    /// Create an empty action map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind a physical input to a named action.
    ///
    /// # Example
    /// ```
    /// use veltrix::input::action_map::{ActionMap, Binding};
    /// use winit::keyboard::KeyCode;
    ///
    /// let mut map = ActionMap::new();
    /// map.bind("jump", Binding::Key(KeyCode::Space));
    /// ```
    pub fn bind(&mut self, action: &str, binding: Binding) {
        self.bindings
            .entry(action.to_string())
            .or_default()
            .insert(binding);
    }

    /// Clear all bindings for an action.
    pub fn unbind_all(&mut self, action: &str) {
        self.bindings.remove(action);
    }

    /// Returns `true` if any binding for the given action was pressed *this frame*.
    pub(crate) fn is_pressed(
        &self,
        action: &str,
        keyboard: &KeyboardState,
        mouse: &MouseState,
    ) -> bool {
        if let Some(bounds) = self.bindings.get(action) {
            for b in bounds {
                match b {
                    Binding::Key(k) => {
                        if keyboard.just_pressed(*k) {
                            return true;
                        }
                    }
                    Binding::Mouse(m) => {
                        if mouse.just_pressed(*m) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Returns `true` if any binding for the action is *currently held down*.
    pub fn is_held(
        &self,
        action: &str,
        keyboard: &KeyboardState,
        mouse: &MouseState,
    ) -> bool {
        if let Some(bounds) = self.bindings.get(action) {
            for b in bounds {
                match b {
                    Binding::Key(k) => {
                        if keyboard.is_pressed(*k) {
                            return true;
                        }
                    }
                    Binding::Mouse(m) => {
                        if mouse.is_pressed(*m) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binds_and_evaluates_correctly() {
        let mut map = ActionMap::new();
        map.bind("attack", Binding::Key(KeyCode::KeyZ));
        map.bind("attack", Binding::Mouse(MouseButton::Left));

        let mut kb = KeyboardState::default();
        let mut ms = MouseState::default();

        // Simulate Z pressed
        kb.process_event(KeyCode::KeyZ, true);
        assert!(map.is_pressed("attack", &kb, &ms));

        // Advance frame
        kb.begin_frame();
        ms.begin_frame();
        // Z is held, not just pressed
        assert!(!map.is_pressed("attack", &kb, &ms));
        assert!(map.is_held("attack", &kb, &ms));
    }
}
