use gltf::Gltf;

use crate::{
  file_utilities::{file_name_from_path, read_path_to_buf_read},
  game::client::render_engine::model::Model,
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
    // The file name. This will be used later.
    let file_name = match file_name_from_path(path) {
      Ok(file_name) => file_name,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    // The buffer we're going to read the model into.
    let mut model_reader = match read_path_to_buf_read(path) {
      Ok(model_reader) => model_reader,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    // Now if there was an issue, stop everything.
    // !TODO: Maybe in the future we can just return out a result from this.
    // ! But this is currently being written from scratch at the time of this comment.
    let gltf = match Gltf::from_reader(model_reader) {
      Ok(gltf) => gltf,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    println!("{:#?}", gltf);

    for mesh in gltf.meshes() {
      for primitive in mesh.primitives() {}
    }
  }
}
