@vertex
fn vertex_main(@location(0) in_pos: vec3<f32>, @location(1) in_col: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.pos = vec4(in_pos, 1.0);
    output.col = in_col;
    return output;
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec3<f32>,
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.col, 1.0);
}
