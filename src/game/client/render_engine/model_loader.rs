mod gltf_loader;
mod obj_loader;

use crate::{
  file_utilities::{file_extension_from_path, file_name_from_path},
  game::client::render_engine::model_loader::{gltf_loader::GLTFLoader, obj_loader::ObjLoader},
};

use super::model::Model;

///
/// Load a model up without having to worry about file extensions.
///
/// If someone wants to add a new file format loader integration
/// for the render engine, they just bolt in a new extension/function here.
///
pub struct ModelLoader {}

impl ModelLoader {
  pub fn load_model(
    path: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
  ) -> Result<Model, String> {
    println!("Hello I am loading hooray!");

    let file_name = match file_name_from_path(path) {
      Ok(file_name) => file_name,
      Err(e) => panic!("ModelLoader: {}", e),
    };

    let extension = match file_extension_from_path(path) {
      Ok(extension) => extension,
      Err(e) => panic!("ModelLoader: {}", e),
    };

    match extension {
      "gltf" => {
        println!("ModelLoader: this is a GLTF model file.");
        Ok(GLTFLoader::load(path, device, queue))
      }
      "obj" => {
        println!("ModelLoader: this is an OBJ model file.");
        Ok(ObjLoader::load(path, device, queue))
      }
      _ => Err(format!(
        "ModelLoader: Failed to load {}. Extension [{}] is not implemented.",
        file_name, extension
      )),
    }

    // println!("ModelLoader: the extension is [{}]", extension);
  }
}
