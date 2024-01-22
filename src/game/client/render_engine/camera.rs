use glam::DVec3;

use crate::game::client::window_handler::WindowHandler;

pub struct Camera {
  position: DVec3,
  target: DVec3,
  up: DVec3,
  aspect_ratio: f64,
  fov_y: f64,
  z_near: f64,
  z_far: f64,
}

impl Camera {
  pub fn new(position: DVec3, fov_y: f64, window: WindowHandler) -> Self {
    Camera {
      position,
      target: DVec3::new(0.0, 0.0, 0.0),
      up: glam::DVec3::Y,
      aspect_ratio: window.get_width() as f64 / window.get_height() as f64,
      fov_y: 45.0,
      z_near: 0.1,
      z_far: 100.0,
    }
  }
}
