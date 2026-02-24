//! Built-in performance and debug overlay.

use egui::{Align2, Color32, Window};
use crate::core::time::Time;

/// Renders a non-intrusive debug overlay showing FPS and frame times.
pub fn draw_debug_overlay(ui_canvas: &mut super::canvas::UICanvas, time: &Time) {
    let ctx = &ui_canvas.ctx;

    Window::new("Debug")
        .anchor(Align2::LEFT_TOP, [10.0, 10.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(egui::Frame::window(&ctx.style()).fill(Color32::from_black_alpha(150)))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("FPS: {:.1}", time.fps()))
                    .color(if time.fps() < 30.0 { Color32::RED } else { Color32::WHITE })
                    .strong(),
            );
            ui.label(format!("Frame Setup: {:.2}ms", time.delta_seconds() * 1000.0));
            // Add entity counts, memory usage, etc., in a fully scoped engine.
        });
}
