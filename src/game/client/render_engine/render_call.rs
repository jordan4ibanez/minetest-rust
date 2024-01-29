use glam::Vec3A;

///
/// A container to handle unbatched draw calls.
///
pub struct RenderCall {
  mesh_name: String,
  texture_name: String,
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}

///
/// Unbatched render call struct.
///
/// If you want to batch it, see BatchRenderCall.
///
impl RenderCall {
  pub fn new(
    mesh_name: &str,
    texture_name: &str,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) -> Self {
    RenderCall {
      mesh_name: mesh_name.to_owned(),
      texture_name: texture_name.to_owned(),
      translation,
      rotation,
      scale,
    }
  }

  ///
  /// Get the RenderCall's mesh name.
  ///
  pub fn get_mesh_name(&self) -> &String {
    &self.mesh_name
  }

  ///
  /// Get the texture that will be used for this RenderCall.
  ///
  pub fn get_texture_name(&self) -> &String {
    &self.texture_name
  }

  ///
  /// Get the translation of the RenderCall.
  ///
  pub fn get_translation(&self) -> &Vec3A {
    &self.translation
  }

  ///
  /// Get the rotation of the RenderCall.
  ///
  pub fn get_rotation(&self) -> &Vec3A {
    &self.rotation
  }

  ///
  /// Get the scale of the RenderCall.
  ///
  pub fn get_scale(&self) -> &Vec3A {
    &self.scale
  }
}
