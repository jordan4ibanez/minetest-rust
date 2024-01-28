use glam::{Mat4, Quat, Vec3A};

///
/// A BatchRenderCall is a batched [aka instanced] render call optimized to draw
/// many of the same model at once. This is much faster than regular RenderCall when
/// attempting to draw things like items and mobs, so please use it as so.
///
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BatchRaw {
  matrix: [[f32; 4]; 4],
}

impl BatchRaw {
  pub fn new(translation: Vec3A, rotation: Vec3A, scale: Vec3A) -> Self {
    let rotation = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    let matrix = Mat4::from_scale_rotation_translation(scale.into(), rotation, translation.into())
      .to_cols_array_2d();
    BatchRaw { matrix }
  }
}
