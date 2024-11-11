struct Vertex {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@group(0) @binding(0)
var<uniform> projection: mat4x4f;

@vertex 
fn vs_main(
    @location(0) pos: vec2f,  // xy
    @location(1) color: vec3f,  // rgb
) -> Vertex {
    var output: Vertex;

    output.position = projection * vec4f(pos, 0.0, 1.0);
    output.color = vec4f(color, 1.0);

    return output;
}

@fragment
fn fs_main(in: Vertex) -> @location(0) vec4f {
    return in.color;
}