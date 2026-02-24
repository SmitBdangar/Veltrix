// Veltrix — Shape WGSL Shader
//
// Renders 2D primitive shapes (filled rects, circles, lines) used by the
// DebugRenderer and UICanvas. Each vertex carries a position and color;
// no texturing is performed.

// ── Bind group 0: camera uniform ─────────────────────────────────────────────
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ── Vertex input ──────────────────────────────────────────────────────────────
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color:    vec4<f32>,
}

// ── Vertex → fragment ─────────────────────────────────────────────────────────
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       color:         vec4<f32>,
}

// ── Vertex shader ─────────────────────────────────────────────────────────────
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

// ── Fragment shader ───────────────────────────────────────────────────────────
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
