mod camera_uniform;

use glam::{Mat4, Vec3, Vec3A, Vec4};
use wgpu::util::DeviceExt;

use crate::game::client::window_handler::WindowHandler;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4 {
  x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
  y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
  z_axis: Vec4::new(0.0, 0.0, 0.5, 0.5),
  w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
};

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array
  view_projection: [[f32; 4]; 4],
}
impl CameraUniform {
  pub fn new() -> Self {
    Self {
      view_projection: Mat4::IDENTITY.to_cols_array_2d(),
    }
  }
}

pub struct Camera {
  eye: Vec3A,
  target: Vec3A,
  rotation: Vec3A,
  up: Vec3A,
  aspect_ratio: f32,
  fov_y: f32,
  z_near: f32,
  z_far: f32,

  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
}

impl Camera {
  pub fn new(
    position: Vec3A,
    fov_y: f32,
    device: &wgpu::Device,
    window_handler: &WindowHandler,
  ) -> Self {
    // First up is the Camera's uniform.
    let camera_uniform = CameraUniform::new();

    // Now we create the Camera's buffer.
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("camera_buffer"),
      contents: bytemuck::cast_slice(&camera_uniform.view_projection),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Then the bind group.
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &Camera::get_wgpu_bind_group_layout(device),
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
      label: Some("camera_bind_group"),
    });

    // Now you have a new camera.
    Camera {
      eye: position,
      target: Vec3A::new(0.0, 0.0, 0.0),
      rotation: Vec3A::new(0.0, 0.0, 0.0),
      up: glam::Vec3A::Y,
      aspect_ratio: window_handler.get_width() as f32 / window_handler.get_height() as f32,
      fov_y: 45.0,
      z_near: 0.1,
      z_far: 100.0,

      camera_uniform,
      camera_buffer,
      camera_bind_group,
    }
  }

  ///
  /// Set the FOV of the Camera.
  ///
  pub fn set_fov(&mut self, new_fov: f32) {
    self.fov_y = new_fov;
  }

  ///
  /// Set the position of the Camera.
  ///
  pub fn set_position(&mut self, new_position: &Vec3A) {
    self.eye.x = new_position.x;
    self.eye.y = new_position.y;
    self.eye.z = new_position.z;
  }

  ///
  /// Get the Camera's position.
  ///
  pub fn get_position(&self) -> &Vec3A {
    &self.eye
  }

  ///
  /// Set the Camera's rotation.
  ///
  pub fn set_rotation(&mut self, new_rotation: &Vec3A) {
    self.rotation.x = new_rotation.x;
    self.rotation.y = new_rotation.y;
    self.rotation.z = new_rotation.z;
  }

  ///
  /// Get the Camera's rotation.
  ///
  pub fn get_rotation(&self) -> &Vec3A {
    &self.rotation
  }

  ///
  /// Rebuild the projection matrix.
  ///
  /// Passes back a new view projection matrix.
  ///
  /// This also updates the aspect ratio, so WindowHandler is required.
  ///
  /// On top of this, it will also update the wgpu matrix uniform automatically.
  /// So the queue is required.
  ///
  pub fn build_view_projection_matrix(
    &mut self,
    device: &wgpu::Device,
    window_handler: &WindowHandler,
    queue: &wgpu::Queue,
  ) {
    self.aspect_ratio = window_handler.get_width() as f32 / window_handler.get_height() as f32;

    let view_rot = Mat4::from_euler(
      glam::EulerRot::XYZ,
      self.rotation.x,
      self.rotation.y,
      self.rotation.z,
    );

    let view_translation = Mat4::from_translation(Vec3::from(self.eye));

    // let view = Mat4::look_at_rh(self.eye.into(), self.target.into(), self.up.into());

    let projection = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far);

    self.camera_uniform.view_projection =
      (OPENGL_TO_WGPU_MATRIX * projection * view_rot * view_translation).to_cols_array_2d();

    // Automatically writes the camera's matrix information in wgpu.
    queue.write_buffer(self.get_buffer(), 0, self.get_wgpu_raw_matrix());
  }

  ///
  /// Get the wgpu raw uniform contents to pass into the pipelne.
  ///
  pub fn get_wgpu_raw_matrix(&self) -> &[u8] {
    bytemuck::cast_slice(&self.camera_uniform.view_projection)
  }

  ///
  /// Get the Camera's wgpu bind group for rendering.
  ///
  pub fn get_bind_group(&self) -> &wgpu::BindGroup {
    &self.camera_bind_group
  }

  ///
  /// Get the Camera's wgpu buffer.
  pub fn get_buffer(&self) -> &wgpu::Buffer {
    &self.camera_buffer
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
      label: Some("camera_bind_group_layout"),
    })
  }
}
