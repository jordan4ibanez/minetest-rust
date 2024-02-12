use crate::game::client::render_engine::model::Model;

///
/// The GLTF file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct GLTFLoader {}

impl GLTFLoader {
  pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Model {}
}
