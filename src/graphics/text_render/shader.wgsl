// TEXT RENDERING

// VERTEX
struct VertexInput {
    @location(0) vert_pos: vec3<f32>,
    @location(1) tex_pos: vec2<f32>,
}

struct InstanceInput {
    @location(2) sentence_position: vec3<f32>,
    @location(3) letter_position_px: vec3<f32>,
    @location(4) direction: vec4<f32>,
    @location(5) tex_offset: vec2<f32>,
    @location(6) tex_size: vec2<f32>,
    @location(7) color: vec4<f32>,
    @location(8) scale: f32,
    @location(9) affected_by_camera: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>,
    @location(1) tex_offset: vec2<f32>,
    @location(2) tex_size: vec2<f32>,
    @location(3) color: vec4<f32>
};

struct Uniform {
    view_proj: mat4x4<f32>,
    screen_size: vec2<f32>
}

@group(0) @binding(0)
var<uniform> uni: Uniform;

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    if instance.affected_by_camera == 1u {
        // Primary output
       let letter_pos_cs = instance.letter_position_px / vec3<f32>(uni.screen_size, 1.0) + instance.sentence_position;
        let scaled_vert = model.vert_pos * vec3<f32>(instance.tex_size * instance.scale / uni.screen_size, 0);

        out.clip_position = uni.view_proj * vec4<f32>(scaled_vert + letter_pos_cs, 1);
    } else {
        let letter_pos_cs = instance.letter_position_px / vec3<f32>(uni.screen_size, 1.0) + instance.sentence_position;
        let scaled_vert = model.vert_pos * vec3<f32>(instance.tex_size * instance.scale / uni.screen_size, 0);

        out.clip_position = vec4<f32>(scaled_vert + letter_pos_cs, 1);
    }

    // Send to fragment
    out.tex_pos = model.tex_pos;
    out.tex_offset = instance.tex_offset;
    out.tex_size = instance.tex_size;
    out.color = instance.color;

    return out;
}


// FRAGMENT
@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dims = textureDimensions(texture);
    let dims_f32 = vec2<f32>(f32(dims.x), f32(dims.y));
    return textureSample(texture, tex_sampler, (in.tex_offset + in.tex_pos * in.tex_size) / dims_f32) * in.color;
}

fn rotate_vec3_by_quat(v: vec3<f32>, q: vec4<f32>) -> vec3<f32> {
    let u = q.xyz;
    let t = 2.0 * cross(u, v);
    return v + q.w * t + cross(u, t);
}