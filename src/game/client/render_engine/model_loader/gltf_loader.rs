use crate::{
  file_utilities::{file_name_from_path, read_path_to_buf_read},
  game::client::render_engine::mesh::Mesh,
};
///
/// The GLTF file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct GLTFLoader {}

impl GLTFLoader {
  pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) /*-> Model*/
  {
    let scenes = match easy_gltf::load(path) {
      Ok(data) => data,
      Err(e) => panic!("GLTFLoader: {}", e),
    };
  }
}
