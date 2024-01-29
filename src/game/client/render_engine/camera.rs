use glam::{Mat4, Vec3, Vec3A};

use wgpu::util::DeviceExt;

use crate::game::client::window_handler::WindowHandler;

use super::trs_projection_data::{TRSProjectionData, OPENGL_TO_WGPU_MATRIX};

pub struct Camera {
  eye: Vec3A,
  target: Vec3A,
  rotation: Vec3A,
  up: Vec3A,
  aspect_ratio: f32,
  fov_y: f32,
  z_near: f32,
  z_far: f32,

  // wgpu raw data.
  camera_uniform: TRSProjectionData,

  // wgpu components.
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
}

impl Camera {
  pub fn new(
    position: Vec3A,
    fov_y: f32,
    device: &wgpu::Device,
    window_handler: &WindowHandler,
    mesh_buffer: &wgpu::Buffer,
  ) -> Self {
    // First up is the Camera's uniform.
    let camera_uniform = TRSProjectionData::new();

    // Now we create the Camera's buffer.
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("camera_buffer"),
      contents: bytemuck::cast_slice(&camera_uniform.projection),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Then the bind group.
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &Camera::get_wgpu_bind_group_layout(device),
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: camera_buffer.as_entire_binding(),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: mesh_buffer.as_entire_binding(),
        },
      ],
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

      // wgpu raw data.
      camera_uniform,

      // wgpu components.
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
    self.eye = new_position.to_owned();
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
    self.rotation = new_rotation.to_owned();
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

    let view_rotation = Mat4::from_euler(
      glam::EulerRot::XYZ,
      self.rotation.x,
      self.rotation.y,
      self.rotation.z,
    );

    let view_translation = Mat4::from_translation(Vec3::from(self.eye));

    // let view = Mat4::look_at_rh(self.eye.into(), self.target.into(), self.up.into());

    let projection = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far);

    self.camera_uniform.projection =
      (OPENGL_TO_WGPU_MATRIX * projection * view_rotation * view_translation).to_cols_array_2d();

    // Automatically write the data into the queue.
    queue.write_buffer(self.get_buffer(), 0, self.get_wgpu_raw_matrix());
  }

  ///
  /// Get the wgpu raw uniform contents to pass into the pipelne.
  ///
  pub fn get_wgpu_raw_matrix(&self) -> &[u8] {
    bytemuck::cast_slice(&self.camera_uniform.projection)
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
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::VERTEX,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
      ],
      label: Some("camera_bind_group_layout"),
    })
  }
}
