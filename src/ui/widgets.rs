//! Convenience wrappers for standard UI widgets.

use egui::{Button, Label, ProgressBar, Slider, Ui};

/// A simple push button.
pub fn button(ui: &mut Ui, text: &str) -> bool {
    ui.add(Button::new(text)).clicked()
}

/// A static text label.
pub fn label(ui: &mut Ui, text: &str) {
    ui.add(Label::new(text));
}

/// A slider for adjusting numeric values.
pub fn slider(ui: &mut Ui, value: &mut f32, range: std::ops::RangeInclusive<f32>, text: &str) -> bool {
    ui.add(Slider::new(value, range).text(text)).changed()
}

/// A progress bar.
pub fn progress_bar(ui: &mut Ui, progress: f32, text: &str) {
    ui.add(ProgressBar::new(progress).text(text));
}

/// A toggle checkbox.
pub fn checkbox(ui: &mut Ui, checked: &mut bool, text: &str) -> bool {
    ui.checkbox(checked, text).changed()
}
