# Veltrix 2D Game Engine

<div align="center">

[![Crates.io](https://img.shields.io/badge/crates.io-veltrix-orange)](https://crates.io/crates/veltrix)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange)](https://www.rust-lang.org)
[![cargo check](https://img.shields.io/badge/cargo%20check-passing-brightgreen)](#)

**A modular, production-ready 2D game engine built entirely in Rust.**

[Quickstart](#quickstart) · [Architecture](#architecture) · [Examples](#examples) · [API Docs](#documentation) · [Contributing](#contributing)

</div>

---

## Features

| Module | Capabilities |
|---|---|
| 🎮 **ECS** | Fast bespoke archetype ECS — `World`, `Query`, `QueryMut`, `Resources` |
| 🖥️ **Renderer** | `wgpu` sprite batching, z-sorted render layers, texture atlases, WGSL shaders |
| 🅰️ **Text** | `ab_glyph` font rasterizer, ASCII atlas, `TextBatcher` |
| ✨ **Particles** | ECS-driven emitters with lifetime, velocity variance, and color gradients |
| 🎬 **Transitions** | `ScreenTransition` fade-in/out with pluggable scene manager |
| ⚡ **Physics** | `rapier2d` rigid bodies, colliders, raycasting, broadphase |
| 🔊 **Audio** | `rodio` bus graph (Master/SFX/Music), 2D spatial falloff |
| 🗺️ **Tilemap** | Chunk-based tile batching, `.tmx` Tiled loader |
| 🎥 **Camera** | Orthographic, trauma-based shake, smooth entity-follow |
| ⌨️ **Input** | Keyboard, mouse, gamepad (gilrs), action maps |
| 🖼️ **Assets** | Typed handles, async `AssetServer`, hot-reload via `notify` |
| 🖱️ **Window** | `winit` windowing, custom icon, cursor hide/show |
| 🐛 **Debug** | FPS profiler, in-game console, inspector, debug line renderer |
| 💾 **Save** | `RON`-based save/load, scene serialiser |

---

## Quickstart

### Prerequisites

- **Rust** 1.75+ (`rustup update stable`)
- **GPU drivers** supporting Vulkan, DX12, or Metal (any modern GPU works)

### Add to your project

```toml
# Cargo.toml
[dependencies]
veltrix = { git = "https://github.com/SmitBdangar/Veltrix" }
```

### Hello World

```rust
use veltrix::prelude::*;

fn main() {
    // 1. Build the engine
    let mut engine = Engine::new()
        .with_title("Hello Veltrix")
        .with_resolution(1280, 720)
        .build();

    // 2. Spawn an entity with a position
    let entity = engine.world.spawn();
    engine.world.insert(entity, Transform2D {
        position: Vec2::new(640.0, 360.0),
        rotation: 0.0,
        scale: Vec2::ONE,
    });

    // 3. Run! (supplies a fixed game loop at 60 Hz)
    engine.run(|_world, _res, _dt| {
        // your per-frame logic here
    });
}
```

> Full tutorial → [docs/hello_world.md](docs/hello_world.md)

---

## Examples

```bash
# Open a window and draw a sprite
cargo run --example hello_window

# Animated sprites
cargo run --example sprite_demo

# Physics bodies and collision
cargo run --example physics_demo

# Tiled map loading
cargo run --example tilemap_demo

# Pong clone — showcases ECS, Input, Renderer, and Physics
cargo run --example pong

# Large scene stress test
cargo run --example full_game_demo
```

---

## Architecture

```
veltrix/
├── src/
│   ├── core/           Engine, GameLoop (fixed 60 Hz), EventBus, Time, Config
│   ├── ecs/            World, Entity, ComponentStorage, Query, Resources
│   ├── renderer/       RenderDevice (wgpu), SpriteBatcher (z-sorted), particles, text
│   ├── scene/          Scene, SceneManager (stack), components, ScreenTransition
│   ├── assets/         AssetServer, Handle<T>, hot-reload
│   ├── physics/        rapier2d wrapper, ECS sync, raycast, CollisionEvents
│   ├── audio/          AudioManager, AudioBus (rodio), spatial falloff
│   ├── camera/         Camera2D, shake, smooth follow
│   ├── animation/      AnimationClip, AnimationController FSM, Tweening
│   ├── tilemap/        TileMap, TileSet, chunk renderer, .tmx loader
│   ├── ui/             egui canvas, widgets, debug overlay
│   ├── input/          Keyboard, Mouse, Gamepad, ActionMap
│   ├── math/           Transform2D, Rect, Circle, Vec2/Vec3/Mat4
│   ├── scripting/      Behavior trait, Coroutines, StateMachine<S>
│   ├── debug/          DebugRenderer, Profiler, Console, Inspector
│   ├── serialization/  SaveGame (RON), SceneSerializer
│   └── platform/       FileSystem, Clipboard, SystemInfo
├── examples/
├── assets/shaders/     sprite.wgsl, shape.wgsl
└── docs/               hello_world.md, architecture.md
```

### Render Layer Z-Indices

| Z-Index | Content |
|--------:|---------|
| `0.0` | World sprites and tilemaps |
| `10.0` | Particle effects |
| `50.0` | UI / HUD text |
| `100.0` | Screen transition overlay |

---

## Documentation

```bash
# Build and open the full API docs locally
cargo doc --no-deps --open
```

All public types, structs, and functions carry `///` doc comments with usage notes.

---

## Building & Testing

```bash
# Check everything compiles
cargo check

# Run unit tests
cargo test

# Lint (recommended before committing)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Release build (LTO enabled, stripped)
cargo build --release
```

---

## Contributing

1. Fork the repository  
2. Create a feature branch (`git checkout -b feat/my-system`)  
3. Write code + doc comments + tests  
4. Run `cargo fmt && cargo clippy` — fix any warnings  
5. Open a pull request against `main`

See [CHANGELOG.md](CHANGELOG.md) for version history.

---

## License

MIT — see [LICENSE](LICENSE) for details.
