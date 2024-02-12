use crate::file_utilities::file_name_from_path;

///
/// The GLTF file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct GLTFLoader {}

impl GLTFLoader {
  pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) /*-> Model*/
  {
    // The file name. This will be used later.
    let file_name = match file_name_from_path(path) {
      Ok(file_name) => file_name,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    let scenes = match easy_gltf::load(path) {
      Ok(data) => data,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    if scenes.is_empty() {
      panic!("GLTFLoader: {} is a blank model!", file_name)
    }
  }
}
