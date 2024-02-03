mod obj_loader;

use log::error;

use crate::{
  file_utilities::{file_extension_from_path, file_name_from_path},
  game::client::render_engine::model_loader::obj_loader::ObjLoader,
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
  pub fn load_model(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Model> {
    println!("Hello I am loading hooray!");

    let file_name = file_name_from_path(path);
    let extension = file_extension_from_path(path);

    match extension {
      "gltf" => {
        println!("ModelLoader: this is a GLTF model file.");
        None
      }
      "obj" => {
        println!("ModelLoader: this is an OBJ model file.");
        Some(ObjLoader::load(path, device, queue))
      }
      _ => {
        error!(
          "ModelLoader: error loading [{}]. [{}] is not an integrated model format.",
          file_name, extension
        );
        None
      }
    }

    // println!("ModelLoader: the extension is [{}]", extension);
  }
}
