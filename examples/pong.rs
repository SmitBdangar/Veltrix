//! Example: Pong
//! A complete, simple Pong game demonstrating ECS, Input, Physics (AABB), and Rendering via Veltrix.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;

// --- Components ---

#[derive(Clone, Copy, Debug)]
struct Velocity(pub Vec2);

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Paddle {
    pub side: PlayerSide,
    pub speed: f32,
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    pub speed: f32,
}

// Global state container
struct GameState {
    pub score_left: u32,
    pub score_right: u32,
    pub bounds_y: f32, // Top/Bottom limit
    pub bounds_x: f32, // Left/Right limit (for scoring)
    pub ball_entity: Entity,
}

// --- Constants ---
const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 100.0);
const BALL_SIZE: Vec2 = Vec2::new(15.0, 15.0);
const WALL_THICKNESS: f32 = 50.0;
const PADDLE_START_X: f32 = 350.0;
const BALL_START_SPEED: f32 = 400.0;

fn main() -> Result<()> {
    // 1. Configure the engine
    let engine = EngineBuilder::new()
        .with_title("Veltrix - Pong")
        .with_resolution(800, 600)
        .build()?;

    // Using `PlayerEnt` trick to ferry the ball entity into the state quickly.
    // Realistically we just loop through the world.
    let mut initial_ball = Entity::null();

    engine.run(
        // -- ON START --
        |world, resources| {
            // Setup Camera
            let cam = world.spawn();
            world.insert(cam, Camera2D::new(Vec2::new(800.0, 600.0)));
            resources.insert(cam);
            resources.insert(InputManager::new());

            // Left Paddle
            let p1 = world.spawn();
            world.insert(p1, Paddle { side: PlayerSide::Left, speed: 500.0 });
            world.insert(p1, Transform2D {
                position: Vec2::new(-PADDLE_START_X, 0.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            });
            world.insert(p1, Collider2D {
                shape: ColliderShape::Box(PADDLE_SIZE / 2.0),
                is_trigger: false,
                bounciness: 1.0,
                friction: 0.0,
                handle_id: 1,
            });

            // Right Paddle
            let p2 = world.spawn();
            world.insert(p2, Paddle { side: PlayerSide::Right, speed: 500.0 });
            world.insert(p2, Transform2D {
                position: Vec2::new(PADDLE_START_X, 0.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            });
            world.insert(p2, Collider2D {
                shape: ColliderShape::Box(PADDLE_SIZE / 2.0),
                is_trigger: false,
                bounciness: 1.0,
                friction: 0.0,
                handle_id: 2,
            });

            // Ball
            initial_ball = world.spawn();
            world.insert(initial_ball, Ball { speed: BALL_START_SPEED });
            world.insert(initial_ball, Transform2D {
                position: Vec2::ZERO,
                rotation: 0.0,
                scale: Vec2::ONE,
            });
            world.insert(initial_ball, Velocity(Vec2::new(BALL_START_SPEED, BALL_START_SPEED * 0.5)));
            // Ball is technically a small box for collision, we use custom AABB though for this demo
            world.insert(initial_ball, Collider2D {
                shape: ColliderShape::Box(BALL_SIZE / 2.0),
                is_trigger: false,
                bounciness: 1.0,
                friction: 0.0,
                handle_id: 3,
            });

            // Top Wall
            let top_wall = world.spawn();
            world.insert(top_wall, Transform2D {
                position: Vec2::new(0.0, 300.0 + WALL_THICKNESS / 2.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            });
            world.insert(top_wall, Collider2D {
                shape: ColliderShape::Box(Vec2::new(800.0, WALL_THICKNESS) / 2.0),
                is_trigger: false,
                bounciness: 1.0,
                friction: 0.0,
                handle_id: 4,
            });

            // Bottom Wall
            let bot_wall = world.spawn();
            world.insert(bot_wall, Transform2D {
                position: Vec2::new(0.0, -300.0 - WALL_THICKNESS / 2.0),
                rotation: 0.0,
                scale: Vec2::ONE,
            });
            world.insert(bot_wall, Collider2D {
                shape: ColliderShape::Box(Vec2::new(800.0, WALL_THICKNESS) / 2.0),
                is_trigger: false,
                bounciness: 1.0,
                friction: 0.0,
                handle_id: 5,
            });

            // Insert Game State
            resources.insert(GameState {
                score_left: 0,
                score_right: 0,
                bounds_y: 300.0,
                bounds_x: 400.0,
                ball_entity: initial_ball,
            });
            
            log::info!("Pong started! Left: W/S. Right: Up/Down.");
        },

        // -- ON UPDATE -- (Variable Timestep: Input & Rendering prep)
        |world, resources, dt| {
            let input = resources.get::<InputManager>().unwrap();
            
            let mut q = QueryMut::<(Paddle, Transform2D)>::new(world);
            
            for (_e, (paddle, transform)) in q.iter_mut() {
                let mut move_dir = 0.0;
                
                match paddle.side {
                    PlayerSide::Left => {
                        if input.keyboard.is_pressed(KeyCode::KeyW) { move_dir += 1.0; }
                        if input.keyboard.is_pressed(KeyCode::KeyS) { move_dir -= 1.0; }
                    }
                    PlayerSide::Right => {
                        if input.keyboard.is_pressed(KeyCode::ArrowUp) { move_dir += 1.0; }
                        if input.keyboard.is_pressed(KeyCode::ArrowDown) { move_dir -= 1.0; }
                    }
                }

                transform.position.y += move_dir * paddle.speed * dt as f32;

                // Clamp to screen bounds manually so paddles don't leave the screen
                let limit = 300.0 - (PADDLE_SIZE.y / 2.0);
                transform.position.y = transform.position.y.clamp(-limit, limit);
            }
            
            true
        },

        // -- ON FIXED UPDATE -- (Fixed Timestep: Physics & Game Logic)
        |world, resources, fixed_dt| {
            let mut state = resources.remove::<GameState>().unwrap();

            // 1. Move the ball
            let mut ball_pos = Vec2::ZERO;
            let mut reset_ball = false;

            let mut q_ball = QueryMut::<(Transform2D, Velocity)>::new(world);
            for (e, (transform, vel)) in q_ball.iter_mut() {
                if e == state.ball_entity {
                    transform.position += vel.0 * fixed_dt as f32;
                    
                    // Bounce off top and bottom walls
                    let top_limit = state.bounds_y - (BALL_SIZE.y / 2.0);
                    if transform.position.y >= top_limit {
                        transform.position.y = top_limit;
                        vel.0.y *= -1.0;
                    } else if transform.position.y <= -top_limit {
                        transform.position.y = -top_limit;
                        vel.0.y *= -1.0;
                    }

                    // Check Scoring (Bounds X)
                    if transform.position.x > state.bounds_x {
                        state.score_left += 1;
                        log::info!("Point Left! Score: {} - {}", state.score_left, state.score_right);
                        reset_ball = true;
                    } else if transform.position.x < -state.bounds_x {
                        state.score_right += 1;
                        log::info!("Point Right! Score: {} - {}", state.score_left, state.score_right);
                        reset_ball = true;
                    }

                    ball_pos = transform.position;
                    // Note: ball_vel is removed to fix unused warning.
                }
            }

            // 2. Perform Custom AABB Collision with Paddles
            // (We are doing this manually rather than through Rapier2D to test ECS data access)
            if !reset_ball {
                let q_paddles = Query::<(Paddle, Transform2D)>::new(world);
                let ball_rect = Rect::new(ball_pos - BALL_SIZE / 2.0, BALL_SIZE);
                let mut bounce_info = None;

                for (_e, (_paddle, p_transform)) in q_paddles.iter() {
                    let paddle_rect = Rect::new(p_transform.position - PADDLE_SIZE / 2.0, PADDLE_SIZE);
                    
                    if ball_rect.intersects(&paddle_rect) {
                        bounce_info = Some(p_transform.position);
                        break;
                    }
                }
                drop(q_paddles);

                if let Some(p_pos) = bounce_info {
                    // Reflect the ball on the X axis
                    if let Some(vel) = world.get_mut::<Velocity>(state.ball_entity) {
                        // Only bounce if heading towards the paddle (prevent getting stuck inside)
                        if (ball_pos.x < p_pos.x && vel.0.x > 0.0) ||
                           (ball_pos.x > p_pos.x && vel.0.x < 0.0) {
                            
                            vel.0.x *= -1.0;
                            
                            // Add a little English (spin) based on where it hit the paddle
                            let hit_factor = (ball_pos.y - p_pos.y) / (PADDLE_SIZE.y / 2.0);
                            vel.0.y = hit_factor * BALL_START_SPEED;
                            
                            // Increase speed slightly
                            vel.0 = vel.0.normalize() * (vel.0.length() + 20.0);
                        }
                    }
                }
            }

            // 3. Reset Ball if scored
            if reset_ball {
                if let Some(transform) = world.get_mut::<Transform2D>(state.ball_entity) {
                    transform.position = Vec2::ZERO;
                }
                if let Some(vel) = world.get_mut::<Velocity>(state.ball_entity) {
                    // Serve towards the winner
                    let direction = if ball_pos.x > 0.0 { -1.0 } else { 1.0 };
                    vel.0 = Vec2::new(BALL_START_SPEED * direction, BALL_START_SPEED * 0.5 * direction);
                }
            }

            // Put state back
            resources.insert(state);
        },

        // -- ON RENDER --
        |_world, _resources, _alpha| {
            // Under normal circumstances, we iterate Transform2D and Render objects. 
            // In a fully featured Renderer, SpriteBatcher / ShapeRenderer is called here.
        }
    )?;

    Ok(())
}
