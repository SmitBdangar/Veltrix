//! 2D transform: position, rotation (radians), scale, and matrix conversion.

use glam::{Mat3, Mat4, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// A 2D spatial transform consisting of position, rotation, and scale.
///
/// Rotation is stored in **radians** (counter-clockwise positive).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    /// World-space position.
    pub position: Vec2,
    /// Rotation in radians (CCW positive).
    pub rotation: f32,
    /// Scale factor. `(1, 1)` is identity.
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform2D {
    /// The identity transform — position `(0,0)`, rotation `0`, scale `(1,1)`.
    pub const IDENTITY: Self = Self {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    /// Construct a transform with the given position, zero rotation, and unit scale.
    pub fn from_position(position: Vec2) -> Self {
        Self { position, ..Self::IDENTITY }
    }

    /// Construct a transform with given position and rotation.
    pub fn from_position_rotation(position: Vec2, rotation: f32) -> Self {
        Self { position, rotation, scale: Vec2::ONE }
    }

    /// Construct a full transform.
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self { position, rotation, scale }
    }

    /// Convert to a 3×3 affine matrix (suitable for 2D computation).
    pub fn to_mat3(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(self.scale, self.rotation, self.position)
    }

    /// Convert to a 4×4 homogeneous matrix (suitable for GPU shaders).
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale.x, self.scale.y, 1.0),
            glam::Quat::from_rotation_z(self.rotation),
            Vec3::new(self.position.x, self.position.y, 0.0),
        )
    }

    /// Local right direction vector.
    pub fn right(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }

    /// Local up direction vector.
    pub fn up(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }

    /// Translate by a world-space offset.
    pub fn translate(&mut self, offset: Vec2) {
        self.position += offset;
    }

    /// Rotate by `delta` radians.
    pub fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
    }

    /// Linearly interpolate toward `other` by factor `t ∈ [0, 1]`.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            position: self.position.lerp(other.position, t),
            rotation: self.rotation + (other.rotation - self.rotation) * t,
            scale: self.scale.lerp(other.scale, t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn identity_mat4_is_valid() {
        let t = Transform2D::IDENTITY;
        let m = t.to_mat4();
        // A 4×4 identity-ish matrix for (0,0), 0-rot, (1,1) scale.
        assert!((m.col(0).x - 1.0).abs() < 1e-6);
        assert!((m.col(1).y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn right_and_up_are_perpendicular() {
        let t = Transform2D::from_position_rotation(Vec2::ZERO, FRAC_PI_2);
        let dot = t.right().dot(t.up());
        assert!(dot.abs() < 1e-6, "right and up must be perpendicular, dot={dot}");
    }

    #[test]
    fn lerp_midpoint_is_midpoint() {
        let a = Transform2D::from_position(Vec2::ZERO);
        let b = Transform2D::from_position(Vec2::new(10.0, 0.0));
        let mid = a.lerp(&b, 0.5);
        assert!((mid.position.x - 5.0).abs() < 1e-6);
    }
}
