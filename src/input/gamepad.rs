//! Gamepad support using the `gilrs` library.
//!
//! Note: Deep integration with the `ActionMap` requires pumping the `Gilrs` event
//! loop each frame. This module provides a basic wrapper.

use gilrs::{Button, Event, EventType, Gilrs};
use std::collections::HashSet;

/// Tracks gamepad state across all connected controllers.
pub struct GamepadState {
    /// The gilrs context.
    pub gilrs: Gilrs,
    /// Buttons pressed *this frame* across any connected gamepad.
    pub just_pressed: HashSet<Button>,
    /// Buttons released *this frame*.
    pub just_released: HashSet<Button>,
}

impl std::fmt::Debug for GamepadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GamepadState")
            .field("just_pressed", &self.just_pressed)
            .field("just_released", &self.just_released)
            .finish()
    }
}

impl Default for GamepadState {
    fn default() -> Self {
        Self::new()
    }
}

impl GamepadState {
    /// Initialize gamepad tracking.
    pub fn new() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                log::warn!("Failed to initialize gilrs (gamepad support): {e}");
                // Return a dummy Gilrs instance if initialization fails
                // In a real engine we might use an Option<Gilrs>
                Gilrs::new().unwrap_or_else(|_| panic!("Fatal gilrs failure"))
            }
        };

        Self {
            gilrs,
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    /// Process gamepad events for the current frame.
    ///
    /// Must be called once per frame from the main loop BEFORE checking inputs.
    pub(crate) fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();

        // Pump all pending gilrs events
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    self.just_pressed.insert(button);
                }
                EventType::ButtonReleased(button, _) => {
                    self.just_released.insert(button);
                }
                _ => {}
            }
        }
    }

    /// Returns `true` if the button on ANY connected gamepad is currently held.
    pub fn is_pressed(&self, button: Button) -> bool {
        for (_id, gamepad) in self.gilrs.gamepads() {
            if gamepad.is_pressed(button) {
                return true;
            }
        }
        false
    }

    /// Returns `true` if the button on ANY gamepad was pressed this exact frame.
    pub fn just_pressed(&self, button: Button) -> bool {
        self.just_pressed.contains(&button)
    }

    /// Returns `true` if the button on ANY gamepad was released this exact frame.
    pub fn just_released(&self, button: Button) -> bool {
        self.just_released.contains(&button)
    }
}
