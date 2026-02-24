//! State machine for switching between animations.

use std::collections::HashMap;
use super::clip::AnimationClip;
use crate::scene::components::AnimatedSprite;

/// A simple animation state machine controller.
///
/// Attached to an entity alongside `AnimatedSprite`.
#[derive(Debug, Default)]
pub struct AnimationController {
    /// Available animation clips by name.
    clips: HashMap<String, AnimationClip>,
    /// The currently playing clip name.
    current_clip: Option<String>,
}

impl AnimationController {
    /// Create a new controller.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a clip to the controller.
    pub fn add_clip(&mut self, clip: AnimationClip) {
        self.clips.insert(clip.name.clone(), clip);
    }

    /// Play the named clip. If it is already playing, this does nothing.
    ///
    /// Requires mutable access to the entity's `AnimatedSprite` to update the actual run state.
    pub fn play(&mut self, name: &str, sprite: &mut AnimatedSprite) {
        if self.current_clip.as_deref() == Some(name) {
            return; // Already playing
        }

        if let Some(clip) = self.clips.get(name) {
            self.current_clip = Some(name.to_string());
            
            // Reconfigure the sprite
            sprite.frames = clip.frames.clone();
            sprite.frame_rate = clip.frame_rate;
            sprite.looping = clip.looping;
            sprite.current_frame = 0;
            sprite.timer = 0.0;
        } else {
            log::warn!("Attempted to play unknown animation clip: {}", name);
        }
    }

    /// Update the currently playing animation timer (called typically in an `Update` system).
    pub fn update_sprite(sprite: &mut AnimatedSprite, dt: f32) {
        if sprite.frames.is_empty() {
            return;
        }

        sprite.timer += dt * sprite.frame_rate;

        while sprite.timer >= 1.0 {
            sprite.timer -= 1.0;
            sprite.current_frame += 1;

            if sprite.current_frame >= sprite.frames.len() {
                if sprite.looping {
                    sprite.current_frame = 0;
                } else {
                    // Clamp to the last frame if not looping
                    sprite.current_frame = sprite.frames.len() - 1;
                    // Prevent timer from building up infinitely
                    sprite.timer = 0.0;
                }
            }
        }
    }

    /// Check if the current animation has finished playing (if not looping).
    pub fn is_finished(&self, sprite: &AnimatedSprite) -> bool {
        if sprite.looping || sprite.frames.is_empty() {
            return false;
        }
        sprite.current_frame >= sprite.frames.len() - 1
    }
}
