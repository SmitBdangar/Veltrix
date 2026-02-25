//! Example: Pong
//! A complete, simple Pong game demonstrating ECS, Input, Physics (AABB), and Rendering via Veltrix.

use anyhow::Result;
use glam::Vec2;
use veltrix::prelude::*;
use std::borrow::Cow;
use wgpu::util::DeviceExt;

// --- Components ---

/// Player side enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSide {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Paddle {
    pub side: PlayerSide,
    pub speed: f32,
}

#[derive(Debug, Clone)]
struct Ball {
    pub _speed: f32, // Prevent dead code warning
}

#[derive(Debug, Clone)]
struct Velocity(Vec2);

// Global state container
struct GameState {
    pub score_left: u32,
    pub score_right: u32,
    pub bounds_y: f32, // Top/Bottom limit
    pub bounds_x: f32, // Left/Right limit (for scoring)
    pub ball_entity: Entity,
}

// ---- Rendering Structs ----
struct PongRenderer {
    pub pipeline: veltrix::renderer::pipeline::RenderPipeline,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub batcher: veltrix::renderer::SpriteBatcher,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

// Store diffuse bind group manually in resources since we hacked it for tests
pub struct GlobalTextureBindGroup(wgpu::BindGroup);

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
    let mut initial_ball = Entity::default();
            
