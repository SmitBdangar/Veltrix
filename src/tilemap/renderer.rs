//! Efficient chunk-based rendering for tilemaps.

use crate::math::Transform2D;
use crate::renderer::{color::Color, sprite_batcher::SpriteBatcher};
use crate::scene::components::Sprite;
use super::{tilemap::Tilemap, tileset::Tileset};

/// Helper to render a tilemap efficiently using the SpriteBatcher.
/// In a fully optimized engine, this would build static vertex buffers per chunk.
/// Here we submit visible tiles to the dynamic sprite batcher.
pub struct TilemapRenderer;

impl TilemapRenderer {
    /// Render the visible portion of a tilemap.
    ///
    /// `bounds` is the camera's visible AABB in world space.
    pub fn render(
        tilemap: &Tilemap,
        tileset: &Tileset,
        transform: &Transform2D,
        camera_bounds: &crate::math::Rect,
        batcher: &mut SpriteBatcher,
        z_index: f32,
    ) {
        if let Some(texture) = &tileset.texture {
            // Determine grid bounds based on camera
            // We expand the bounds slightly to prevent pop-in at the edges
            let expanded_bounds = camera_bounds.expanded(
                (tilemap.tile_width.max(tilemap.tile_height) as f32) * 2.0
            );

            // Convert world bounds to grid coordinates relative to the tilemap's transform
            let min_local = expanded_bounds.min - transform.position;
            let max_local = expanded_bounds.max - transform.position;

            let min_x = (min_local.x / tilemap.tile_width as f32).floor().max(0.0) as u32;
            let min_y = (min_local.y / tilemap.tile_height as f32).floor().max(0.0) as u32;

            let max_x = ((max_local.x / tilemap.tile_width as f32).ceil() as u32)
                .min(tilemap.width);
            let max_y = ((max_local.y / tilemap.tile_height as f32).ceil() as u32)
                .min(tilemap.height);

            for y in min_y..max_y {
                for x in min_x..max_x {
                    if let Some(tile_id) = tilemap.get_tile(x, y) {
                        if tile_id == 0 {
                            continue; // Empty tile
                        }

                        if let Some(src_rect) = tileset.get_src_rect(tile_id) {
                            let mut tile_transform = *transform;
                            tile_transform.position += glam::Vec2::new(
                                (x * tilemap.tile_width) as f32,
                                (y * tilemap.tile_height) as f32,
                            );

                            let sprite = Sprite {
                                texture: *texture,
                                color: Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                src_rect: Some(src_rect),
                            };

                            batcher.draw_sprite(&sprite, &tile_transform, z_index);
                        }
                    }
                }
            }
        }
    }
}
