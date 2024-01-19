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
