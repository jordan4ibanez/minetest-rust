use glam::DVec3;

///
/// A container to handle unbatched draw calls.
///
pub struct RenderCall {
  model_name: String,
  translation: DVec3,
  rotation: DVec3,
  scale: DVec3,
}

impl RenderCall {
  pub fn new(model_name: &str, translation: DVec3, rotation: DVec3, scale: DVec3) -> Self {
    RenderCall {
      model_name: model_name.to_owned(),
      translation,
      rotation,
      scale,
    }
  }

  pub fn get_model_name(&self) -> &String {
    &self.model_name
  }

  pub fn get_translation(&self) -> &DVec3 {
    &self.translation
  }

  pub fn get_rotation(&self) -> &DVec3 {
    &self.rotation
  }

  pub fn get_scale(&self) -> &DVec3 {
    &self.scale
  }
}
