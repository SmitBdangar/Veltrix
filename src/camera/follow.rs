//! Smooth camera follow for characters.

use glam::Vec2;
use crate::ecs::world::Entity;

/// Component attached to the active camera to smoothly follow a target entity.
#[derive(Debug, Clone)]
pub struct CameraFollow {
    /// The ECS entity the camera should follow (often the player).
    pub target: Option<Entity>,
    /// Interpolation speed (`0.0 = no movement`, `1.0 = instant snap`).
    pub smoothing: f32,
    /// Distance from target before camera starts panning (deadzone radius).
    pub deadzone: f32,
    /// Lookahead offset applied when the target is moving.
    pub lookahead: Vec2,
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self {
            target: None,
            smoothing: 5.0,
            deadzone: 10.0,
            lookahead: Vec2::ZERO,
        }
    }
}

impl CameraFollow {
    pub fn new(target: Entity) -> Self {
        Self {
            target: Some(target),
            ..Default::default()
        }
    }

    /// Calculate the ideal new camera position given the current camera position,
    /// the target's position, and delta time.
    pub fn calculate_position(
        &self,
        current_cam_pos: Vec2,
        target_pos: Vec2,
        dt: f32,
    ) -> Vec2 {
        let diff = target_pos - current_cam_pos;
        let distance = diff.length();

        if distance > self.deadzone {
            let direction = if distance > 0.0 { diff / distance } else { Vec2::ZERO };
            let desired_pos = current_cam_pos + direction * (distance - self.deadzone) + self.lookahead;
            // Lerp towards the desired edge of the deadzone
            current_cam_pos.lerp(desired_pos, (self.smoothing * dt).clamp(0.0, 1.0))
        } else {
            current_cam_pos
        }
    }
}
