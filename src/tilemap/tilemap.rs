//! 2D grid of tile indices.

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Represents a 2D tilemap layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tilemap {
    /// Width in tiles.
    pub width: u32,
    /// Height in tiles.
    pub height: u32,
    /// Width of a single tile in pixels.
    pub tile_width: u32,
    /// Height of a single tile in pixels.
    pub tile_height: u32,
    /// Flat array of tile indices (0 usually means empty).
    pub tiles: Vec<u32>,
}

impl Tilemap {
    /// Create a new tilemap of given dimensions filled with `0` (empty) tiles.
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Self {
        Self {
            width,
            height,
            tile_width,
            tile_height,
            tiles: vec![0; (width * height) as usize],
        }
    }

    /// Get the tile index at `(x, y)`. Returns `None` if out of bounds.
    pub fn get_tile(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.tiles[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Set the tile index at `(x, y)`. Returns `true` if successful.
    pub fn set_tile(&mut self, x: u32, y: u32, index: u32) -> bool {
        if x < self.width && y < self.height {
            self.tiles[(y * self.width + x) as usize] = index;
            true
        } else {
            false
        }
    }

    /// Convert a world position to grid `(x, y)` coordinates.
    pub fn world_to_grid(&self, world_pos: Vec2) -> Option<(u32, u32)> {
        let x = (world_pos.x / self.tile_width as f32).floor();
        let y = (world_pos.y / self.tile_height as f32).floor();

        if x >= 0.0 && y >= 0.0 && x < self.width as f32 && y < self.height as f32 {
            Some((x as u32, y as u32))
        } else {
            None
        }
    }

    /// Get the world-space AABB of a specific tile.
    pub fn tile_bounds(&self, x: u32, y: u32) -> crate::math::Rect {
        let min_x = (x * self.tile_width) as f32;
        let min_y = (y * self.tile_height) as f32;
        crate::math::Rect::from_position_size(
            Vec2::new(min_x, min_y),
            Vec2::new(self.tile_width as f32, self.tile_height as f32),
        )
    }
}
