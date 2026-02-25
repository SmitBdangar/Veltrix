//! Time tracking: delta time, elapsed time, FPS, and fixed timestep.

use std::time::{Duration, Instant};

/// Tracks frame timing information.
///
/// Added to [`Resources`](crate::ecs::resources::Resources) so any system can
/// read `Time` without borrowing the `Engine` directly.
#[derive(Debug)]
pub struct Time {
    /// Wall-clock instant at engine start.
    start: Instant,
    /// Instant of the previous frame's end.
    last_frame: Instant,
    /// Seconds elapsed since the last frame.
    delta: f64,
    /// Total seconds elapsed since engine start.
    elapsed: f64,
    /// Current smoothed frames-per-second estimate.
    fps: f64,
    /// Number of frames rendered.
    frame_count: u64,
    /// Smoothing window for FPS (last N deltas).
    fps_samples: [f64; 16],
    /// Ring-buffer index for fps_samples.
    fps_index: usize,
    /// Smoothed delta time to prevent physics jitter.
    smooth_delta: f64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    /// Create a new `Time` instance starting from *now*.
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start: now,
            last_frame: now,
            delta: 0.0,
            elapsed: 0.0,
            fps: 0.0,
            frame_count: 0,
            fps_samples: [0.0; 16],
            fps_index: 0,
            smooth_delta: 0.0,
        }
    }

    /// Called once per frame to advance internal clocks.
    ///
    /// Returns the raw delta time (seconds since last call).
    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let raw_dt = now.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = now;
        self.delta = raw_dt;
        self.elapsed = now.duration_since(self.start).as_secs_f64();
        self.frame_count += 1;

        // Rolling FPS average.
        self.fps_samples[self.fps_index] = raw_dt;
        self.fps_index = (self.fps_index + 1) % self.fps_samples.len();
        
        let sum: f64 = self.fps_samples.iter().sum();
        let valid_samples = self.frame_count.min(self.fps_samples.len() as u64) as f64;
        
        self.fps = if sum > 0.0 { valid_samples / sum } else { 0.0 };
        self.smooth_delta = if valid_samples > 0.0 { sum / valid_samples } else { raw_dt };

        raw_dt
    }

    /// Smoothed seconds elapsed since the last frame. Useful for preventing stutter.
    #[inline]
    pub fn delta_seconds(&self) -> f64 {
        self.smooth_delta
    }

    /// Raw un-smoothed seconds elapsed since the last frame.
    #[inline]
    pub fn delta_seconds_raw(&self) -> f64 {
        self.delta
    }

    /// Smoothed seconds elapsed since the last frame as `f32`.
    #[inline]
    pub fn delta_f32(&self) -> f32 {
        self.smooth_delta as f32
    }

    /// Total seconds elapsed since the engine started.
    #[inline]
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed
    }

    /// Smoothed frames per second.
    #[inline]
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Total number of frames rendered.
    #[inline]
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Sleep until `target_frame_time` has elapsed since `last_frame`.
    ///
    /// Used to enforce an FPS cap when vsync is off.
    pub fn enforce_fps_cap(&self, target_frame_time: Duration) {
        let elapsed = self.last_frame.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn tick_increments_frame_count() {
        let mut t = Time::new();
        t.tick();
        t.tick();
        t.tick();
        assert_eq!(t.frame_count(), 3);
    }

    #[test]
    fn elapsed_increases_monotonically() {
        let mut t = Time::new();
        sleep(Duration::from_millis(5));
        t.tick();
        let e1 = t.elapsed_seconds();
        sleep(Duration::from_millis(5));
        t.tick();
        let e2 = t.elapsed_seconds();
        assert!(e2 > e1, "elapsed must be monotonic: {e1} < {e2}");
    }

    #[test]
    fn delta_seconds_is_positive() {
        let mut t = Time::new();
        sleep(Duration::from_millis(10));
        t.tick();
        assert!(t.delta_seconds() > 0.0);
    }
}
