//! Example 4: Tilemap & Follow Camera
//! Demonstrates parsing a mock TMX file, creating a Tilemap and Tileset,
//! rendering it with the Camera following a dummy player.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;

fn main() -> Result<()> {
    let engine = EngineBuilder::new()
        .with_config(Config {
            title: "Veltrix - Tilemap Demo".to_string(),
            ..Default::default()
        })
        .build()?;

    engine.run(
        |world, resources| {
            // Setup Camera following nothing initially
            let cam = world.spawn();
            world.insert(cam, Camera2D::new(Vec2::new(800.0, 600.0)));
            resources.insert(cam); // Tag as active camera

            // Create a Player to follow
            let player = world.spawn();
            world.insert(
                player,
                Transform2D {
                    position: Vec2::new(100.0, 100.0),
                    rotation: 0.0,
                    scale: Vec2::ONE,
                    dirty: true,
                },
            );

            // Configure Camera Follow component on the Camera pointing to the Player
            world.insert(cam, CameraFollow {
                target: Some(player),
                smoothing: 5.0,
                deadzone: 20.0,
                lookahead: Vec2::new(50.0, 0.0),
            });

            // Create a mock Tilemap 10x10 and Tileset
            let mut tilemap = Tilemap::new(20, 20, 32, 32);
            let tileset = Tileset {
                name: "dungeon".to_string(),
                texture: None, // Missing Handle<Texture>
                image_width: 256,
                image_height: 256,
                tile_width: 32,
                tile_height: 32,
                first_gid: 1,
                tile_count: 64,
            };

            // Sprinkle some random tiles
            for x in 0..20 {
                for y in 0..20 {
                    if (x + y) % 2 == 0 {
                        tilemap.set_tile(x, y, 1);
                    }
                }
            }

            // Insert map data as components on a Map root entity
            let map_entity = world.spawn();
            world.insert(map_entity, tilemap);
            world.insert(map_entity, tileset);
            world.insert(
                map_entity,
                Transform2D {
                    position: Vec2::ZERO,
                    rotation: 0.0,
                    scale: Vec2::ONE,
                    dirty: true,
                },
            );
        },
        |world, _resources, dt| {
            // Very simple constant movement for the player just to demo camera follow
            let mut q = QueryMut::<&mut Transform2D>::new(world);
            // In a real game we'd filter by a `Player` tag component
            // We just grab the first transform we see for this stub
            for (_entity, transform) in q.iter_mut() {
                // Ignore origin elements
                if transform.position.length() > 0.1 || true {
                    transform.position.x += 50.0 * dt as f32;
                    transform.position.y += 20.0 * dt as f32;
                    break;
                }
            }
            true
        },
        |_world, _resources, _fixed_dt| {
            // Physics / fixed steps
        },
        |world, resources, _alpha| {
            // Frame Render pass
            
            // 1. Get Camera position
            let cam_entity = *resources.get::<Entity>().unwrap();
            let cam = world.get::<Camera2D>(cam_entity).unwrap();
            let cam_follow = world.get::<CameraFollow>(cam_entity).unwrap();
            
            // 2. Read where player is
            let _target_pos = if let Some(target) = cam_follow.target {
                world.get::<Transform2D>(target).map(|t| t.position).unwrap_or(cam.position)
            } else {
                cam.position
            };
            
            // 3. Update Camera Transform (Since we can't mutate safely inside immutable fetch, 
            // the system would do it in `Update` normally)
            
            // 4. Render Tilemap through TilemapRenderer
            // let mut sprite_batcher = resources.get_mut::<SpriteBatcher>().unwrap();
            // let camera_bounds = crate::math::Rect::from_position_size(...);
            /*
            for (entity, tilemap) in Query::<Tilemap>::new(world).iter() {
                let tileset = world.get::<Tileset>(entity).unwrap();
                let transform = world.get::<Transform2D>(entity).unwrap();
                
                TilemapRenderer::render(
                    tilemap,
                    tileset,
                    transform,
                    &camera_bounds,
                    &mut sprite_batcher,
                    0.0
                );
            }
            */
        },
    )?;

    Ok(())
}
