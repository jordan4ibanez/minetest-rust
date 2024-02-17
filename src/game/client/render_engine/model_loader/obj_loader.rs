use std::path::Path;

use ahash::AHashMap;
use tobj::MTLLoadResult;
use wgpu::util::DeviceExt;

use crate::{
  file_utilities::{file_name_from_path, read_path_to_buf_read},
  game::client::render_engine::{
    mesh::{Mesh, Vertex},
    model::Model,
  },
};

///
/// The OBJ file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct ObjLoader {}

impl ObjLoader {
  pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Model {
    // The file name. This will be used later.
    let file_name = match file_name_from_path(path) {
      Ok(file_name) => file_name,
      Err(e) => panic!("ObjLoader: {}", e),
    };

    // The buffer we're going to read the model into.
    let mut model_reader = match read_path_to_buf_read(path) {
      Ok(model_reader) => model_reader,
      Err(e) => panic!("ObjLoader: {}", e),
    };

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
      ObjLoader::fake_material_loader,
    );

    // Now if there was an issue, stop everything.
    // !TODO: Maybe in the future we can just return out a result from this.
    // ! But this is currently being written from scratch at the time of this comment.
    let (raw_models, _) = match result {
      Ok(gotten_data) => gotten_data,
      Err(error) => panic!("ObjLoader: {}", error),
    };

    // Next we load up the raw data.
    let mut meshes: Vec<Mesh> = vec![];

    for (model_index, model) in raw_models.iter().enumerate() {
      // Push all vertex data into a vector.
      let mut vertices = vec![];

      for index in 0..model.mesh.positions.len() / 3 {
        let new_vertex = Vertex {
          position: [
            model.mesh.positions[index * 3],
            model.mesh.positions[index * 3 + 1],
            model.mesh.positions[index * 3 + 2],
          ],
          texture_coordinates: [
            model.mesh.texcoords[index * 2],
            // This flips the texture coordinates right side up.
            1.0 - model.mesh.texcoords[index * 2 + 1],
          ],

          color: [1.0, 1.0, 1.0],
        };

        // ? note: meshes can also have normals from obj models.

        vertices.push(new_vertex);
      }

      // Now create the buffers.
      let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Vertex Buffer", file_name)),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
      });
      let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Index Buffer", file_name)),
        contents: bytemuck::cast_slice(&model.mesh.indices),
        usage: wgpu::BufferUsages::INDEX,
      });

      // Finally, make a mesh struct from the data! Hooray.
      let new_mesh = Mesh::new_from_existing(
        file_name,
        vertex_buffer,
        index_buffer,
        model.mesh.indices.len() as u32,
        model_index as u32,
      );

      meshes.push(new_mesh);
    }

    let number_of_texture_buffers = meshes.len() as u32;

    // Some nice debug info.
    println!(
      "ObjLoader: Model [{}] was created with [{}] texture buffer(s).",
      file_name, number_of_texture_buffers
    );

    Model {
      name: file_name.to_owned(),
      meshes,
      number_of_texture_buffers,
      animations: None,
      lock: false,
    }
  }

  ///
  /// tobj requires a function to execute instructions to load materials, so we do that.
  ///
  /// And by do that, I mean, do absolutely nothing but allocate to satisfy the return type.
  ///
  /// This will go out of scope and be freed pretty much immediately.
  ///
  fn fake_material_loader(path: &Path) -> MTLLoadResult {
    Ok((vec![], AHashMap::new()))
  }
}
