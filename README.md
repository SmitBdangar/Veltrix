# Veltrix 2D Engine

**Veltrix** is an experimental, highly modular, data-driven 2D game engine written in Rust, built with modern Rust game development architecture principles in mind. It uses a lightweight bespoke ECS, `wgpu` for fast cross-platform rendering, `winit` for windowing, `rapier2d` for physics, and `rodio` for audio.

## Status

Veltrix is currently in a prototype phase designed to showcase engine architecture. It successfully abstracts over OS sub-routines into clean `core`, `scene`, `ecs`, `input`, `assets`, `renderer`, `physics`, `audio`, `math`, `ui`, `tilemap`, `scripting`, `debug`, `serialization`, and `platform` modules. 

The demo programs currently run headless logic tests using standard types. To activate raw draw calls, the `renderer` module's internal device context wrappers must be built over the `wgpu` surface hook provided in `window::manager`.

## Architecture

Veltrix is split into roughly 15 independent but cleanly integrated subsystems:
- **`core/`**: Fixed timestep `GameLoop`, `EngineBuilder`, `EventBus`, configuration mapping, and delta timing.
- **`ecs/`**: A lightweight and fast array-of-structs Sparse-Set ECS avoiding macro complexity.
- **`assets/`**: Asynchronous `AssetServer`, handle caching, type erasure, and file watching (`notify`).
- **`scene/`**: Data encapsulation, rendering scene graph elements (`Parent`, `Transform2D`, `Name`), and stack manager.
- **`renderer/`**: Types for rendering APIs, batching primitives, and WGSL shaders.
- **`physics/`**: Integrates Rapier2D for rigidbodies, broadphase collision, raycasts, and syncing physical translation data to the ECS `Transform2D`.
- **`audio/`**: Uses `rodio` mapped into a bus graph (`Master`, `SFX`, `Music`) and handles 2D spatial falloff.
- **`math/`**: Linear algebra extensions, `Rect`/`Circle` overlaps, tweening math.
- **`animation/`**: Clip-based sprite atlas tweeners integrated through an FSM `AnimationController`.
- **`camera/`**: Flexible 2D Orthographic matrices featuring trauma-based procedural shake, un-projection logic, and smooth tracking.
- **`tilemap/`**: A mock TMX XML loader handling chunk-based sprite batch submissions.
- **`ui/`**: Immediate-mode styling wrapped over `egui`.
- **`debug/`**: FPS profilers, in-game developer console, and line render logic.
- **`scripting/`**: FSM and Coroutine utilities.
- **`serialization/`**: `RON` based progression serialization and OS abstraction via the `directories` crate.

## Examples

Run the provided engine usage example tests locally to review API structure:

```bash
cargo run --example hello_window
cargo run --example sprite_demo
cargo run --example physics_demo
cargo run --example tilemap_demo
cargo run --example full_game_demo
```

## Setup

Ensure your local development environment supports Vulkan, DX12, or Metal dependencies used by `wgpu`. 

```bash
# Verify it builds cleanly
cargo build
cargo test
```

## Contributing

Veltrix was architected to be extensible. To add new custom components, declare standard struct data in the `scene` module, then wrap them gracefully inside a `QueryMut` system attached to one of the execution loops (Update/FixedUpdate/Render) bound onto the main `Engine.run(...)` trait method parameter.

## License

MIT License.
