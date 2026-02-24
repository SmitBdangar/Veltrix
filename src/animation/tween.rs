//! Tweening of continuous values (f32, Vec2) using standard easing functions.

use glam::Vec2;
use crate::math::lerp;

/// Standard easing functions for smooth transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

impl Easing {
    /// Apply the easing function to a linear `t ∈ [0, 1]`.
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Self::Elastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    (2.0_f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }
}

/// A generic tween that animates a value of type `T` over `duration`.
#[derive(Debug, Clone)]
pub struct Tween<T> {
    pub start: T,
    pub end: T,
    pub duration: f32,
    pub elapsed: f32,
    pub easing: Easing,
}

impl<T> Tween<T> {
    pub fn new(start: T, end: T, duration: f32, easing: Easing) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
            easing,
        }
    }
    
    /// Update elapsed time.
    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
    }
    
    /// Return the interpolated value if we can lerp `T`.
    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
    
    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        }
    }
}

impl Tween<f32> {
    pub fn value(&self) -> f32 {
        let t = self.easing.apply(self.progress());
        lerp(self.start, self.end, t)
    }
}

impl Tween<Vec2> {
    pub fn value(&self) -> Vec2 {
        let t = self.easing.apply(self.progress());
        self.start.lerp(self.end, t)
    }
}
