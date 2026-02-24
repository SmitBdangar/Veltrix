//! Animation: clips, controllers, tweening, and easing functions.

pub mod clip;
pub mod controller;
pub mod tween;

pub use clip::AnimationClip;
pub use controller::AnimationController;
pub use tween::{Easing, Tween};
