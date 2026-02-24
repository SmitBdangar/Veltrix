//! Example 5: Full Game Assembly
//! Demonstrates the integration of ECS, physics, audio, animation, windowing,
//! and raw rendering setup all in one file.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;

fn main() -> Result<()> {
    // 1. Configure the engine
    let config = Config {
        title: "Veltrix - Full Game Demo".to_string(),
        width: 1280,
        height: 720,
        vsync: true,
        ..Default::default()
    };

    // 2. Build the main engine wrapper
    let engine = EngineBuilder::new()
        .with_config(config)
        .build()?;

    let mut player_entity = Entity::null();

    // 3. Register global subsystems and execute
    engine.run(
        |world, resources| {
            // Setup an active camera
            let cam = world.spawn();
            world.insert(cam, Camera2D::new(Vec2::new(1280.0, 720.0)));
            
            // Add some trauma shake to demo the shake system
            let mut shake = CameraShake::new();
            shake.add_trauma(0.5);
            world.insert(cam, shake);

            resources.insert(cam);

            // Setup a character entity with animations, physics, and input mapping
            player_entity = world.spawn();
            world.insert(player_entity, Name("Hero".to_string()));
            
            // Transform & Sprite
            world.insert(player_entity, Transform2D {
                position: Vec2::new(0.0, 100.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            });

            // Physics RigidBody
            world.insert(player_entity, RigidBody2D {
                handle_id: 42,
            });
            world.insert(player_entity, Collider2D {
                shape: ColliderShape::Box(Vec2::new(16.0, 32.0)),
                is_trigger: false,
                bounciness: 0.0,
                friction: 1.0,
                handle_id: 43,
            });

            // Animation Controller
            let mut anim_ctrl = AnimationController::new();
            anim_ctrl.add_clip(AnimationClip {
                name: "idle".to_string(),
                frames: vec![Rect::new(Vec2::new(0.0, 0.0), Vec2::new(32.0, 64.0))],
                frame_rate: 6.0,
                looping: true,
            });
            world.insert(player_entity, anim_ctrl);

            // Register global physics
            let physics = PhysicsWorld::new(Vec2::new(0.0, -9.81));
            resources.insert(physics);
            
            // Register AudioManager
            let audio_manager = AudioManager::new();
            // Start playing background music loop
            // let bgm_handle = assets.load("bgm.ogg").unwrap();
            // audio_manager.play_looped(&assets, &bgm_handle, AudioBusName::Music, 0.5);
            resources.insert(audio_manager);

            log::info!("Full Game initialized.");
        },
        |world, resources, dt| {
            // Variable update Loop (Input -> logic -> camera follows -> animation updates)

            // Input mapping example:
            if let Some(input) = resources.get::<InputManager>() {
                if input.keyboard.just_pressed(KeyCode::Space) {
                    // Start screen shake
                    let cam_ent = *resources.get::<Entity>().unwrap();
                    if let Some(shake) = world.get_mut::<CameraShake>(cam_ent) {
                        shake.add_trauma(0.8);
                    }
                }
            }

            // Update Animations
            let dt_f32 = dt as f32;
            let mut query = QueryMut::<AnimationController>::new(world);
            // In a real framework, we'd join QueryMut with `AnimatedSprite`
            for (_e, _ctrl) in query.iter_mut() {
                // AnimationController::update_sprite(...) 
            }

            // Update Camera Shake
            let cam_ent = *resources.get::<Entity>().unwrap();
            if let Some(shake) = world.get_mut::<CameraShake>(cam_ent) {
                let (offset, _rot) = shake.update(dt_f32);
                if let Some(cam_transform) = world.get_mut::<Transform2D>(cam_ent) {
                    cam_transform.position += offset;
                }
            }
            true
        },
        |world, resources, fixed_dt| {
            // Fixed update Loop (Physics stepping & syncing)
            if let Some(physics) = resources.get_mut::<PhysicsWorld>() {
                physics.step(fixed_dt as f32);
                sync_physics_to_transforms(world, &physics);
                
                // Raycasting example
                if let Some(_hit) = physics.raycast(Vec2::ZERO, Vec2::new(0.0, -1.0), 100.0, true) {
                    // Hit floor
                }
            }
        },
        |_world, _resources, _alpha| {
            // Render Loop
            // Typically:
            // 1. Begin wgpu render pass
            // 2. Fetch Camera2D projection
            // 3. TilemapRenderer::render(...)
            // 4. SpriteBatcher::draw_sprite(...) for all models
            // 5. Draw Debug Overlay (FPS, sysinfo) via `ui::debug_overlay::draw_debug_overlay`
        },
    )?;

    Ok(())
}
