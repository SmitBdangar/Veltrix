//! Physics integration using rapier2d: world simulation, ECS sync, collision events, raycasting.

pub mod collision;
pub mod raycast;
pub mod sync;
pub mod world;

pub use world::PhysicsWorld;
pub use sync::sync_physics_to_transforms;
pub use collision::CollisionEvent;
pub use raycast::RaycastHit;
