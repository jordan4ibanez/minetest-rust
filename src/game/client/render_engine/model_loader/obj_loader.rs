use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader, Cursor},
  path::Path,
};

use tobj::{LoadError, MTLLoadResult, Material, Mesh};

use crate::file_utilities::{read_file_to_buf_read, read_file_to_string};

///
/// This struct simply holds the Obj model before we convert it into the
/// internal generic format.
///
struct ObjModel {
  pub meshes: Vec<Mesh>,
  pub materials: Vec<Material>,
}

///
/// The OBJ file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct ObjLoader {}

impl ObjLoader {
  pub fn load(path: &str) {
    println!("Obj loader is working");

    // The buffer we're going to read the model into.
    let mut model_reader = read_file_to_buf_read(path);

    // Model loading options, we just want the basics.
    let model_load_options = tobj::LoadOptions {
      triangulate: true,
      single_index: true,
      ..Default::default()
    };

    let result = tobj::load_obj_buf(
      &mut model_reader,
      &model_load_options,
      ObjLoader::material_loader,
    );

    let (models, obj_materials) = match result {
      Ok(gotten_data) => gotten_data,
      Err(error) => panic!("ObjLoader: {}", error),
    };
  }

  ///
  /// tobj requires a function to execute instructions to load materials, so we do that.
  ///
  fn material_loader(path: &Path) -> MTLLoadResult {
    let material_text = read_file_to_string(path.to_str().unwrap());
    tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(material_text)))
  }
}
