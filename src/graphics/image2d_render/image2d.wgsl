
struct VertexInput {
    @location(0) vert_position: vec2<f32>,
    @location(1) tex_position: vec2<f32>,
    @location(2) tex_size: vec2<f32>
}

struct InstanceInput {
    @location(3) position: vec2<f32>,
    @location(4) scale: f32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_position: vec2<f32>
}

@vertex
fn vs_main(vert: VertexInput, instance: InstanceInput) -> VertexOutput{
    var out: VertexOutput;

    out.clip_position = vec4<f32>(vert.vert_position * vert.tex_size * instance.scale + instance.position, 0, 1);

    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, in.tex_position);
}

