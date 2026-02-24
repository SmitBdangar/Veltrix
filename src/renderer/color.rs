//! RGBA color type with common color presets and conversions.

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// A 32-bit floating-point RGBA color.
///
/// Components are in linear color space, in the range `[0.0, 1.0]`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Color {
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel (`1.0` = fully opaque).
    pub a: f32,
}

impl Color {
    /// Create a color from red, green, blue, and alpha components.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque color from red, green, blue components.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Create a color from 8-bit integer components (`0..=255`).
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create a color from a hex string, e.g. `"#FF8800"` or `"FF8800FF"`.
    ///
    /// Supports `#RRGGBB`, `#RRGGBBAA`, `RRGGBB`, and `RRGGBBAA`.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let n = i64::from_str_radix(hex, 16).ok()?;
        match hex.len() {
            6 => Some(Self::from_rgba8(
                ((n >> 16) & 0xFF) as u8,
                ((n >> 8) & 0xFF) as u8,
                (n & 0xFF) as u8,
                255,
            )),
            8 => Some(Self::from_rgba8(
                ((n >> 24) & 0xFF) as u8,
                ((n >> 16) & 0xFF) as u8,
                ((n >> 8) & 0xFF) as u8,
                (n & 0xFF) as u8,
            )),
            _ => None,
        }
    }

    /// Return the color as a `[f32; 4]` array (for GPU uploads).
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Return the color as a `wgpu::Color` for render pass clear values.
    pub fn to_wgpu(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }

    /// Linearly interpolate toward `other` by `t ∈ [0, 1]`.
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Return a version of this color with the given alpha.
    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    // ── Colour presets ─────────────────────────────────────────────────────
    pub const WHITE:       Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK:       Self = Self::rgb(0.0, 0.0, 0.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const RED:         Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN:       Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE:        Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW:      Self = Self::rgb(1.0, 1.0, 0.0);
    pub const CYAN:        Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA:     Self = Self::rgb(1.0, 0.0, 1.0);
    pub const ORANGE:      Self = Self::rgb(1.0, 0.647, 0.0);
    pub const PURPLE:      Self = Self::rgb(0.502, 0.0, 0.502);
    pub const GRAY:        Self = Self::rgb(0.5, 0.5, 0.5);
    pub const DARK_GRAY:   Self = Self::rgb(0.25, 0.25, 0.25);
    pub const LIGHT_GRAY:  Self = Self::rgb(0.75, 0.75, 0.75);
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_parses_correctly() {
        let c = Color::from_hex("#FF8000").unwrap();
        assert!((c.r - 1.0).abs() < 0.005);
        assert!((c.g - 0.502).abs() < 0.005);
        assert!((c.b - 0.0).abs() < 0.005);
    }

    #[test]
    fn lerp_midpoint() {
        let mid = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
    }
}
