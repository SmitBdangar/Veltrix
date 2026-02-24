//! Raycasting and shape casting queries against the physics world.

use glam::Vec2;
use rapier2d::prelude::*;

use super::world::PhysicsWorld;

/// Contains the result of a raycast query.
#[derive(Debug, Clone, Copy)]
pub struct RaycastHit {
    /// The collider that was hit.
    pub collider: ColliderHandle,
    /// Distance along the ray at which the intersection occurred.
    pub time_of_impact: f32,
    /// The world space normal vector at the hit point.
    pub normal: Vec2,
    /// The exact world space point of intersection.
    pub point: Vec2,
}

impl PhysicsWorld {
    /// Cast a ray into the scene and return the closest hit, if any.
    pub fn raycast(
        &self,
        origin: Vec2,
        direction: Vec2,
        max_dist: f32,
        solid: bool,
    ) -> Option<RaycastHit> {
        let ray = Ray::new(
            point![origin.x, origin.y],
            vector![direction.x, direction.y],
        );
        let filter = QueryFilter::default();

        if let Some((handle, intersection)) = self.query_pipeline.cast_ray_and_get_normal(
            &self.bodies,
            &self.colliders,
            &ray,
            max_dist,
            solid,
            filter,
        ) {
            let n = intersection.normal;
            let point = ray.point_at(intersection.toi);
            Some(RaycastHit {
                collider: handle,
                time_of_impact: intersection.toi,
                normal: Vec2::new(n.x, n.y),
                point: Vec2::new(point.x, point.y),
            })
        } else {
            None
        }
    }
}
