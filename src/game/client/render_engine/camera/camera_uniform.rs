use glam::Mat4;

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

  ///
  /// Updates the raw view projection matrix from the camera.
  ///
  pub fn update_view_projection(&mut self, new_matrix: Mat4) {
    self.view_projection = new_matrix.to_cols_array_2d();
  }

  pub fn get_view_projection(&self) -> &[[f32; 4]; 4] {
    &self.view_projection
  }
}
