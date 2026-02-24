//! Physics collision events (OnCollisionEnter, OnCollisionExit, etc).

use crate::ecs::world::Entity;

/// Type of collision event emitted by the physics engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionState {
    /// Two colliders just touched.
    Enter,
    /// Two colliders just separated.
    Exit,
}

/// Event emitted when two entities with colliders interact.
#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    /// The state of the collision.
    pub state: CollisionState,
    /// The first entity involved.
    pub entity_a: Entity,
    /// The second entity involved.
    pub entity_b: Entity,
}

/// Buffer for collision events generated during a physics timestep.
/// Usually inserted into the ECS as a `Resource`.
#[derive(Default, Debug)]
pub struct CollisionEvents {
    pub events: Vec<CollisionEvent>,
}

impl CollisionEvents {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn push(&mut self, event: CollisionEvent) {
        self.events.push(event);
    }
    
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
