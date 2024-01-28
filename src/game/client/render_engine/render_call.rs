use glam::Vec3A;

///
/// A container to handle unbatched draw calls.
///
pub struct RenderCall {
  model_name: String,
  texture_name: String,
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}

impl RenderCall {
  pub fn new(
    model_name: &str,
    texture_name: &str,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) -> Self {
    RenderCall {
      model_name: model_name.to_owned(),
      texture_name: texture_name.to_owned(),
      translation,
      rotation,
      scale,
    }
  }

  pub fn get_model_name(&self) -> &String {
    &self.model_name
  }

  pub fn get_texture_name(&self) -> &String {
    &self.texture_name
  }

  pub fn get_translation(&self) -> &Vec3A {
    &self.translation
  }

  pub fn get_rotation(&self) -> &Vec3A {
    &self.rotation
  }

  pub fn get_scale(&self) -> &Vec3A {
    &self.scale
  }
}