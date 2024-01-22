use glam::{Mat4, Vec3A, Vec4};

use crate::game::client::window_handler::WindowHandler;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4 {
  x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
  y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
  z_axis: Vec4::new(0.0, 0.0, 0.5, 0.5),
  w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
};

pub struct Camera {
  eye: Vec3A,
  target: Vec3A,
  up: Vec3A,
  aspect_ratio: f32,
  fov_y: f32,
  z_near: f32,
  z_far: f32,
}

impl Camera {
  pub fn new(position: Vec3A, fov_y: f32, window_handler: &WindowHandler) -> Self {
    Camera {
      eye: position,
      target: Vec3A::new(0.0, 0.0, 0.0),
      up: glam::Vec3A::Y,
      aspect_ratio: window_handler.get_width() as f32 / window_handler.get_height() as f32,
      fov_y: 45.0,
      z_near: 0.1,
      z_far: 100.0,
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
    self.eye = *new_position;
  }

  ///
  /// Rebuild the projection matrix.
  ///
  /// Passes back a new view projection matrix.
  ///
  /// This also updates the aspect ratio, so WindowHandler is required.
  ///
  pub fn build_view_projection_matrix(&mut self, window_handler: &WindowHandler) -> Mat4 {
    self.aspect_ratio = window_handler.get_width() as f32 / window_handler.get_height() as f32;

    let view = Mat4::look_at_rh(self.eye.into(), self.target.into(), self.up.into());

    let projection = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far);

    OPENGL_TO_WGPU_MATRIX * projection * view
  }
}
