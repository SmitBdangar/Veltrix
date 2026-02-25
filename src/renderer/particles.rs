//! Particle System

use crate::ecs::world::World;
use crate::ecs::resources::Resources;
use crate::scene::components::ParticleEmitter;
use crate::math::{Transform2D, Vec2};

pub struct ParticleSystem;

impl ParticleSystem {
    pub fn update(world: &mut World, dt: f32) {        // 1. Gather all emitters and their initial transforms
        // Gather entities with ParticleEmitter to avoid borrowing the whole world mutably simultaneously.
        // In our ECS, we fetch the entities first.
        let mut active_emitters = Vec::new();
        {
            let mut q = crate::ecs::query::QueryMut::<ParticleEmitter>::new(world);
            for (entity, _) in q.iter_mut() {
                active_emitters.push(entity);
            }
        }

        for entity in active_emitters {
            // Fetch components sequentially to avoid borrow checker conflicts
            // Since we need to push particles, we have to borrow the emitter mutably.
            if let Some(mut emitter) = world.get_mut::<ParticleEmitter>(entity) {
                // Advance emission timer
                emitter.emission_timer += dt;
                let emit_count = (emitter.emission_timer * emitter.emission_rate).floor() as u32;
                
                if emit_count > 0 {
                    emitter.emission_timer -= emit_count as f32 / emitter.emission_rate;
                    
                    for _ in 0..emit_count {
                        // Very simple pseudo-random variance
                        let rand_x = (fast_rand() - 0.5) * 2.0;
                        let rand_y = (fast_rand() - 0.5) * 2.0;
                        
                        let vel = Vec2::new(
                            emitter.initial_velocity.x + rand_x * emitter.velocity_variance.x,
                            emitter.initial_velocity.y + rand_y * emitter.velocity_variance.y,
                        );
                        
                        emitter.particles.push(crate::scene::components::Particle {
                            position: Vec2::ZERO,
                            velocity: vel,
                            lifetime: emitter.lifetime,
                            max_lifetime: emitter.lifetime,
                            start_color: emitter.start_color,
                            end_color: emitter.end_color,
                            size: Vec2::new(10.0, 10.0), // fixed size for now
                        });
                    }
                }
                
                // Update existing particles
                for i in (0..emitter.particles.len()).rev() {
                    emitter.particles[i].lifetime -= dt;
                    
                    if emitter.particles[i].lifetime <= 0.0 {
                        emitter.particles.swap_remove(i);
                    } else {
                        let vel = emitter.particles[i].velocity;
                        emitter.particles[i].position += vel * dt;
                    }
                }
            }
        }
    }

    pub fn draw(world: &mut World, batcher: &mut crate::renderer::SpriteBatcher) {
        // Collect emitters first to respect borrowing rules
        let mut active_entities = Vec::new();
        {
            let q = crate::ecs::query::Query::<ParticleEmitter>::new(world);
            for (entity, _) in q.iter() {
                active_entities.push(entity);
            }
        }
        
        for entity in active_entities {
            let base_mat = if let Some(transform) = world.get::<Transform2D>(entity) {
                transform.to_mat4()
            } else {
                continue;
            };

            if let Some(emitter) = world.get::<ParticleEmitter>(entity) {
                for p in &emitter.particles {
                    // Calculate life ratio (0.0 to 1.0) for color interpolation
                    let mut ratio = 1.0 - (p.lifetime / p.max_lifetime);
                    if ratio < 0.0 { ratio = 0.0; }
                    if ratio > 1.0 { ratio = 1.0; }

                    let color = crate::renderer::Color {
                        r: p.start_color.r + (p.end_color.r - p.start_color.r) * ratio,
                        g: p.start_color.g + (p.end_color.g - p.start_color.g) * ratio,
                        b: p.start_color.b + (p.end_color.b - p.start_color.b) * ratio,
                        a: p.start_color.a + (p.end_color.a - p.start_color.a) * ratio,
                    };

                    let local_mat = crate::math::Mat4::from_scale_rotation_translation(
                        crate::math::Vec3::new(p.size.x, p.size.y, 1.0),
                        crate::math::Quat::IDENTITY,
                        crate::math::Vec3::new(p.position.x, p.position.y, 0.0),
                    );

                    let final_mat = base_mat * local_mat;

                    let col0 = final_mat.col(0).into();
                    let col1 = final_mat.col(1).into();
                    let col2 = final_mat.col(2).into();
                    let col3 = final_mat.col(3).into();

                    // Build a simple colored quad
                    let quads = [
                        ( [-0.5, -0.5], [0.0, 1.0] ), 
                        ( [ 0.5, -0.5], [1.0, 1.0] ), 
                        ( [-0.5,  0.5], [0.0, 0.0] ), 
                        ( [ 0.5, -0.5], [1.0, 1.0] ),
                        ( [ 0.5,  0.5], [1.0, 0.0] ),
                        ( [-0.5,  0.5], [0.0, 0.0] ),
                    ];

                    let mut quad_vertices = [crate::renderer::sprite_batcher::VertexInput {
                        position: [0.0, 0.0],
                        uv: [0.0, 0.0],
                        color: [0.0; 4],
                        transform_col0: [0.0; 4],
                        transform_col1: [0.0; 4],
                        transform_col2: [0.0; 4],
                        transform_col3: [0.0; 4],
                    }; 6];

                    for (i, (pos, uv)) in quads.iter().enumerate() {
                        quad_vertices[i] = crate::renderer::sprite_batcher::VertexInput {
                            position: *pos,
                            uv: *uv,
                            color: [color.r, color.g, color.b, color.a],
                            transform_col0: col0,
                            transform_col1: col1,
                            transform_col2: col2,
                            transform_col3: col3,
                        };
                    }

                    batcher.push_quad(quad_vertices, 10.0); // Draw particles on top
                }
            }
        }
    }
}

// Simple LCG PRNG for variance
static mut SEED: u32 = 12345;
fn fast_rand() -> f32 {
    unsafe {
        SEED = SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        (SEED as f32) / (std::u32::MAX as f32)
    }
}
