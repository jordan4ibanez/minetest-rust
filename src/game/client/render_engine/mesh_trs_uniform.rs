use glam::{Mat4, Vec3A, Vec4};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4 {
  x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
  y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
  z_axis: Vec4::new(0.0, 0.0, 0.5, 0.5),
  w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
};

pub struct MeshTRSUniform {
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,

  model_projection: [[f32; 4]; 4],
}

impl MeshTRSUniform {
  pub fn new() -> Self {
    MeshTRSUniform {
      translation: Vec3A::new(0.0, 0.0, 0.0),
      rotation: Vec3A::new(0.0, 0.0, 0.0),
      scale: Vec3A::new(0.0, 0.0, 0.0),
      model_projection: Mat4::IDENTITY.to_cols_array_2d(),
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
