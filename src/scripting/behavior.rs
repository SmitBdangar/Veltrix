//! Trait for attaching script-like logic directly to entities.

use crate::ecs::world::Entity;
use crate::scene::scene::Scene;

/// A trait for script-like behavior that can be attached to entities.
///
/// In Veltrix, behaviors are usually implemented as standard ECS components
/// and systems. This trait is provided for users who prefer an OOP-like
/// `MonoBehaviour` style approach for specific one-off logic.
pub trait Behavior: Send + Sync {
    /// Called when the behavior is first added to the entity.
    fn start(&mut self, entity: Entity, scene: &mut Scene) {
        let _ = (entity, scene);
    }

    /// Called every variable timestep frame.
    fn update(&mut self, entity: Entity, scene: &mut Scene, dt: f32) {
        let _ = (entity, scene, dt);
    }

    /// Called every fixed physics timestep frame.
    fn fixed_update(&mut self, entity: Entity, scene: &mut Scene, fixed_dt: f32) {
        let _ = (entity, scene, fixed_dt);
    }
}
