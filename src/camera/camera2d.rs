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
    pub viewport_size: Vec2,
    /// The fixed virtual resolution. If set, the camera will letterbox to maintain this aspect ratio.
    pub virtual_resolution: Option<Vec2>,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(1280.0, 720.0),
            virtual_resolution: None,
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

    /// Calculate the physical viewport rect `(x, y, width, height)` for `wgpu::RenderPass::set_viewport`.
    pub fn calculate_viewport_rect(&self) -> (f32, f32, f32, f32) {
        if let Some(target) = self.virtual_resolution {
            let target_aspect = target.x / target.y;
            let window_aspect = self.viewport_size.x / self.viewport_size.y;
            
            if window_aspect > target_aspect {
                // Window is wider than target. Pillarbox (bars on left/right)
                let scale = self.viewport_size.y / target.y;
                let scaled_width = target.x * scale;
                let x_offset = (self.viewport_size.x - scaled_width) / 2.0;
                (x_offset, 0.0, scaled_width, self.viewport_size.y)
            } else {
                // Window is taller than target. Letterbox (bars on top/bottom)
                let scale = self.viewport_size.x / target.x;
                let scaled_height = target.y * scale;
                let y_offset = (self.viewport_size.y - scaled_height) / 2.0;
                (0.0, y_offset, self.viewport_size.x, scaled_height)
            }
        } else {
            (0.0, 0.0, self.viewport_size.x, self.viewport_size.y)
        }
    }

    /// Calculate the view-projection matrix for the GPU shader.
    ///
    /// Projection: Orthographic projecting `[-half_width, half_width]` to `[-1, 1]`.
    /// View: Translates and rotates the world so the camera is at the origin.
    pub fn view_projection(&self) -> Mat4 {
        let size = self.virtual_resolution.unwrap_or(self.viewport_size);
        let half_w = size.x * 0.5 / self.zoom;
        let half_h = size.y * 0.5 / self.zoom;

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
        let (vx, vy, vw, vh) = self.calculate_viewport_rect();
        
        // Adjust screen pos by viewport offset
        let local_x = screen_pos.x - vx;
        let local_y = screen_pos.y - vy;

        // Normalised device coordinates (NDC): [-1, 1]
        let ndc_x = (local_x / vw) * 2.0 - 1.0;
        // wgpu NDC y is inverted compared to window pixel Y
        let ndc_y = 1.0 - (local_y / vh) * 2.0;
        
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
