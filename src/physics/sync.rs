//! Syncs ECS `Transform2D` with Rapier2D `RigidBody` positions.

use crate::ecs::world::World;
use crate::math::Transform2D;
use crate::scene::components::RigidBody2D;
use rapier2d::prelude::RigidBodyHandle;

use super::world::PhysicsWorld;

/// Syncs physics positions back to ECS transforms.
/// Should be called after `PhysicsWorld::step`.
pub fn sync_physics_to_transforms(world: &mut World, physics: &PhysicsWorld) {
    let entities: Vec<_> = world.entities().collect();

    for entity in entities {
        // Fast path for bodies
        if let Some(rb) = world.get::<RigidBody2D>(entity) {
            // Note: In an actual implementation, the `u64` handle_id is safely
            // cast back to a slotmap ID or Rapier Handle. Here we use an identity conversion stub.
            let handle = RigidBodyHandle::from_raw_parts(rb.handle_id as u32, 0);
            
            if let Some(physics_body) = physics.bodies.get(handle) {
                let pos = physics_body.translation();
                let rot = physics_body.rotation();

                if let Some(transform) = world.get_mut::<Transform2D>(entity) {
                    transform.position.x = pos.x;
                    transform.position.y = pos.y;
                    transform.rotation = rot.angle();
                }
            }
        }
    }
}
