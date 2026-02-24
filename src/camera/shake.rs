//! Trauma-based procedural camera shake.

use glam::Vec2;

/// Procedural trauma-based 2D camera shake.
#[derive(Debug, Clone)]
pub struct CameraShake {
    /// Current trauma level `[0.0, 1.0]`. Decay over time.
    pub trauma: f32,
    /// Maximum positional displacement in pixels.
    pub max_offset: f32,
    /// Maximum rotational displacement in radians.
    pub max_rotation: f32,
    /// How fast trauma decays per second.
    pub trauma_decay: f32,
    /// Perlin noise time accumulator.
    time: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            max_offset: 20.0,
            max_rotation: 0.1,
            trauma_decay: 0.8,
            time: 0.0,
        }
    }
}

impl CameraShake {
    /// Create a new shake generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add trauma (clamped to 1.0).
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }

    /// Update the shake state and return the current `(offset_x, offset_y, rotation)` displacement.
    pub fn update(&mut self, dt: f32) -> (Vec2, f32) {
        if self.trauma <= 0.0 {
            return (Vec2::ZERO, 0.0);
        }

        self.time += dt * 10.0;
        self.trauma = (self.trauma - self.trauma_decay * dt).max(0.0);
        
        // Shake magnitude scales by trauma squared for impact.
        let shake = self.trauma * self.trauma;

        // In a real engine, we'd sample 1D Perlin noise here.
        // For scaffolding, we use a simple pseudo-random sine wave mix.
        let noise_x = (self.time).sin() * (self.time * 2.3).cos();
        let noise_y = (self.time * 1.5).cos() * (self.time * 3.1).sin();
        let noise_rot = (self.time * 0.8).sin();

        let offset = Vec2::new(noise_x * self.max_offset * shake, noise_y * self.max_offset * shake);
        let rot = noise_rot * self.max_rotation * shake;

        (offset, rot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trauma_decays_properly() {
        let mut shake = CameraShake::new();
        shake.add_trauma(1.0);
        shake.update(1.0);
        assert!(shake.trauma <= 0.201); // 1.0 - 0.8 * 1.0
    }
}
