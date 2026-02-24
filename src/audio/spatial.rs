//! Spatial audio falloff helper.

use glam::Vec2;

/// A simple 2D spatial audio volume falloff configuration.
#[derive(Debug, Clone, Copy)]
pub struct SpatialSettings {
    /// Audio source position in world space.
    pub source_pos: Vec2,
    /// Listener position (usually the camera).
    pub listener_pos: Vec2,
    /// Minimum distance for maximum volume.
    pub min_distance: f32,
    /// Maximum distance where audio falls to 0.
    pub max_distance: f32,
}

impl SpatialSettings {
    /// Calculates the effective volume multiplier based on distance.
    pub fn calculate_volume(&self) -> f32 {
        let dist = self.source_pos.distance(self.listener_pos);
        if dist <= self.min_distance {
            return 1.0;
        }
        if dist >= self.max_distance {
            return 0.0;
        }
        
        // Linear falloff
        1.0 - (dist - self.min_distance) / (self.max_distance - self.min_distance)
    }
}
