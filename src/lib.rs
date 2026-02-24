//! # Veltrix — 2D Game Engine
//!
//! A modular, production-ready 2D game engine built entirely in Rust.
//!
//! ## Prelude
//! Import `veltrix::prelude::*` to bring all commonly-used types into scope.
//!
//! ## Architecture
//! | Module | Purpose |
//! |---|---|
//! | [`core`] | Engine entry point, game loop, events, time, config |
//! | [`window`] | Window creation and management (winit) |
//! | [`math`] | Math types — Transform2D, Rect, Circle, utilities |
//! | [`renderer`] | wgpu-based sprite/shape rendering pipeline |
//! | [`input`] | Keyboard, mouse, gamepad, action mapping |
//! | [`ecs`] | Entity Component System — World, Query, Systems |
//! | [`assets`] | Asset server, typed handles, hot-reload |
//! | [`scene`] | Scene, SceneManager, built-in components |
//! | [`physics`] | rapier2d integration, collision events, raycasting |
//! | [`audio`] | rodio audio manager, spatial audio, buses |
//! | [`animation`] | Clips, controllers, tweening with easing |
//! | [`camera`] | Orthographic camera, shake, smooth follow |
//! | [`tilemap`] | Tilemaps, tilesets, Tiled .tmx loader |
//! | [`ui`] | egui-based canvas, widgets, debug overlay |
//! | [`scripting`] | Behavior trait, coroutines, state machines |
//! | [`debug`] | Debug renderer, profiler, inspector, console |
//! | [`serialization`] | Save/load game state, scene serializer |
//! | [`platform`] | FileSystem, clipboard, system info |

#![warn(missing_docs)]

pub mod animation;
pub mod assets;
pub mod audio;
pub mod camera;
pub mod core;
pub mod debug;
pub mod ecs;
pub mod input;
pub mod math;
pub mod physics;
pub mod platform;
pub mod renderer;
pub mod scene;
pub mod scripting;
pub mod serialization;
pub mod tilemap;
pub mod ui;
pub mod window;

/// Engine prelude — import everything you need with `use veltrix::prelude::*`.
pub mod prelude {
    pub use crate::animation::*;
    pub use crate::assets::*;
    pub use crate::audio::*;
    pub use crate::camera::*;
    pub use crate::core::*;
    pub use crate::debug::*;
    pub use crate::ecs::*;
    pub use crate::input::*;
    pub use crate::math::*;
    pub use crate::physics::*;
    pub use crate::platform::*;
    pub use crate::renderer::*;
    pub use crate::scene::*;
    pub use crate::scripting::*;
    pub use crate::serialization::*;
    pub use crate::tilemap::*;
    pub use crate::ui::*;
    pub use crate::window::*;

    // Log & Utility crates
    pub use log::{debug, error, info, trace, warn};
    pub use anyhow::{anyhow, bail, Context, Result};
}
