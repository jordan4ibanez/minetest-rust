use glam::UVec2;

pub struct Mouse {
  position: UVec2,
}

impl Mouse {
  pub fn new() -> Self {
    Mouse {
      position: UVec2::new(0, 0),
    }
  }

  ///
  /// Set the Mouse' position.
  ///
  /// * This should only be used in window_handler!
  ///
  pub fn set_position(&mut self, new_position: UVec2) {
    self.position = new_position;
  }

  ///
  /// Get the Mouse' position.
  ///
  pub fn get_position(&self) -> &UVec2 {
    &self.position
  }
}
