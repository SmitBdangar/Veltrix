# Hello World Tutorial

This guide walks you through creating your very first Veltrix game from scratch — a window that shows a coloured sprite at the centre of the screen.

---

## Prerequisites

- Rust 1.75+ installed (`rustup update stable`)
- A GPU driver that supports Vulkan, DX12, or Metal

---

## Step 1 — Create a new binary crate

```bash
cargo new my_game
cd my_game
```

## Step 2 — Add Veltrix as a dependency

In `Cargo.toml`:

```toml
[dependencies]
veltrix = { git = "https://github.com/SmitBdangar/Veltrix" }
```

## Step 3 — Write your first game

Replace `src/main.rs` with:

```rust
use veltrix::prelude::*;

// ── Components ────────────────────────────────────────────────

/// Custom tag so we can find our player entity later.
struct Player;

// ── Entry Point ───────────────────────────────────────────────

fn main() {
    env_logger::init(); // optional: see log output

    let mut engine = Engine::new()
        .with_title("Hello, Veltrix!")
        .with_resolution(1280, 720)
        .build();

    // Load a texture from disk
    let texture = engine
        .asset_server
        .load::<Texture>("assets/player.png")
        .expect("player.png not found");

    // Spawn the player entity
    let player = engine.world.spawn();
    engine.world.insert(player, Transform2D {
        position: Vec2::new(640.0, 360.0), // centre of screen
        rotation: 0.0,
        scale:    Vec2::ONE,
    });
    engine.world.insert(player, Sprite::new(texture));
    engine.world.insert(player, Player);

    // Run the engine — the closure is called every frame
    engine.run(|world, res, dt| {
        // Query every entity that has both a Transform2D and the Player tag
        let q = Query::<Transform2D>::new(world);
        for (entity, transform) in q.iter() {
            if world.get::<Player>(entity).is_some() {
                // Simple rotation each frame
                if let Some(mut t) = world.get_mut::<Transform2D>(entity) {
                    t.rotation += 1.0 * dt;
                }
            }
        }
    });
}
```

## Step 4 — Add a placeholder texture

Create an `assets/` folder in your project root and drop any PNG file there named `player.png` (a 64×64 square works great).

## Step 5 — Run it!

```bash
cargo run
```

You should see a window with your sprite spinning at the centre. ✅

---

## What just happened?

| Concept | What it does |
|---|---|
| `Engine::new()…build()` | Initialises wgpu, winit window, and all subsystems |
| `engine.world.spawn()` | Creates a new ECS entity |
| `world.insert(entity, T)` | Attaches component `T` to that entity |
| `Query::<T>::new(world)` | Borrows the world to iterate all entities with component `T` |
| `world.get_mut::<T>(entity)` | Mutably borrows a single component from a known entity |
| `engine.run(closure)` | Starts the fixed-timestep game loop; `dt` is seconds since last frame |

---

## Next Steps

- **Add keyboard movement** — check `veltrix::input::Keyboard::is_pressed`
- **Add a second entity** — spawn another sprite and add physics with `RigidBody2D` + `Collider2D`
- **Play a sound** — `AudioManager::play_sound(&clip_handle, volume)`
- **Load a tilemap** — `TileMapLoader::load("assets/level.tmx")`

Browse the full API at `cargo doc --no-deps --open`.
