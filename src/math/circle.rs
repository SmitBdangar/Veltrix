//! Circle: center, radius, and geometric query helpers.

use glam::Vec2;
use serde::{Deserialize, Serialize};

use super::rect::Rect;

/// A 2D circle defined by a center point and radius.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    /// Center of the circle in world space.
    pub center: Vec2,
    /// Radius (must be > 0).
    pub radius: f32,
}

impl Circle {
    /// Create a new circle.
    pub fn new(center: Vec2, radius: f32) -> Self {
        debug_assert!(radius > 0.0, "Circle radius must be positive");
        Self { center, radius }
    }

    /// Returns `true` if `point` is inside the circle.
    pub fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance_squared(point) <= self.radius * self.radius
    }

    /// Returns `true` if this circle overlaps `other`.
    pub fn intersects_circle(&self, other: &Self) -> bool {
        let dist_sq = self.center.distance_squared(other.center);
        let sum = self.radius + other.radius;
        dist_sq <= sum * sum
    }

    /// Returns `true` if this circle overlaps an AABB `rect`.
    pub fn intersects_rect(&self, rect: &Rect) -> bool {
        // Closest point on the rect to the circle center.
        let closest = self.center.clamp(rect.min, rect.max);
        self.contains_point(closest)
    }

    /// Area of the circle.
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    /// Circumference of the circle.
    pub fn circumference(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    /// Axis-aligned bounding box that tightly encloses this circle.
    pub fn bounding_rect(&self) -> Rect {
        Rect::from_center_half_extents(self.center, Vec2::splat(self.radius))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_center_point() {
        let c = Circle::new(Vec2::ZERO, 5.0);
        assert!(c.contains_point(Vec2::ZERO));
    }

    #[test]
    fn does_not_contain_far_point() {
        let c = Circle::new(Vec2::ZERO, 5.0);
        assert!(!c.contains_point(Vec2::new(10.0, 0.0)));
    }

    #[test]
    fn overlapping_circles_intersect() {
        let a = Circle::new(Vec2::ZERO, 5.0);
        let b = Circle::new(Vec2::new(3.0, 0.0), 5.0);
        assert!(a.intersects_circle(&b));
    }

    #[test]
    fn non_overlapping_circles_do_not_intersect() {
        let a = Circle::new(Vec2::ZERO, 2.0);
        let b = Circle::new(Vec2::new(10.0, 0.0), 2.0);
        assert!(!a.intersects_circle(&b));
    }
}
