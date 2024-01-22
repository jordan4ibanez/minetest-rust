use glam::Mat4;

use crate::game::client::window_handler::WindowHandler;

use super::Camera;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]

struct CameraUniform {
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array
  view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  fn new() -> Self {
    Self {
      view_proj: Mat4::IDENTITY.to_cols_array_2d(),
    }
  }

  fn update_view_proj(&mut self, camera: &mut Camera, window_handler: &WindowHandler) {
    self.view_proj = camera
      .build_view_projection_matrix(window_handler)
      .to_cols_array_2d();
  }
}
