
struct Output {
    [[builtin(position)]] position : vec4<f32>;
    [[location(0)]] v_color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> Output {    
    var pos = array<vec2<f32>,6>(
        vec2<f32>(-0.5,  0.7),
        vec2<f32>( 0.3,  0.6),
        vec2<f32>( 0.5,  0.3),
        vec2<f32>( 0.4, -0.5),
        vec2<f32>(-0.4, -0.4),
        vec2<f32>(-0.3,  0.2)
    );

    var colors = array<vec3<f32>,6>(
        vec3<f32>(0.1,  0.0, 0.0),
        vec3<f32>( 0.0,  0.1, 0.0),
        vec3<f32>( 0.0,  0.0, 0.1),
        vec3<f32>( 0.1, 0.0, 0.1),
        vec3<f32>(0.1, 0.1, 0.0),
        vec3<f32>(0.0,  0.1, 0.1)
    );
    var output: Output;
    output.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    output.v_color = vec4<f32>(colors[in_vertex_index],1.0);
    return output;
}

// fragment shader

[[stage(fragment)]]
fn fs_main([[location(0)]] v_color:vec4<f32>) -> [[location(0)]] vec4<f32> {
    return v_color;
}