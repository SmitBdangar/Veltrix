//! UI Layout helpers (Vertical, Horizontal, Grid).

use egui::{Align, Layout as EguiLayout, Ui};

/// Start a vertical layout block.
pub fn vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(EguiLayout::top_down(Align::Min), add_contents)
        .inner
}

/// Start a horizontal layout block.
pub fn horizontal<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(EguiLayout::left_to_right(Align::Min), add_contents)
        .inner
}

/// Center the rest of the layout horizontally.
pub fn align_center<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(
        EguiLayout::top_down(Align::Center).with_cross_align(Align::Center),
        add_contents,
    )
    .inner
}

/// Right-align a layout.
pub fn align_right<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(EguiLayout::top_down(Align::Max), add_contents)
        .inner
}
