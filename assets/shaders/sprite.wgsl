// Veltrix — Sprite WGSL Shader
//
// Vertex stage: transforms instance data (position, UV, color, transform matrix)
// using the camera's view-projection matrix.
//
// Fragment stage: samples from a texture atlas and multiplies by the per-instance
// color tint. Alpha blending must be enabled in the render pipeline state.

// ── Bind group 0: camera uniform ─────────────────────────────────────────────
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ── Bind group 1: texture + sampler ──────────────────────────────────────────
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

// ── Vertex input (per-instance, packed into vertex buffer) ────────────────────
struct VertexInput {
    // Quad corner position in local space ([-0.5, 0.5]).
    @location(0) position:  vec2<f32>,
    // Texture UV for this corner.
    @location(1) uv:        vec2<f32>,
    // Per-instance RGBA color tint.
    @location(2) color:     vec4<f32>,
    // Per-instance 4×4 model transform (columns packed as 4 vec4s).
    @location(3) transform_col0: vec4<f32>,
    @location(4) transform_col1: vec4<f32>,
    @location(5) transform_col2: vec4<f32>,
    @location(6) transform_col3: vec4<f32>,
}

// ── Vertex → fragment interpolants ────────────────────────────────────────────
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       uv:            vec2<f32>,
    @location(1)       color:         vec4<f32>,
}

// ── Vertex shader ─────────────────────────────────────────────────────────────
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // Reconstruct the per-instance model matrix from packed columns.
    let model = mat4x4<f32>(
        in.transform_col0,
        in.transform_col1,
        in.transform_col2,
        in.transform_col3,
    );

    var out: VertexOutput;
    // Position: model → world → clip space via camera view-projection.
    let world_pos = model * vec4<f32>(in.position, 0.0, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    out.uv    = in.uv;
    out.color = in.color;
    return out;
}

// ── Fragment shader ───────────────────────────────────────────────────────────
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv);
    // Discard fully-transparent pixels for crisp sprite edges.
    if tex_color.a < 0.01 {
        discard;
    }
    return tex_color * in.color;
}
