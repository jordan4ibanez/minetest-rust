use glam::Vec3A;

pub struct MeshTRSUniform {
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}

impl MeshTRSUniform {
  pub fn new() -> Self {
    MeshTRSUniform {
      translation: Vec3A::new(0.0, 0.0, 0.0),
      rotation: Vec3A::new(0.0, 0.0, 0.0),
      scale: Vec3A::new(0.0, 0.0, 0.0),
    }
  }

  ///
  /// Set the translation of the Mesh TRS Uniform.
  ///
  pub fn set_translation(&mut self, new_translation: &Vec3A) {
    self.translation.x = new_translation.x;
    self.translation.y = new_translation.y;
    self.translation.z = new_translation.z;
  }

  ///
  /// Set the rotation of the Mesh TRS Uniform.
  ///
  pub fn set_rotation(&mut self, new_rotation: &Vec3A) {
    self.rotation.x = new_rotation.x;
    self.rotation.y = new_rotation.y;
    self.rotation.z = new_rotation.z;
  }

  ///
  /// Set the scale of the Mesh TRS Uniform.
  ///
  pub fn set_scale(&mut self, new_scale: &Vec3A) {
    self.scale.x = new_scale.x;
    self.scale.y = new_scale.y;
    self.scale.z = new_scale.z;
  }
}
