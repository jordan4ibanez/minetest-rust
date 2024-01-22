use glam::UVec2;

pub struct MouseController {
  position: UVec2,
  relative_position: UVec2,
}

impl MouseController {
  pub fn new() -> Self {
    MouseController {
      position: UVec2::new(0, 0),
      relative_position: UVec2::new(0, 0),
    }
  }

  ///
  /// Set the Mouse' position.
  ///
  /// * This should only be used in WindowHandler!
  ///
  pub fn set_position(&mut self, x: u32, y: u32) {
    self.position.x = x;
    self.position.y = y;
  }

  ///
  /// Get the Mouse' position.
  ///
  pub fn get_position(&self) -> &UVec2 {
    &self.position
  }

  ///
  /// Set the Mouse' relative position.
  ///
  /// * This should only be used in WindowHandler!
  ///
  pub fn set_relative_position(&mut self, xrel: u32, yrel: u32) {
    self.relative_position.x = xrel;
    self.relative_position.y = yrel;
  }

  ///
  /// Get the Mouse' relative position.
  ///
  pub fn get_relative_position(&self) -> &UVec2 {
    &self.relative_position
  }
}
