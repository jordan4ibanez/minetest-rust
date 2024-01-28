use glam::{Mat4, Quat, Vec3A};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
  matrix: [[f32; 4]; 4],
}

pub struct InstanceRenderCall {
  translation: Vec3A,
  rotation: Vec3A,
  scale: Vec3A,
}

impl InstanceRenderCall {
  pub fn new(translation: Vec3A, rotation: Vec3A, scale: Vec3A) -> Self {
    InstanceRenderCall {
      translation,
      rotation,
      scale,
    }
  }

  pub fn as_instance_raw(&self) -> InstanceRaw {
    let rotation = Quat::from_euler(
      glam::EulerRot::XYZ,
      self.rotation.x,
      self.rotation.y,
      self.rotation.z,
    );
    let matrix =
      Mat4::from_scale_rotation_translation(self.scale.into(), rotation, self.translation.into())
        .to_cols_array_2d();
    InstanceRaw { matrix }
  }
}
