use log::error;

use crate::file_utilities::file_extension_from_path;

mod obj_loader;

///
/// Load a model up without having to worry about file extensions.
///
/// If someone wants to add a new file format loader integration
/// for the render engine, they just bolt in a new extension/function here.
///
pub struct ModelLoader {}

impl ModelLoader {
  pub fn load_model(path: &str) {
    println!("Hello I am loading hooray!");

    let extension = file_extension_from_path(path);

    match extension {
      "gltf" => {
        println!("ModelLoader: this is a GLTF model file.");
      }
      "obj" => {
        println!("ModelLoader: this is an OBJ model file.");
      }
      _ => {
        error!(
          "ModelLoader: [{}] is not an integrated model format.",
          extension
        );
      }
    }

    // println!("ModelLoader: the extension is [{}]", extension);
  }
}
