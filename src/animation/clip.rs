//! Definition of animation sequences from a texture atlas.

use crate::math::Rect;
use serde::{Deserialize, Serialize};

/// Defines a sequence of texture atlas frames with playback metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    /// The name of this sequence (e.g., "run", "jump").
    pub name: String,
    /// The rectangular frames in the texture atlas.
    pub frames: Vec<Rect>,
    /// Frame rate (frames per second). Default: 12.0
    pub frame_rate: f32,
    /// Whether the animation should restart when finished.
    pub looping: bool,
}

impl AnimationClip {
    /// Calculate total duration in seconds.
    pub fn duration(&self) -> f32 {
        self.frames.len() as f32 / self.frame_rate
    }
}
