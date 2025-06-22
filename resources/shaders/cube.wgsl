
// VERTEX
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_pos: vec3<f32>,
    @location(2) tex_offset: f32
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_pos: vec3<f32>,
    @location(1) tex_offset: f32
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Primary output
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1);

    // Send to fragment
    out.tex_pos = model.tex_pos;
    out.tex_offset = model.tex_offset;

    return out;
}


// FRAGMENT
@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, vec2(in.tex_pos.x + in.tex_offset, in.tex_pos.y));
}