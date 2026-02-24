//! Orthographic 2D camera with viewport scaling and bounds.

use glam::{Mat4, Vec2};

/// A 2D orthographic camera for rendering scenes.
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// World-space position of the camera center.
    pub position: Vec2,
    /// Zoom level (1.0 is default, > 1.0 is zoomed in, < 1.0 is zoomed out).
    pub zoom: f32,
    /// Rotation in radians (CCW).
    pub rotation: f32,
    /// The logical viewport resolution (width, height) we are projecting to.
    viewport_size: Vec2,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(1280.0, 720.0),
        }
    }
}

impl Camera2D {
    /// Create a new camera centered at `(0, 0)` for a given viewport size.
    pub fn new(viewport_size: Vec2) -> Self {
        Self {
            viewport_size,
            ..Default::default()
        }
    }

    /// Update the internal viewport resolution (call this on window resize).
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width.max(1.0), height.max(1.0));
    }

    /// Calculate the view-projection matrix for the GPU shader.
    ///
    /// Projection: Orthographic projecting `[-half_width, half_width]` to `[-1, 1]`.
    /// View: Translates and rotates the world so the camera is at the origin.
    pub fn view_projection(&self) -> Mat4 {
        let half_w = self.viewport_size.x * 0.5 / self.zoom;
        let half_h = self.viewport_size.y * 0.5 / self.zoom;

        // Ortho projection mapping to wgpu coordinate space (y goes UP in NDC, but down in our game world conventionally?
        // We will assume Y goes UP in the world for standard 2D physics.
        let proj = Mat4::orthographic_rh(
            -half_w, half_w,
            -half_h, half_h,
            -1.0, 1.0, // Z depth
        );

        let view = Mat4::from_scale_rotation_translation(
            glam::Vec3::ONE,
            glam::Quat::from_rotation_z(-self.rotation),
            glam::Vec3::new(-self.position.x, -self.position.y, 0.0),
        );

        proj * view
    }

    /// Convert a screen coordinate (e.g. from mouse) to a world space coordinate.
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        // Normalised device coordinates (NDC): [-1, 1]
        let ndc_x = (screen_pos.x / self.viewport_size.x) * 2.0 - 1.0;
        // wgpu NDC y is inverted compared to window pixel Y
        let ndc_y = 1.0 - (screen_pos.y / self.viewport_size.y) * 2.0;
        
        // Un-project
        let inverse_vp = self.view_projection().inverse();
        let world_homo = inverse_vp * glam::Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
        
        Vec2::new(world_homo.x, world_homo.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_to_world_at_origin() {
        let cam = Camera2D::new(Vec2::new(800.0, 600.0));
        let world = cam.screen_to_world(Vec2::new(400.0, 300.0));
        assert!(world.length() < 1e-4);
    }
}
