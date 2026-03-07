#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}
#import bevy_render::globals::Globals

struct CustomMaterial {
    color: vec4<f32>,
};
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: CustomMaterial;
@group(0) @binding(1) var<uniform> globals: Globals;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) v: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) v: f32,
    @location(1) world_position: vec2<f32>
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    let world_pos = world_from_local * vec4<f32>(vertex.position, 1.0);
    out.world_position = world_pos.xy;


    out.v = vertex.v;
    return out;
}

struct FragmentInput {
    @location(0) v: f32,
    @location(1) world_position: vec2<f32>
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let t = globals.time;
    let v = min(input.v * 3.0, 1.0);
    let mul = input.v * 0.2 + 0.8;
    let v2 = pow(1.0 - v, 2.0);
    let sinadd = sin((mul * 10) + (t * 3) + input.world_position.x / 5 / 2 + mul + input.world_position.y / 5 / 2 + mul);
    let cosadd = cos((mul * 35) + (t * 2) + input.world_position.x / 25 / 2 + mul + input.world_position.y / 25 / 2 + mul);
    let v3 = v2 + sinadd / 10 - cosadd / 10;
    return material.color * v3 * pow(1.0 - input.v, 3.0);
}