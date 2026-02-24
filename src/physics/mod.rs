//! Physics integration using rapier2d: world simulation, ECS sync, collision events, raycasting.

pub mod collision;
pub mod raycast;
pub mod sync;
pub mod world;

pub use world::PhysicsWorld;
