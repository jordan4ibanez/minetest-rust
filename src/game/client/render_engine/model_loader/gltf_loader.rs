use wgpu::util::DeviceExt;

use crate::{
  file_utilities::file_name_from_path,
  game::client::render_engine::{
    mesh::{Mesh, Vertex},
    model::Model,
  },
};

///
/// The GLTF file loader.
///
/// This is a wrapper to namespace the functionality as a pseudo struct.
///
pub struct GLTFLoader {}

impl GLTFLoader {
  pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Model {
    // The file name. This will be used later.
    let file_name = match file_name_from_path(path) {
      Ok(file_name) => file_name,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    let minetest_gltf = match minetest_gltf::load(path) {
      Ok(data) => data,
      Err(e) => panic!("GLTFLoader: {}", e),
    };

    // If there are no scenes, give up.
    // We only want scene 0.
    let model = match &minetest_gltf.model {
      Some(model) => model,
      None => panic!(
        "GLTFLoader: {} has no model! Full path: {}",
        file_name, path
      ),
    };

    // Next we load up the raw data.
    let mut meshes: Vec<Mesh> = vec![];

    for (prim_index, primitive) in model.primitives.iter().enumerate() {
      // We have to transmute the
      let mut vertices: Vec<Vertex> = vec![];
      for vertex in primitive.vertices() {
        // These containers are CGMath, converting into GLAM. This should never randomly blow up.
        let new_vertex = Vertex {
          position: vertex.position.into(),
          texture_coordinates: vertex.tex_coords.into(),
          color: [1.0, 1.0, 1.0],
        };

        vertices.push(new_vertex);
      }

      // Now create the buffers.
      let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Vertex Buffer", file_name)),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
      });

      // The GLTF Model might be a bit messed up.
      let indices = match primitive.indices() {
        Some(indices) => indices,
        None => panic!("GLTFLoader: Model [{}] has no indices!", file_name),
      };

      let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Index Buffer", file_name)),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
      });

      // Finally, make a mesh struct from the data! Hooray.
      let new_mesh = Mesh::new_from_existing(
        file_name,
        vertex_buffer,
        index_buffer,
        indices.len() as u32,
        prim_index as u32,
      );

      meshes.push(new_mesh);
    }

    let number_of_texture_buffers = meshes.len() as u32;

    // Some nice debug info.
    println!(
      "GLTFLoader: Model [{}] was created with [{}] texture buffer(s).",
      file_name, number_of_texture_buffers
    );

    let mut animations = None;

    // Animation data
    if minetest_gltf.is_animated() {
      animations = minetest_gltf.bone_animations;
    }

    Model {
      name: file_name.to_owned(),
      meshes,
      number_of_texture_buffers,
      animations,
      lock: false,
    }
  }
}
