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

struct VertexUniform {
    screen_size: vec2<f32>,
};
@group(0) @binding(0)
var<uniform> uni: VertexUniform;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.col = model.col;
    var clip_pos = (model.pos / uni.screen_size);
    clip_pos.y = 1.0 - clip_pos.y;
    clip_pos = clip_pos * 2.0 - 1.0;
    out.clip_pos = vec4<f32>(clip_pos, 0.0, 1.0);
    return out;
}

// @group(0) @binding(0)
// var diff: texture_2d<f32>;
// @group(0) @binding(1)
// var diff_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(diff, diff_sampler, in.uv) * vec4(in.col, 1.0);
    return vec4(in.col, 1.0);
}
