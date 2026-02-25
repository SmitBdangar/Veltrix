# Changelog

All notable changes to **Veltrix** are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).  
Veltrix uses [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Planned
- Networked multiplayer support
- Hot-reload shaders at runtime
- Visual scene editor (egui-based)
- WebAssembly / WebGPU target

---

## [0.2.0] — 2026-02-25

### Added
- **Text Rendering** — `FontAsset`, `FontAtlas` (ab_glyph rasterizer), `TextBatcher::draw_text`
- **Particle System** — `ParticleEmitter` + `Particle` ECS components, `ParticleSystem::update` + `::draw`
- **Render Layers** — `SpriteBatcher` now accepts `z_index` per quad, sorts all draw calls back-to-front before GPU upload
  - Layer convention: sprites `0.0`, particles `10.0`, UI text `50.0`, screen transitions `100.0`
- **Screen Transitions** — `ScreenTransition` with `FadeIn` / `FadeOut` kinds; `update(dt)` + `draw(batcher, w, h)`
- **Resolution Scaling** — virtual resolution with letterboxing; configure via `Engine::with_resolution`
- **Error Screen** — global panic hook captures crash info and renders a readable error screen
- **Asset Bundling** — pack assets into a binary-embeddable archive via `AssetBundler`
- **Delta Time Smoothing** — rolling-average smoothing prevents frame-time jitter
- **Window Icon** — set custom app icon with `Engine::with_icon`
- **Cursor Customization** — hide, show, or replace the cursor via `WindowManager`
- `Text` built-in ECS component
- `ParticleEmitter` + `Particle` built-in ECS components

### Changed
- `SpriteBatcher::new` now initialises an internal `draw_calls: Vec<DrawCall>` queue instead of a flat vertex vec
- `SpriteBatcher::push_quad(vertices, z_index)` is the new primary API for custom renderers

### Fixed
- Duplicate `impl Asset for FontAsset` compile error in `renderer::text`
- Type mismatch `usize` vs `u32` in `FontAtlas::new` pixel-coordinate math

---

## [0.1.0] — 2026-01-15

### Added
- Full 16-module engine architecture (core, ecs, renderer, scene, physics, audio, camera, animation, tilemap, ui, scripting, debug, serialization, platform, input, window)
- Custom sparse-set ECS: `World`, `Entity`, `Query`, `QueryMut`, `Resources`, `Commands`
- `wgpu` 0.19 rendering backend with `SpriteBatcher` and WGSL shaders (`sprite.wgsl`, `shape.wgsl`)
- `AssetServer` with typed `Handle<T>` and file-watching hot-reload
- `rapier2d` rigid bodies, colliders, broadphase, raycasting
- `rodio` audio manager with Master/SFX/Music bus graph and 2D spatial falloff
- Orthographic `Camera2D` with trauma-based shake and smooth entity follow
- `AnimationController` FSM with `AnimationClip` atlas tweening and easing functions
- Chunk-based `TileMap` renderer and `.tmx` Tiled map loader
- `egui`-based UI canvas with Label, Button, Layout, and `DebugOverlay`
- `Behavior` trait, `Coroutine` executor, `StateMachine<S>`
- `DebugRenderer` (line segments), `Profiler`, `Console`, `Inspector`
- RON save-game system and scene serialiser
- FileSystem, Clipboard, SystemInfo platform abstractions
- Keyboard, Mouse, Gamepad (gilrs) input and `ActionMap`
- Pong demo (`examples/pong.rs`) showcasing full ECS + Physics + Rendering

[Unreleased]: https://github.com/SmitBdangar/Veltrix/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/SmitBdangar/Veltrix/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SmitBdangar/Veltrix/releases/tag/v0.1.0
