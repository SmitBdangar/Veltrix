//! Built-in ECS components.

use crate::assets::handle::Handle;
use crate::ecs::world::Entity;
use crate::math::Rect;
use crate::renderer::{color::Color, texture::Texture};
pub use crate::renderer::text::Text;
use serde::{Deserialize, Serialize};

/// Sets the string label of an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag(pub String);

impl Default for Tag {
    fn default() -> Self {
        Self(String::new())
    }
}

/// Sets the human-readable debug name of an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(pub String);

impl Default for Name {
    fn default() -> Self {
        Self(String::new())
    }
}

/// Relational component indicating the parent entity in a scene graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parent(pub Entity);

/// Relational component tracking child entities.
#[derive(Debug, Clone, Default)]
pub struct Children(pub Vec<Entity>);

// ── Built-in Visual Components ───────────────────────────────────────────────

/// Renders a static texture sprite.
#[derive(Debug)]
pub struct Sprite {
    /// Texture asset handle. Optional for solid colored quads.
    pub texture: Option<Handle<Texture>>,
    /// Color tint applied over the texture.
    pub color: Color,
    /// Whether to flip horizontally.
    pub flip_x: bool,
    /// Whether to flip vertically.
    pub flip_y: bool,
    /// Optional sub-rectangle of the texture to render (e.g., for an atlas).
    pub src_rect: Option<Rect>,
}

impl Sprite {
    /// Create a new sprite wrapping a texture handle.
    pub fn new(texture: Handle<Texture>) -> Self {
        Self {
            texture: Some(texture),
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            src_rect: None,
        }
    }
}

/// Renders an animated sequence of sprite frames.
#[derive(Debug)]
pub struct AnimatedSprite {
    /// Ordered frames to play (from an atlas).
    pub frames: Vec<Rect>,
    /// Speed of animation.
    pub frame_rate: f32,
    /// Whether to loop continuously.
    pub looping: bool,
    /// Currently playing frame index.
    pub current_frame: usize,
    /// Elapsed fractional frame time.
    pub timer: f32,
}

/// A single active particle in the world.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: crate::math::Vec2,
    pub velocity: crate::math::Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub start_color: Color,
    pub end_color: Color,
    pub size: crate::math::Vec2,
}

/// Component that emits particles over time.
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    /// Particles to emit per second.
    pub emission_rate: f32,
    /// Timer accumulating fractional emission.
    pub emission_timer: f32,
    /// Initial velocity for spawned particles.
    pub initial_velocity: crate::math::Vec2,
    /// Velocity random variance.
    pub velocity_variance: crate::math::Vec2,
    /// Starting color.
    pub start_color: Color,
    /// Ending color.
    pub end_color: Color,
    /// How long particles live.
    pub lifetime: f32,
    /// Current active particles. (In a real system, these would be separate ECS entities or a contiguous array).
    pub particles: Vec<Particle>,
}

impl ParticleEmitter {
    pub fn new(rate: f32, lifetime: f32) -> Self {
        Self {
            emission_rate: rate,
            emission_timer: 0.0,
            initial_velocity: crate::math::Vec2::new(0.0, 50.0),
            velocity_variance: crate::math::Vec2::new(20.0, 20.0),
            start_color: Color::WHITE,
            end_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 },
            lifetime,
            particles: Vec::new(),
        }
    }
}

// ── Built-in Physics Components (rapier wrappers) ────────────────────────────

/// Wrapper around a Rapier 2D rigid body handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RigidBody2D {
    // We cannot construct actual handles here because rapier2d module is internal to physics
    // For scaffolding, we store it as a generic u64 ID placeholder.
    pub handle_id: u64,
}

/// Abstract representation of a 2D collider shape.
#[derive(Debug, Clone, Copy)]
pub enum ColliderShape {
    /// A circle defined by radius.
    Circle(f32),
    /// A box defined by half-extents.
    Box(crate::math::Vec2),
}

/// Wrapper around a Rapier 2D collider handle.
#[derive(Debug, Clone)]
pub struct Collider2D {
    pub shape: ColliderShape,
    pub is_trigger: bool,
    pub bounciness: f32,
    pub friction: f32,
    /// Scaffolding placeholder for actual Rapier handle ID.
    pub handle_id: u64,
}

// ── Built-in Audio Components ────────────────────────────────────────────────

/// Renders spatial or non-spatial audio.
#[derive(Debug)]
pub struct AudioSource {
    pub clip: Handle<crate::audio::clip::AudioClip>,
    pub volume: f32,
    pub pitch: f32,
    pub looping: bool,
    pub auto_play: bool,
    pub is_playing: bool,
}

impl AudioSource {
    pub fn new(clip: Handle<crate::audio::clip::AudioClip>) -> Self {
        Self {
            clip,
            volume: 1.0,
            pitch: 1.0,
            looping: false,
            auto_play: false,
            is_playing: false,
        }
    }
}
