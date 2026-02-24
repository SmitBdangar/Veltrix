//! Tileset mapping tile indices to a texture atlas.

use crate::assets::handle::Handle;
use crate::math::Rect;
use crate::renderer::texture::Texture;
use serde::{Deserialize, Serialize};

/// Maps tile indices (IDs) to specific UV regions on a texture atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tileset {
    /// The name of this tileset.
    pub name: String,
    /// Handle to the texture atlas containing the tiles.
    #[serde(skip)]
    pub texture: Option<Handle<Texture>>,
    /// Width of the texture atlas in pixels.
    pub image_width: u32,
    /// Height of the texture atlas in pixels.
    pub image_height: u32,
    /// Tile width in pixels.
    pub tile_width: u32,
    /// Tile height in pixels.
    pub tile_height: u32,
    /// Starting tile ID (usually 1, since 0 is empty).
    pub first_gid: u32,
    /// Total number of tiles in this tileset.
    pub tile_count: u32,
}

impl Tileset {
    /// Calculate the source rectangle for a given tile ID.
    ///
    /// Returns `None` if the ID is outside this tileset's range.
    pub fn get_src_rect(&self, tile_id: u32) -> Option<Rect> {
        if tile_id < self.first_gid || tile_id >= self.first_gid + self.tile_count {
            return None;
        }

        let local_id = tile_id - self.first_gid;
        let columns = self.image_width / self.tile_width;

        if columns == 0 {
            return None;
        }

        let row = local_id / columns;
        let col = local_id % columns;

        let x = (col * self.tile_width) as f32;
        let y = (row * self.tile_height) as f32;

        Some(Rect::from_position_size(
            glam::Vec2::new(x, y),
            glam::Vec2::new(self.tile_width as f32, self.tile_height as f32),
        ))
    }
}
