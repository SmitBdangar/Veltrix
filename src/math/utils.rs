//! Math utility functions: lerp, clamp, remap, angle↔vector, random range.

use glam::Vec2;

// ── Scalar helpers ────────────────────────────────────────────────────────────

/// Linear interpolation between `a` and `b` by factor `t`.
///
/// `t = 0` returns `a`; `t = 1` returns `b`. `t` is unclamped.
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp `value` to `[min, max]`.
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

/// Remap `value` from range `[in_min, in_max]` to `[out_min, out_max]`.
///
/// Does not clamp the output.
#[inline]
pub fn remap(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let t = (value - in_min) / (in_max - in_min);
    lerp(out_min, out_max, t)
}

/// Smooth-step interpolation (Hermite) between `0` and `1`.
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Wrap `value` into `[min, max)`.
pub fn wrap(value: f32, min: f32, max: f32) -> f32 {
    let range = max - min;
    if range == 0.0 {
        return min;
    }
    min + ((value - min) % range + range) % range
}

// ── Angle / vector conversion ─────────────────────────────────────────────────

/// Convert an angle in radians to a normalised direction vector.
///
/// `0` radians → `(1, 0)` (right).
#[inline]
pub fn angle_to_vec(angle_rad: f32) -> Vec2 {
    Vec2::new(angle_rad.cos(), angle_rad.sin())
}

/// Convert a direction vector to an angle in radians.
///
/// Returns a value in `[-π, π]`.
#[inline]
pub fn vec_to_angle(v: Vec2) -> f32 {
    v.y.atan2(v.x)
}

/// Shortest angular difference from `a` to `b`, in radians.
pub fn angle_diff(a: f32, b: f32) -> f32 {
    let diff = (b - a).rem_euclid(std::f32::consts::TAU);
    if diff > std::f32::consts::PI {
        diff - std::f32::consts::TAU
    } else {
        diff
    }
}

// ── Random helpers (thin wrapper around `rand`) ───────────────────────────────

/// Return a random `f32` in `[min, max)`.
pub fn random_range(min: f32, max: f32) -> f32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..max)
}

/// Return a random `i32` in `[min, max]` (inclusive).
pub fn random_range_i(min: i32, max: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..=max)
}

/// Return a random unit vector (uniformly distributed on the unit circle).
pub fn random_direction() -> Vec2 {
    let angle = random_range(0.0, std::f32::consts::TAU);
    angle_to_vec(angle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn lerp_endpoints() {
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn remap_basic() {
        let v = remap(5.0, 0.0, 10.0, 0.0, 100.0);
        assert!((v - 50.0).abs() < 1e-4);
    }

    #[test]
    fn angle_vec_round_trip() {
        for &angle in &[0.0, FRAC_PI_2, PI, -FRAC_PI_2] {
            let v = angle_to_vec(angle);
            let back = vec_to_angle(v);
            assert!((back - angle).abs() < 1e-5, "angle={angle} back={back}");
        }
    }

    #[test]
    fn wrap_in_range() {
        assert!((wrap(11.0, 0.0, 10.0) - 1.0).abs() < 1e-6);
        assert!((wrap(-1.0, 0.0, 10.0) - 9.0).abs() < 1e-6);
    }
}
