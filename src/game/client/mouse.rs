use glam::UVec2;

pub struct MouseController {
  position: UVec2,
  relative_position: UVec2,
  relative_mode: bool,
  sensitivity: f32,
}

impl MouseController {
  pub fn new() -> Self {
    MouseController {
      position: UVec2::new(0, 0),
      relative_position: UVec2::new(0, 0),
      relative_mode: false,
      sensitivity: 0.001,
    }
  }

  ///
  /// Toggle Mouse' relative mode.
  ///
  /// * This should only be used in WindowHandler!
  ///
  pub fn toggle_relative_mode(&mut self) {
    self.relative_mode = !self.relative_mode
  }

  ///
  /// Get if the Mouse is in relative mode.
  ///
  pub fn is_relative_mode(&self) -> bool {
    self.relative_mode
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
  /// Get the X position of the Mouse.
  ///
  pub fn get_x(&self) -> u32 {
    self.position.x
  }

  ///
  /// Get the Y position of the Mouse.
  ///
  pub fn get_y(&self) -> u32 {
    self.position.y
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

  ///
  /// Set the Mouse' sensitivity.
  ///
  pub fn set_sensitivity(&mut self, new_sensitivity: f32) {
    self.sensitivity = new_sensitivity;
  }

  ///
  /// Get the Mouse' sensitivity.
  ///
  pub fn get_sensitivity(&self) -> f32 {
    self.sensitivity
  }
}
