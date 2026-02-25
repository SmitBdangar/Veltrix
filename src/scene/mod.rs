//! Scene graph: Scene, SceneManager, and built-in ECS components.

pub mod components;
pub mod loader;
pub mod manager;
pub mod scene;
pub mod transitions;

pub use manager::SceneManager;
pub use scene::Scene;
pub use transitions::ScreenTransition;
pub use components::*;
