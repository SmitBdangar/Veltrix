//! Batches sprites for drawing.

use crate::scene::components::Sprite;
use crate::math::Transform2D;

/// Submits sprites for instanced rendering.
#[derive(Default, Debug)]
pub struct SpriteBatcher {}

impl SpriteBatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_sprite(&mut self, _sprite: &Sprite, _transform: &Transform2D, _z_index: f32) {
    }
}
