use glam::{IVec2, UVec2};

pub struct MouseController {
  position: IVec2,
  relative_position: IVec2,
  relative_mode: bool,
  sensitivity: f32,
}

impl MouseController {
  pub fn new() -> Self {
    MouseController {
      position: IVec2::new(0, 0),
      relative_position: IVec2::new(0, 0),
      relative_mode: false,
      sensitivity: 0.001,
    }
  }

  /// ! This should only be run by the Client!
  ///
  /// Resets the Mouse' relative position for Camera controls.
  ///
  pub fn reset_mouse_relative_position(&mut self) {
    self.relative_position.x = 0;
    self.relative_position.y = 0;
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
  pub fn set_position(&mut self, x: i32, y: i32) {
    self.position.x = x;
    self.position.y = y;
  }

  ///
  /// Get the Mouse' position.
  ///
  pub fn get_position(&self) -> &IVec2 {
    &self.position
  }

  ///
  /// Get the X position of the Mouse.
  ///
  pub fn get_x(&self) -> i32 {
    self.position.x
  }

  ///
  /// Get the Y position of the Mouse.
  ///
  pub fn get_y(&self) -> i32 {
    self.position.y
  }

  ///
  /// Set the Mouse' relative position.
  ///
  /// * This should only be used in WindowHandler!
  ///
  pub fn set_relative_position(&mut self, xrel: i32, yrel: i32) {
    println!("{}", self.relative_position);
    self.relative_position.x = xrel;
    self.relative_position.y = yrel;
  }

  ///
  /// Get the Mouse' relative position.
  ///
  pub fn get_relative_position(&self) -> &IVec2 {
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
