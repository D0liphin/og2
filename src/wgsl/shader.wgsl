struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] texture_coordinates: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] texture_coordinates: vec2<f32>;
};

[[block]] struct MatrixInput {
    matrix: mat3x2<f32>;
};

// Bind Group

[[group(0), binding(0)]]
var external_texture: texture_2d<f32>;

[[group(0), binding(1)]]
var sampler_type: sampler;

[[group(0), binding(2)]]
var<uniform> matrix_input: MatrixInput;

// Entry Points

[[stage(vertex)]]
fn main(vertex_input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = vertex_input.texture_coordinates;
    let position = vec3<f32>(vertex_input.position.x, vertex_input.position.y, 1.0);
    out.clip_position = vec4<f32>(matrix_input.matrix * position, 1.0, 1.0);
    return out;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(external_texture, sampler_type, in.texture_coordinates);
}