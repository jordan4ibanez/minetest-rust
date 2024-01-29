use glam::Mat4;

///
/// Translation Rotation Scale Projection Data is a pure data container.
///
/// Used for many thing. A zeitgeist of capability in the RenderEngine.
///
/// It stores TRS data in a single 4x4 matrix in raw form.
///
// We need this for Rust to store our data correctly for the shaders.
#[repr(C)]
// This is so we can store this in a buffer.
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TRSProjectionData {
  // We can't use glam with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array.
  pub projection: [[f32; 4]; 4],
}
impl TRSProjectionData {
  pub fn new() -> Self {
    Self {
      projection: Mat4::IDENTITY.to_cols_array_2d(),
    }
  }
}
