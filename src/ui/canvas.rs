//! Immediate-mode UI canvas wrapping `egui`.

use egui::Context;

/// The root UI container mapped to the game window.
///
/// Veltrix uses an immediate-mode UI paradigm built on top of `egui` for
/// debug overlays, developer tools, and basic in-game UI.
#[derive(Debug, Clone)]
pub struct UICanvas {
    /// The global egui Context for the frame.
    pub ctx: Context,
    /// Whether the UI is currently capturing mouse clicks (hovering a window).
    pub wants_pointer_input: bool,
    /// Whether the UI is currently capturing keyboard input (typing in a text box).
    pub wants_keyboard_input: bool,
}

impl Default for UICanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl UICanvas {
    /// Create a new UI Canvas.
    pub fn new() -> Self {
        Self {
            ctx: Context::default(),
            wants_pointer_input: false,
            wants_keyboard_input: false,
        }
    }

    /// Begin a new UI frame. Returns true if the UI wants pointer input.
    pub fn begin_frame(&mut self, raw_input: egui::RawInput) {
        self.ctx.begin_frame(raw_input);
    }

    /// End the current UI frame and extract the shapes to render.
    pub fn end_frame(&mut self) -> egui::FullOutput {
        let output = self.ctx.end_frame();
        self.wants_pointer_input = self.ctx.wants_pointer_input();
        self.wants_keyboard_input = self.ctx.wants_keyboard_input();
        output
    }
}
