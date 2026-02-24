//! Example 3: Physics Integration
//! Demonstrates spawning a floor and multiple falling dynamic boxes
//! using the ECS and Physics module.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;

fn main() -> Result<()> {
    let mut engine = EngineBuilder::new()
        .with_config(Config {
            title: "Veltrix - Physics Demo".to_string(),
            ..Default::default()
        })
        .build()?;

    engine.run(
        |world, resources| {
            // Setup camera
            let cam = world.spawn();
            world.insert(cam, Camera2D::new(Vec2::new(800.0, 600.0)));
            resources.insert(cam);

            // Access the global physics world from resources
            let mut physics = PhysicsWorld::new(Vec2::new(0.0, -9.81));

            // Create a static floor
            let floor = world.spawn();
            world.insert(
                floor,
                Transform2D {
                    position: Vec2::new(0.0, -200.0),
                    rotation: 0.0,
                    scale: Vec2::ONE,
                },
            );
            world.insert(
                floor,
                Collider2D {
                    shape: ColliderShape::Box(Vec2::new(400.0, 20.0)),
                    is_trigger: false,
                    bounciness: 0.1,
                    friction: 0.5,
                    handle_id: 1, // Scaffold handle
                },
            );
            
            // In a complete implementation, we would register the Collider2D
            // shape with the `physics` rigid body / collider sets here.

            // Spawn 100 falling boxes
            for i in 0..100 {
                let box_entity = world.spawn();
                
                // Stagger spawn positions
                let x = (i % 10) as f32 * 40.0 - 200.0;
                let y = (i / 10) as f32 * 40.0 + 100.0;

                world.insert(
                    box_entity,
                    Transform2D {
                        position: Vec2::new(x, y),
                        rotation: std::f32::consts::PI / 4.0, // 45 deg tilt
                        scale: Vec2::ONE,
                    },
                );

                world.insert(
                    box_entity,
                    RigidBody2D {
                        handle_id: (i + 2) as u64, // Scaffold handle
                    },
                );

                world.insert(
                    box_entity,
                    Collider2D {
                        shape: ColliderShape::Box(Vec2::new(10.0, 10.0)),
                        is_trigger: false,
                        bounciness: 0.4,
                        friction: 0.5,
                        handle_id: (i + 1000) as u64,
                    },
                );
            }

            resources.insert(physics);
        },
        |_world, _resources, _dt| {
            // Handle input if needed
        },
        |world, resources, fixed_dt| {
            // Fixed update runs the physics engine
            if let Some(mut physics) = resources.get_mut::<PhysicsWorld>() {
                physics.step(fixed_dt as f32);
                
                // Sync positions back to ECS
                sync_physics_to_transforms(world, &physics);
            }
        },
        |_world, _resources, _alpha| {
            // Render shapes or sprites based on Transforms
        },
    )?;

    Ok(())
}
