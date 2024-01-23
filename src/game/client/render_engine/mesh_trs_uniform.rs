use glam::{Mat4, Vec3, Vec3A, Vec4};
use wgpu::util::DeviceExt;

use crate::game::client::window_handler::WindowHandler;

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4 {
  x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
  y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
  z_axis: Vec4::new(0.0, 0.0, 0.5, 0.5),
  w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
};

// We need this for Rust to store our data correctly for the shaders.
#[repr(C)]
// This is so we can store this in a buffer.
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ModelUniform {
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array.
  model_projection: [[f32; 4]; 4],
}
impl ModelUniform {
  pub fn new() -> Self {
    Self {
      model_projection: Mat4::IDENTITY.to_cols_array_2d(),
    }
  }
}

pub struct MeshTRSUniform {
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,

  model_uniform: ModelUniform,

  // wgpu components.
  model_buffer: wgpu::Buffer,
  model_bind_group: wgpu::BindGroup,
}

impl MeshTRSUniform {
  pub fn new(device: &wgpu::Device) -> Self {
    // First up is the MeshTRSUniform's uniform.
    let model_uniform = ModelUniform::new();

    // Now we create the MeshTRSUniform's buffer.
    let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("model_trs_buffer"),
      contents: bytemuck::cast_slice(&model_uniform.model_projection),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Then the bind group.
    let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &MeshTRSUniform::get_wgpu_bind_group_layout(device),
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: model_buffer.as_entire_binding(),
      }],
      label: Some("model_trs_bind_group"),
    });

    MeshTRSUniform {
      translation: Vec3A::new(0.0, 0.0, 0.0),
      rotation: Vec3A::new(0.0, 0.0, 0.0),
      scale: Vec3A::new(0.0, 0.0, 0.0),

      model_uniform,

      // wgpu components.
      model_buffer,
      model_bind_group,
    }
  }

  ///
  /// Set the translation of the Mesh TRS Uniform.
  ///
  pub fn set_translation(&mut self, new_translation: &Vec3A) {
    self.translation.x = new_translation.x;
    self.translation.y = new_translation.y;
    self.translation.z = new_translation.z;
  }

  ///
  /// Set the rotation of the Mesh TRS Uniform.
  ///
  pub fn set_rotation(&mut self, new_rotation: &Vec3A) {
    self.rotation.x = new_rotation.x;
    self.rotation.y = new_rotation.y;
    self.rotation.z = new_rotation.z;
  }

  ///
  /// Set the scale of the Mesh TRS Uniform.
  ///
  pub fn set_scale(&mut self, new_scale: &Vec3A) {
    self.scale.x = new_scale.x;
    self.scale.y = new_scale.y;
    self.scale.z = new_scale.z;
  }

  ///
  /// The TRS model projection will automatically rebuild itself every time it is polled.
  ///
  fn build_model_projection_matrix(
    &mut self,
    device: &wgpu::Device,
    window_handler: &WindowHandler,
    queue: &wgpu::Queue,
  ) {
    let view_rotation = Mat4::from_euler(
      glam::EulerRot::XYZ,
      self.rotation.x,
      self.rotation.y,
      self.rotation.z,
    );

    let view_translation = Mat4::from_translation(Vec3::from(self.translation));

    self.model_uniform.model_projection =
      (OPENGL_TO_WGPU_MATRIX * view_rotation * view_translation).to_cols_array_2d();

    queue.write_buffer(self.get_buffer(), 0, self.get_wgpu_raw_matrix());
  }

  ///
  /// Get the wgpu raw uniform contents to pass into the pipeline.
  ///
  fn get_wgpu_raw_matrix(&self) -> &[u8] {
    bytemuck::cast_slice(&self.model_uniform.model_projection)
  }

  ///
  /// Get the MeshTRSUniform's wgpu bind group for rendering.
  ///
  pub fn get_bind_group(&self) -> &wgpu::BindGroup {
    &self.model_bind_group
  }

  ///
  /// Get the MeshTRSUniform's wgpu buffer.
  pub fn get_buffer(&self) -> &wgpu::Buffer {
    &self.model_buffer
  }

  ///
  /// Get the wgpu bind group layout to tell wgpu how to use the buffer.
  ///
  pub fn get_wgpu_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
      label: Some("model_trs_bind_group_layout"),
    })
  }
}
