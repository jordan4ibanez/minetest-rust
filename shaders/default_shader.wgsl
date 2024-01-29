// Vertex shader

struct CameraUniform {
  view_projection: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct ModelUniform {
  trs_projection: mat4x4<f32>,
}
@group(3) @binding(0)
var<uniform> model_uniform: ModelUniform;

struct InstanceInput {
  @location(5) model_matrix_0: vec4<f32>,
  @location(6) model_matrix_1: vec4<f32>,
  @location(7) model_matrix_2: vec4<f32>,
  @location(8) model_matrix_3: vec4<f32>,
}

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) texture_coordinates: vec2<f32>,
  @location(2) color: vec3<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) texture_coordinates: vec2<f32>,
  @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = model.texture_coordinates;
    out.color = model.color;
    out.clip_position = camera.view_projection * model_uniform.trs_projection * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

struct ColorUniform {
  rgb: vec4<f32>,
}
@group(2) @binding(0)
var<uniform> colorBuffer: ColorUniform;


@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.texture_coordinates) * colorBuffer.rgb * vec4<f32>(in.color, 1.0);
}