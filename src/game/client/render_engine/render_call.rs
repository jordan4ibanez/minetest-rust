use glam::Vec3A;

///
/// A container to handle unbatched draw calls.
///
pub struct MeshRenderCall {
  mesh_id: u64,
  texture_id: u64,
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}

///
/// Unbatched render call struct.
///
/// If you want to batch it, see BatchRenderCall.
///
impl MeshRenderCall {
  pub fn new(
    mesh_id: u64,
    texture_id: u64,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) -> Self {
    MeshRenderCall {
      mesh_id,
      texture_id,
      translation,
      rotation,
      scale,
    }
  }

  ///
  /// Get the MeshRenderCall's Mesh ID.
  ///
  pub fn get_mesh_name(&self) -> u64 {
    self.mesh_id
  }

  ///
  /// Get the Texture ID that will be used for this MeshRenderCall.
  ///
  pub fn get_texture_name(&self) -> u64 {
    self.texture_id
  }

  ///
  /// Get the translation of the MeshRenderCall.
  ///
  pub fn get_translation(&self) -> &Vec3A {
    &self.translation
  }

  ///
  /// Get the rotation of the MeshRenderCall.
  ///
  pub fn get_rotation(&self) -> &Vec3A {
    &self.rotation
  }

  ///
  /// Get the scale of the MeshRenderCall.
  ///
  pub fn get_scale(&self) -> &Vec3A {
    &self.scale
  }
}

///
/// Unbatched render call struct for Model.
///
/// If you want to batch it, see BatchRenderCall.
///
pub struct ModelRenderCall {
  model_name: String,
  texture_names: Vec<String>,
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}
impl ModelRenderCall {
  pub fn new(
    mesh_name: &str,
    texture_names: Vec<String>,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) -> Self {
    ModelRenderCall {
      model_name: mesh_name.to_owned(),
      texture_names,
      translation,
      rotation,
      scale,
    }
  }

  ///
  /// Get the ModelRenderCall's name.
  ///
  pub fn get_model_name(&self) -> &String {
    &self.model_name
  }

  ///
  /// Get the texture that will be used for this ModelRenderCall.
  ///
  pub fn get_texture_names(&self) -> &Vec<String> {
    &self.texture_names
  }

  ///
  /// Get the translation of the ModelRenderCall.
  ///
  pub fn get_translation(&self) -> &Vec3A {
    &self.translation
  }

  ///
  /// Get the rotation of the ModelRenderCall.
  ///
  pub fn get_rotation(&self) -> &Vec3A {
    &self.rotation
  }

  ///
  /// Get the scale of the ModelRenderCall.
  ///
  pub fn get_scale(&self) -> &Vec3A {
    &self.scale
  }
}
