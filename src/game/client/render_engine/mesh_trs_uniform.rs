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
}
