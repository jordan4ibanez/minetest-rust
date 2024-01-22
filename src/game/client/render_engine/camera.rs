use glam::{DMat4, DVec3, Mat4, Vec3A};

use crate::game::client::window_handler::WindowHandler;

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
  pub fn new(position: Vec3A, fov_y: f32, window: WindowHandler) -> Self {
    Camera {
      eye: position,
      target: Vec3A::new(0.0, 0.0, 0.0),
      up: glam::Vec3A::Y,
      aspect_ratio: window.get_width() as f32 / window.get_height() as f32,
      fov_y: 45.0,
      z_near: 0.1,
      z_far: 100.0,
    }
  }

  pub fn build_view_projection_matrix(&self) -> Mat4 {
    let x = f32::MAX;

    let view = Mat4::look_at_rh(self.eye.into(), self.target.into(), self.up.into());
    
  }
}
