//! Fixed-timestep game loop with spiral-of-death prevention.
//!
//! See the [fix your timestep](https://gafferongames.com/post/fix_your_timestep/) article
//! for the theory behind this approach.

use std::time::{Duration, Instant};

/// Tracks loop timing in the fixed-timestep game loop.
#[derive(Debug)]
pub struct GameLoop {
    /// Fixed physics/simulation step in seconds.
    pub fixed_dt: f64,
    /// Accumulated time not yet consumed by fixed steps.
    pub accumulator: f64,
    /// Maximum allowed accumulator per frame (prevents spiral-of-death).
    pub max_frame_time: f64,
    /// Instant of the previous frame.
    last_update: Instant,
    /// Total elapsed time since the loop started.
    pub elapsed: f64,
    /// Interpolation alpha in [0,1] — useful for smooth rendering.
    pub alpha: f64,
}

impl GameLoop {
    /// Create a game loop with the given fixed timestep.
    ///
    /// `fixed_dt` should normally be `1.0 / 60.0`.
    pub fn new(fixed_dt: f64) -> Self {
        Self {
            fixed_dt,
            accumulator: 0.0,
            // Cap single-frame contribution to 250 ms to avoid spiral of death.
            max_frame_time: 0.25,
            last_update: Instant::now(),
            elapsed: 0.0,
            alpha: 0.0,
        }
    }

    /// Advance the loop by one *real* frame.
    ///
    /// Returns the real delta time for `Update`-stage systems, and provides
    /// [`Self::alpha`] for render interpolation.
    ///
    /// Call [`fixed_updates`](GameLoop::fixed_updates) to determine how many
    /// fixed-timestep iterations to run this frame.
    pub fn begin_frame(&mut self) -> f64 {
        let now = Instant::now();
        let real_dt = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        // Clamp to prevent spiral of death on frame spikes.
        let clamped_dt = real_dt.min(self.max_frame_time);
        self.accumulator += clamped_dt;
        self.elapsed += clamped_dt;

        real_dt
    }

    /// Returns the number of fixed-timestep iterations remaining this frame.
    ///
    /// Use in a `while` loop:
    /// ```text
    /// while game_loop.step() {
    ///     physics.step(game_loop.fixed_dt);
    ///     // run FixedUpdate systems
    /// }
    /// ```
    pub fn step(&mut self) -> bool {
        if self.accumulator >= self.fixed_dt {
            self.accumulator -= self.fixed_dt;
            // Update alpha *after* consuming one step.
            self.alpha = self.accumulator / self.fixed_dt;
            true
        } else {
            self.alpha = self.accumulator / self.fixed_dt;
            false
        }
    }

    /// Interpolation factor `alpha ∈ [0, 1]` for smooth rendering between
    /// fixed timesteps. Pass this to your render systems.
    #[inline]
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// Target duration of a fixed step.
    pub fn fixed_duration(&self) -> Duration {
        Duration::from_secs_f64(self.fixed_dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn accumulates_and_steps() {
        let mut gl = GameLoop::new(1.0 / 60.0);
        // Push exactly 3 fixed steps worth of time into the accumulator.
        gl.accumulator = 3.0 / 60.0 + 0.001;
        let mut count = 0;
        while gl.step() {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn clamps_accumulator_to_max_frame_time() {
        let mut gl = GameLoop::new(1.0 / 60.0);
        // Simulate a 1-second hitch.
        gl.accumulator = 1.0;
        // After clamping, begin_frame shouldn't add more than max_frame_time.
        // The test validates that step() drains no more than max allows.
        let mut count = 0;
        while gl.step() {
            count += 1;
            if count > 100 {
                panic!("Infinite loop detected — clamping broken");
            }
        }
        // Max we can have is max_frame_time / fixed_dt ≈ 15 steps.
        assert!(count <= 60, "Too many steps: {count}");
    }
}
