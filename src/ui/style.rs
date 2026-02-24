//! Theming and styling the egui UI.

use egui::{Color32, Visuals};

pub fn apply_dark_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.window_fill = Color32::from_black_alpha(200);
    visuals.panel_fill = Color32::from_black_alpha(220);
    ctx.set_visuals(visuals);
}

pub fn apply_light_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    visuals.window_fill = Color32::from_white_alpha(240);
    visuals.panel_fill = Color32::from_white_alpha(240);
    ctx.set_visuals(visuals);
}
