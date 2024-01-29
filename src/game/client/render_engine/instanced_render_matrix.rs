use std::mem::size_of;

use glam::{Mat4, Quat, Vec3A};

///
/// A InstancedRenderUniform is an instanced render call optimized to draw
/// many of the same model at once. This is much faster than regular RenderCall when
/// attempting to draw things like items and mobs, so please use it as so.
///
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstancedRenderData {
  matrix: [[f32; 4]; 4],
}

impl InstancedRenderData {
  pub fn new(translation: Vec3A, rotation: Vec3A, scale: Vec3A) -> Self {
    let rotation = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    let matrix = Mat4::from_scale_rotation_translation(scale.into(), rotation, translation.into())
      .to_cols_array_2d();
    InstancedRenderData { matrix }
  }

  pub fn get_wgpu_descriptor() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: size_of::<InstancedRenderData>() as wgpu::BufferAddress,
      // We need to switch from using a step mode of Vertex to Instance
      // This means that our shaders will only change to use the next
      // instance when the shader starts processing a new instance
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
        // for each vec4. We'll have to reassemble the mat4 in the shader.
        wgpu::VertexAttribute {
          offset: 0,
          // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
          // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
      ],
    }
  }

  pub fn get_blank_data() -> Vec<f32> {
    vec![
      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ]
  }
}