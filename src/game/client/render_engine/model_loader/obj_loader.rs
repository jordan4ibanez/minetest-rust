use std::path::Path;

use ahash::AHashMap;
use tobj::{LoadError, MTLLoadResult, Material, Mesh};

use crate::file_utilities::{read_file_to_buf_read, read_file_to_string_result};

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

    // We want to know if there's an issue loading the obj file.
    // Let it stream a result in.
    let result = tobj::load_obj_buf(
      &mut model_reader,
      &model_load_options,
      ObjLoader::material_loader,
    );

    // Now if there was an issue, stop everything.
    // !TODO: Maybe in the future we can just return out a result from this.
    // ! But this is currently being written from scratch at the time of this comment.
    let (models, object_materials) = match result {
      Ok(gotten_data) => gotten_data,
      Err(error) => panic!("ObjLoader: {}", error),
    };

    for material in object_materials.unwrap() {
      let x = material.diffuse_texture.unwrap();
      println!("the test is: {}", x);
    }
  }

  ///
  /// tobj requires a function to execute instructions to load materials, so we do that.
  ///
  /// And by do that, I mean, do absolutely nothing but allocate to satisfy the return type.
  ///
  fn material_loader(path: &Path) -> MTLLoadResult {
    Ok((vec![], AHashMap::new()))
  }
}
