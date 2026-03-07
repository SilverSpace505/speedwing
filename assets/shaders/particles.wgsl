#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) colour: vec4<f32>,
    @location(2) size: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) colour: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex, @builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
    var positions = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f(1.0, -1.0), vec2f(-1.0, 1.0),
        vec2f(-1.0, 1.0), vec2f(1.0, -1.0), vec2f(1.0, 1.0)
    );

    let offset = positions[vertexIndex % 6u] * vertex.size;
    let pos = vec4<f32>(vertex.position.xy + offset, vertex.position.z, 1.0);

    var out: VertexOutput;
    let world_from_local = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(
        world_from_local,
        pos,
    );
    out.uv = positions[vertexIndex % 6u];
    out.colour = vertex.colour;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv);
    if (d > 1) {
        discard;
    }
    let delta = fwidth(d);
    let alpha = 1.0 - smoothstep(-delta, 1.0, d);

    return vec4f(in.colour.rgb, in.colour.a * alpha);
}