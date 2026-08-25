override screen_width: f32 = 180.0;
override screen_height: f32 = 320.0;

fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    let cutoff = c < vec3<f32>(0.0031308);
    let lo = c * 12.92;
    let hi = 1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, cutoff);
}

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let cutoff = c < vec3<f32>(0.04045);
    let lo = c / 12.92;
    let hi = pow((c + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, cutoff);
}

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) col: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) col: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.col = srgb_to_linear(model.col);
    var clip_pos = (model.pos / vec2(screen_width, screen_height));
    clip_pos.y = 1.0 - clip_pos.y;
    clip_pos = clip_pos * 2.0 - 1.0;
    out.clip_pos = vec4<f32>(clip_pos, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var atlas: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(atlas, atlas_sampler, in.uv) * vec4(in.col, 1.0);
}
