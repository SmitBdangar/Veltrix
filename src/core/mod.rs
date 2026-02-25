//! Core engine systems: Engine, GameLoop, Time, EventSystem, Config.

pub mod config;
pub mod engine;
pub mod event;
pub mod game_loop;
pub mod time;
pub mod error_screen;

pub use config::Config;
pub use engine::{Engine, EngineBuilder};
pub use event::{EventBus, EventHandler};
pub use time::Time;
