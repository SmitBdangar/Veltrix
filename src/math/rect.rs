//! Axis-aligned bounding box (AABB) rectangle.

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// A 2D axis-aligned bounding box defined by a minimum corner and size.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Top-left corner (minimum x, minimum y).
    pub min: Vec2,
    /// Bottom-right corner (maximum x, maximum y).
    pub max: Vec2,
}

impl Rect {
    /// Create from a `min` corner and a `max` corner.
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Create from a center position and half-extents.
    pub fn from_center_half_extents(center: Vec2, half: Vec2) -> Self {
        Self {
            min: center - half,
            max: center + half,
        }
    }

    /// Create from position and size (width, height).
    pub fn from_position_size(position: Vec2, size: Vec2) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }

    /// Width of the rectangle.
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Height of the rectangle.
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Size as `(width, height)`.
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    /// Center point.
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Half-extents `(halfwidth, halfheight)`.
    pub fn half_extents(&self) -> Vec2 {
        self.size() * 0.5
    }

    /// Returns `true` if this rect overlaps `other` (inclusive edges).
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Returns `true` if `point` is inside this rect (inclusive).
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// Returns `true` if `other` is entirely inside this rect.
    pub fn contains_rect(&self, other: &Self) -> bool {
        other.min.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.max.y <= self.max.y
    }

    /// Smallest rect that contains both `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Intersection of `self` and `other`; returns `None` if they don't overlap.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min.x <= max.x && min.y <= max.y {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// Expand the rect by `amount` on all four sides.
    pub fn expanded(&self, amount: f32) -> Self {
        Self {
            min: self.min - Vec2::splat(amount),
            max: self.max + Vec2::splat(amount),
        }
    }

    /// Area of the rectangle.
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_overlapping_rects() {
        let a = Rect::from_position_size(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let b = Rect::from_position_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert!(a.intersects(&b));
    }

    #[test]
    fn no_intersection_for_separate_rects() {
        let a = Rect::from_position_size(Vec2::ZERO, Vec2::new(5.0, 5.0));
        let b = Rect::from_position_size(Vec2::new(10.0, 10.0), Vec2::new(5.0, 5.0));
        assert!(!a.intersects(&b));
    }

    #[test]
    fn union_contains_both() {
        let a = Rect::from_position_size(Vec2::ZERO, Vec2::new(5.0, 5.0));
        let b = Rect::from_position_size(Vec2::new(3.0, 3.0), Vec2::new(5.0, 5.0));
        let u = a.union(&b);
        assert!(u.contains_rect(&a));
        assert!(u.contains_rect(&b));
    }

    #[test]
    fn area_is_correct() {
        let r = Rect::from_position_size(Vec2::ZERO, Vec2::new(4.0, 5.0));
        assert!((r.area() - 20.0).abs() < 1e-6);
    }
}
