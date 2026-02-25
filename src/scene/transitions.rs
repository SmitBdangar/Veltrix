//! Screen Transitions — fade-in / fade-out overlays between scene changes.
//!
//! # Usage
//! ```ignore
//! // Trigger a fade-out then swap scenes and fade back in:
//! transitions.start(TransitionKind::FadeOut, 0.5);
//! ```

use crate::renderer::{Color, SpriteBatcher};
use crate::renderer::sprite_batcher::VertexInput;
use crate::math::{Mat4, Vec3, Quat};

/// The kind of screen transition.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionKind {
    /// Fades the screen to black.
    FadeOut,
    /// Fades from black back to the scene.
    FadeIn,
}

/// State machine for a single in-progress transition.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionState {
    Idle,
    Running,
    Done,
}

/// Manages screen fade transitions.
///
/// Call `update(dt)` every frame and `draw(batcher)` after all scene sprites.
pub struct ScreenTransition {
    /// Current state of the transition.
    pub state: TransitionState,
    /// What kind of transition is happening.
    pub kind: TransitionKind,
    /// Total transition duration in seconds.
    pub duration: f32,
    /// Elapsed time since the transition started.
    pub elapsed: f32,
    /// Current alpha of the overlay (0.0 = transparent, 1.0 = opaque).
    pub alpha: f32,
}

impl ScreenTransition {
    pub fn new() -> Self {
        Self {
            state: TransitionState::Idle,
            kind: TransitionKind::FadeOut,
            duration: 1.0,
            elapsed: 0.0,
            alpha: 0.0,
        }
    }

    /// Begin a new transition. Resets elapsed time.
    pub fn start(&mut self, kind: TransitionKind, duration: f32) {
        self.kind = kind;
        self.duration = duration.max(0.001);
        self.elapsed = 0.0;
        self.state = TransitionState::Running;
        
        // Set initial alpha based on kind
        self.alpha = match self.kind {
            TransitionKind::FadeOut => 0.0,
            TransitionKind::FadeIn  => 1.0,
        };
    }

    /// Call every frame with the delta time. Returns `true` when the transition finishes.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.state != TransitionState::Running {
            return false;
        }

        self.elapsed += dt;
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);

        self.alpha = match self.kind {
            TransitionKind::FadeOut => t,        // 0 → 1 (transparent → opaque)
            TransitionKind::FadeIn  => 1.0 - t, // 1 → 0 (opaque → transparent)
        };

        if self.elapsed >= self.duration {
            self.state = TransitionState::Done;
            return true;
        }

        false
    }

    /// Returns `true` if the transition has completed.
    pub fn is_done(&self) -> bool {
        self.state == TransitionState::Done
    }

    /// Returns `true` if no transition is currently active.
    pub fn is_idle(&self) -> bool {
        self.state == TransitionState::Idle
    }

    /// Draw the black fullscreen overlay quad. Call this after all scene draw calls 
    /// and before `SpriteBatcher::flush`.
    ///
    /// The `screen_width` and `screen_height` should be the current backbuffer dimensions.
    pub fn draw(&self, batcher: &mut SpriteBatcher, screen_width: f32, screen_height: f32) {
        if self.alpha <= 0.0 {
            return;
        }

        let half_w = screen_width * 0.5;
        let half_h = screen_height * 0.5;

        // Full-screen quad centred at origin, scaled to cover the entire viewport.
        let mat = Mat4::from_scale_rotation_translation(
            Vec3::new(screen_width, screen_height, 1.0),
            Quat::IDENTITY,
            Vec3::new(half_w, half_h, 0.0),
        );

        let col0 = mat.col(0).into();
        let col1 = mat.col(1).into();
        let col2 = mat.col(2).into();
        let col3 = mat.col(3).into();

        let c = [0.0_f32, 0.0, 0.0, self.alpha];

        // Full-screen quad as 2 triangles
        let positions: [[f32; 2]; 6] = [
            [-0.5, -0.5],
            [ 0.5, -0.5],
            [-0.5,  0.5],
            [ 0.5, -0.5],
            [ 0.5,  0.5],
            [-0.5,  0.5],
        ];

        let mut verts = [VertexInput {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
            color: c,
            transform_col0: col0,
            transform_col1: col1,
            transform_col2: col2,
            transform_col3: col3,
        }; 6];

        for (i, pos) in positions.iter().enumerate() {
            verts[i].position = *pos;
        }

        // 100.0 z-index — render on top of everything including UI text
        batcher.push_quad(verts, 100.0);
    }
}

impl Default for ScreenTransition {
    fn default() -> Self {
        Self::new()
    }
}
