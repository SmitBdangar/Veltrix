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
    // Core
    pub use crate::core::{
        config::Config,
        engine::{Engine, EngineBuilder},
        event::{EventBus, EventHandler},
        time::Time,
    };

    // Math
    pub use crate::math::{
        circle::Circle,
        rect::Rect,
        transform2d::Transform2D,
        utils::{clamp, lerp, remap},
    };
    pub use glam::{Mat4, Quat, Vec2, Vec3, Vec4};

    // Renderer
    pub use crate::renderer::{
        color::Color,
        texture::{Texture, TextureHandle},
    };

    // Input
    pub use crate::input::{
        action_map::ActionMap,
        keyboard::{KeyCode, KeyboardState},
        mouse::MouseState,
        InputManager,
    };

    // ECS
    pub use crate::ecs::{
        commands::Commands,
        resources::Resources,
        system::{System, SystemStage},
        world::{Entity, World},
    };

    // Assets
    pub use crate::assets::{handle::Handle, server::AssetServer};

    // Scene
    pub use crate::scene::{
        components::*,
        manager::SceneManager,
        scene::Scene,
    };

    // Audio
    pub use crate::audio::{clip::AudioClip, manager::AudioManager};

    // Physics
    pub use crate::physics::world::PhysicsWorld;

    // Animation
    pub use crate::animation::{
        clip::AnimationClip,
        tween::{Easing, Tween},
    };

    // Camera
    pub use crate::camera::{
        camera2d::Camera2D,
        shake::CameraShake,
        follow::CameraFollow,
    };

    // Platform
    pub use crate::platform::filesystem::FileSystem;

    // Serialization
    pub use crate::serialization::save_game::SaveGame;

    // logging re-exports for convenience
    pub use log::{debug, error, info, trace, warn};
    pub use anyhow::{anyhow, bail, Context, Result};
}