    engine.run(
        // -- ON START --
        |world, resources| {
            // Generate dummy 1x1 white texture since we need bind group 1 for our shader
            let rd = resources.get_mut::<veltrix::renderer::RenderDevice>().unwrap();
            let texture_size = wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 };
            let diffuse_texture = rd.device.create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            });
            rd.queue.write_texture(
                wgpu::ImageCopyTexture { texture: &diffuse_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                &[255, 255, 255, 255],
                wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(4), rows_per_image: Some(1) },
                texture_size,
            );
            let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let diffuse_sampler = rd.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            // Shader
            let shader = rd.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Sprite Shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../assets/shaders/sprite.wgsl"))),
            });

            // Camera setup (Bind Group 0)
            let proj = glam::Mat4::orthographic_rh(-400.0, 400.0, -300.0, 300.0, -100.0, 100.0);
            let camera_uniform = CameraUniform { view_proj: proj.to_cols_array_2d() };
            let camera_buffer = rd.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let camera_bind_group_layout = rd.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

            let camera_bind_group = rd.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() }],
                label: Some("camera_bind_group"),
            });

            // Texture setup (Bind Group 1)
            let texture_bind_group_layout = rd.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

            let _texture_bind_group = rd.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&diffuse_texture_view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&diffuse_sampler) },
                ],
                label: Some("diffuse_bind_group"),
            });

            let pipeline_layout = rd.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

            let wgpu_pipeline = rd.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Sprite Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[veltrix::renderer::sprite_batcher::VertexInput::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: rd.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            resources.insert(PongRenderer {
                pipeline: veltrix::renderer::pipeline::RenderPipeline { wgpu_pipeline },
                camera_bind_group,
                camera_buffer,
                batcher: veltrix::renderer::SpriteBatcher::new(),
            });

            resources.insert(GlobalTextureBindGroup(_texture_bind_group));

            // Setup Camera
            let cam = world.spawn();
            let mut camera = Camera2D::new(Vec2::new(800.0, 600.0));
            // Ensure the pong demo maintains a fixed 4:3 aspect ratio internally
            camera.virtual_resolution = Some(Vec2::new(800.0, 600.0)); 
            world.insert(cam, camera);
            resources.insert(cam);
            resources.insert(InputManager::new());

            // Left Paddle
            let p1 = world.spawn();
            world.insert(p1, Paddle { side: PlayerSide::Left, speed: 500.0 });
            world.insert(p1, Transform2D {
                position: Vec2::new(-PADDLE_START_X, 0.0),
                rotation: 0.0,
                scale: PADDLE_SIZE, // Visually match collider size
            });
            world.insert(p1, Sprite {
                texture: None::<veltrix::assets::Handle<veltrix::renderer::Texture>>,
                color: Color::WHITE,
                flip_x: false,
                flip_y: false,
                src_rect: None,
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
                scale: PADDLE_SIZE, // Visually match collider size
            });
            world.insert(p2, Sprite {
                texture: None::<veltrix::assets::Handle<veltrix::renderer::Texture>>,
                color: Color::WHITE,
                flip_x: false,
                flip_y: false,
                src_rect: None,
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
            world.insert(initial_ball, Ball { _speed: BALL_START_SPEED });
            world.insert(initial_ball, Transform2D {
                position: Vec2::ZERO,
                rotation: 0.0,
                scale: BALL_SIZE,
            });
            world.insert(initial_ball, Sprite {
                texture: None::<veltrix::assets::Handle<veltrix::renderer::Texture>>,
                color: Color::WHITE,
                flip_x: false,
                flip_y: false,
                src_rect: None,
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
                scale: Vec2::new(800.0, WALL_THICKNESS),
            });
            world.insert(top_wall, Sprite {
                texture: None::<veltrix::assets::Handle<veltrix::renderer::Texture>>,
                color: Color::GRAY,
                flip_x: false,
                flip_y: false,
                src_rect: None,
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
                scale: Vec2::new(800.0, WALL_THICKNESS),
            });
            world.insert(bot_wall, Sprite {
                texture: None::<veltrix::assets::Handle<veltrix::renderer::Texture>>,
                color: Color::GRAY,
                flip_x: false,
                flip_y: false,
                src_rect: None,
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
        |world, resources, _alpha| {
            // Pull out our custom rendering states first to avoid double borrowing `resources`
            let mut renderer_state = resources.remove::<PongRenderer>().unwrap();
            let tex_bind = resources.remove::<GlobalTextureBindGroup>().unwrap();
            let camera_entity = *resources.get::<veltrix::ecs::Entity>().unwrap();
            
            let mut rd = resources.get_mut::<veltrix::renderer::RenderDevice>().unwrap();
            
            // 1. Acquire swapchain texture
            let output = match rd.begin_frame() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => return,
                Err(e) => {
                    log::error!("Dropped frame: {:?}", e);
                    return;
                }
            };
            
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            
            // 2. Clear Screen
            let mut encoder = rd.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            // Pre-calculate camera viewport so we don't borrow `world` mutably while `render_pass` is active
            let mut vp_rect = (0.0, 0.0, rd.size.width as f32, rd.size.height as f32);
            if let Some(mut current_cam) = world.get_mut::<Camera2D>(camera_entity) {
                // Update camera with current physical window size so it can letterbox correctly
                current_cam.set_viewport(rd.size.width as f32, rd.size.height as f32);
                vp_rect = current_cam.calculate_viewport_rect();
            }

            { // Render Pass Scope
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.05,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                
                let (vx, vy, vw, vh) = vp_rect;
                if vw > 0.0 && vh > 0.0 {
                    render_pass.set_viewport(vx, vy, vw, vh, 0.0, 1.0);
                }

                // 3. Batch and Draw Sprites
                // Build the dynamic vertex batch
                let mut q = QueryMut::<(Transform2D, Sprite)>::new(world);
                for (_e, (transform, sprite)) in q.iter_mut() {
                    renderer_state.batcher.draw_sprite(sprite, transform, 0.0);
                }
                
                // Push it to GPU inside the render pass
                render_pass.set_pipeline(&renderer_state.pipeline.wgpu_pipeline);
                render_pass.set_bind_group(0, &renderer_state.camera_bind_group, &[]);
                render_pass.set_bind_group(1, &tex_bind.0, &[]);
                renderer_state.batcher.flush(&rd, &mut render_pass);
            } // Close encoder block, dropping render_pass which releases borrows of renderer_state
            
            // 4. Submit to GPU
            rd.queue.submit(std::iter::once(encoder.finish()));
            output.present();
            
            // Drop our RenderDevice borrow so we can safely mutate resources again
            drop(rd);
            
            // Store our custom rendering states back
            resources.insert(renderer_state);
            resources.insert(tex_bind);
        }
    )?;

    Ok(())
}
