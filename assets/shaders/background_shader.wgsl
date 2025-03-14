// Reduced version of the texture_shader to render background images!

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

@group(0) @binding(0) var background_texture: texture_2d<f32>;
@group(0) @binding(1) var background_sampler: sampler;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.texture_coordinates = model.texture_coordinates;
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(background_texture, background_sampler, input.texture_coordinates);
}
