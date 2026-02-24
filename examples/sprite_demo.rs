//! Example 2: Sprite Rendering & Input
//! Shows how to spawn an entity with a Sprite and Transform2D,
//! and move it using Keyboard Input.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;

fn main() -> Result<()> {
    let mut engine = EngineBuilder::new()
        .with_config(Config {
            title: "Veltrix - Sprite Demo".to_string(),
            ..Default::default()
        })
        .build()?;

    // Global Entity ID to modify in the update loop
    let mut player_entity = Entity::null();

    engine.run(
        |world, resources| {
            // Setup camera
            let cam = world.spawn();
            world.insert(cam, Camera2D::new(Vec2::new(800.0, 600.0)));
            resources.insert(cam); // Store active camera ID in resources

            // Setup player sprite
            player_entity = world.spawn();
            world.insert(
                player_entity,
                Transform2D {
                    position: Vec2::ZERO,
                    rotation: 0.0,
                    scale: Vec2::ONE,
                },
            );
            
            // In a real example, we'd load a texture via AssetServer here:
            // let handle = assets.load("player.png");
            // world.insert(player_entity, Sprite::new(handle));
        },
        |world, resources, dt| {
            // Update loop: Read input and move player
            let input = resources.get::<InputManager>().unwrap();
            let mut move_dir = Vec2::ZERO;

            if input.keyboard().is_pressed(KeyCode::KeyW) {
                move_dir.y += 1.0;
            }
            if input.keyboard().is_pressed(KeyCode::KeyS) {
                move_dir.y -= 1.0;
            }
            if input.keyboard().is_pressed(KeyCode::KeyA) {
                move_dir.x -= 1.0;
            }
            if input.keyboard().is_pressed(KeyCode::KeyD) {
                move_dir.x += 1.0;
            }

            if move_dir.length_squared() > 0.0 {
                move_dir = move_dir.normalize();
                if let Some(mut transform) = world.get_mut::<Transform2D>(player_entity) {
                    let speed = 200.0;
                    transform.position += move_dir * speed * dt as f32;
                }
            }
        },
        |_world, _resources, _fixed_dt| {
            // Fixed update (Physics)
        },
        |_world, _resources, _alpha| {
            // Render - In a real engine, the internal RenderDevice processes
            // all Sprite and Transform components here.
        },
    )?;

    Ok(())
}
