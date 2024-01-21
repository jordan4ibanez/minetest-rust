use std::mem;
use wgpu::util::DeviceExt;

///
/// The root sizes of the Vertex components.
///
/// X,Y,Z
///
/// R,G,B
///
/// etc
///
const POSITION_COMPONENTS: usize = 3;
const COLOR_COMPONENTS: usize = 3;

///
/// The base of the Mesh.
///
/// Meshes are constructed out of an array of Vertex data.
///
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  pub position: [f32; POSITION_COMPONENTS],
  pub color: [f32; COLOR_COMPONENTS],
}

impl Vertex {
  pub fn new(position: [f32; POSITION_COMPONENTS], color: [f32; COLOR_COMPONENTS]) -> Self {
    Vertex { position, color }
  }
}

///
/// A Mesh is the container that holds the data which makes up a model.
///
#[derive(Debug)]
pub struct Mesh {
  name: String,
  vertex_data: Vec<Vertex>,
  index_data: Vec<u32>,
  vertex_buffer: Option<wgpu::Buffer>,
}

impl Mesh {
  pub fn new(name: &str) -> Self {
    Mesh {
      name: name.to_owned(),
      vertex_data: vec![],
      index_data: vec![],
      vertex_buffer: None,
    }
  }

  ///
  /// Automatically generates the required wgpu data buffers and makes it part of the Mesh.
  ///
  /// Consider this a "finalize" of the Mesh.
  ///
  pub fn generate_wgpu_buffers(&mut self, device: &mut wgpu::Device) {
    // Final check of the data.
    if self.vertex_data.is_empty() {
      panic!("Mesh: attempted to generate wgpu buffers with no vertex data.");
    }

    if self.index_data.is_empty() {
      panic!("Mesh: attempted to generate wgpu buffers with no index data.");
    }

    // Now, it turns into wgpu data.

    let mut vertex_name = self.name.clone();
    vertex_name.push_str("_vertex");

    self.vertex_buffer = Some(
      device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&vertex_name),
        contents: self.get_wgpu_vertex_data(),
        usage: wgpu::BufferUsages::VERTEX,
      }),
    );

    let mut index_name = self.name.clone();
    index_name.push_str("_index");

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some(&index_name),
      contents: self.get_wgpu_index_data(),
      usage: wgpu::BufferUsages::INDEX,
    });
  }

  ///
  /// Push raw vertex data into the Mesh.
  ///
  pub fn push_vertex(&mut self, vertex: Vertex) {
    self.vertex_data.push(vertex);
  }

  ///
  /// Push a Vector of raw vertex data into the Mesh.
  ///
  pub fn push_vertex_array(&mut self, vertex_vec: &mut Vec<Vertex>) {
    self.vertex_data.append(vertex_vec);
  }

  ///
  /// Push raw index data into the Mesh.
  ///
  pub fn push_index(&mut self, index: u32) {
    self.index_data.push(index);
  }

  ///
  /// Push a vector of raw vertex data into the Mesh.
  ///
  pub fn push_index_array(&mut self, index_array: &mut Vec<u32>) {
    self.index_data.append(index_array);
  }

  ///
  /// Grab the wgpu buffer for rendering.
  ///
  pub fn get_wgpu_buffer(&self) -> &wgpu::Buffer {
    self.vertex_buffer.as_ref().unwrap_or_else(|| {
      panic!(
        "Mesh: Buffer was never attached for {} in RenderEngine.",
        self.name
      )
    })
  }

  ///
  /// Grab the raw vertex data from the mesh to create the GPU memory.
  ///
  pub fn as_raw_vertex_array(&self) -> &[Vertex] {
    &self.vertex_data
  }

  ///
  /// Grab the raw vertex data from the mesh to pass to wgpu.
  ///
  fn get_wgpu_vertex_data(&self) -> &[u8] {
    bytemuck::cast_slice(self.vertex_data.as_slice())
  }

  ///
  /// Grab the raw index data from the mesh to pass to wgpu.
  ///
  fn get_wgpu_index_data(&self) -> &[u8] {
    bytemuck::cast_slice(self.index_data.as_slice())
  }

  ///
  /// Get the layout descriptor of Vertex for wgpu.
  ///
  pub fn get_wgpu_descriptor() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        // If we need to add new components, we do it here. Hooray!
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x3,
        },
      ],
    }
  }
}

///
/// !This is a highly experimental function. This might get replaced with something
///
/// Generate an array of Vertex data from raw lists.
///
/// todo: Instead of a () this needs to return the Mesh container when it's made on Ok(())!
///
/// This is primarily aimed at procedurally generated meshes, like map visual data.
///
pub fn generate_mesh(positions: &Vec<f32>, colors: &Vec<f32>) -> Result<Mesh, String> {
  // We want to check all the data to ensure the logic is sound.

  // First, check positions sizing.
  if positions.is_empty() {
    return Err("generate_mesh: sent a blank positions vector!".to_string());
  }
  if positions.len() % POSITION_COMPONENTS != 0 {
    return Err("generate_mesh: sent a wrongly sized positions vector!".to_string());
  }

  // Then check colors sizing.
  if colors.is_empty() {
    return Err("generate_mesh: sent a blank colors vector!".to_string());
  }
  if colors.len() % COLOR_COMPONENTS != 0 {
    return Err("generate_mesh: sent a wrongly sized colors vector!".to_string());
  }

  // Now we need to ensure that these are equally sized.
  let positions_components = positions.len() / POSITION_COMPONENTS;
  let colors_components = colors.len() / COLOR_COMPONENTS;

  if positions_components != colors_components {
    return Err(format!(
      "generate_mesh: sent uneven mesh data! positions: {} | colors: {}",
      positions_components, colors_components
    ));
  }

  //todo: here we will iterate through the data with a mutable vector then dump it into a format. The format needs to be made.

  // ! this is just a test, there is probably a much better way to to this!
  // ! What you're seeing is a raw prototype.

  // Can use one range iterator, they are all supposed to be equal.
  for i in 0..positions_components {
    // todo Instead of unwrapping this in the future, we should match.

    let position_base_offset = i * POSITION_COMPONENTS;

    let position_slice: [f32; 3] = positions
      [position_base_offset..position_base_offset + POSITION_COMPONENTS]
      .try_into()
      .unwrap();

    let color_base_offset = i * COLOR_COMPONENTS;

    let color_slice: [f32; 3] = colors[color_base_offset..color_base_offset + COLOR_COMPONENTS]
      .try_into()
      .unwrap();

    mesh.push_vertex(Vertex {
      position: position_slice,
      color: color_slice,
    });
  }

  Ok(mesh)
}

#[cfg(test)]
mod tests {
  use crate::game::client::render_engine::mesh::generate_mesh;

  #[test]
  fn test_procedural_mesh_creation() {
    println!("--- BEGIN PROCEDURAL MESH TEST ---");
    {
      let positions = vec![1.0, 2.0, 3.0];
      let colors = vec![3.0, 4.0, 5.0];
      let test_mesh = generate_mesh(&positions, &colors);
      assert!(test_mesh.is_ok());
      println!("{:?}", test_mesh.unwrap());
    }

    {
      let positions = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
      let colors = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
      let test_mesh = generate_mesh(&positions, &colors);
      assert!(test_mesh.is_ok());
      println!("{:?}", test_mesh.unwrap());
    }
  }

  #[test]
  fn test_procedural_mesh_creation_failure() {
    println!("--- BEGIN PROCEDURAL MESH FAILURE TEST ---");

    // Missing components.
    {
      let positions = vec![];
      let colors = vec![3.0, 4.0, 5.0];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }
    {
      let positions = vec![1.0, 2.0, 3.0];
      let colors = vec![];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }

    // Wrong size.
    {
      let positions = vec![1.0, 2.0, 3.0, 4.0];
      let colors = vec![4.0, 5.0, 6.0];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }
    {
      let positions = vec![1.0, 2.0, 3.0];
      let colors = vec![4.0, 5.0, 6.0, 7.0];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }

    // Unequal size.
    {
      let positions = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
      let colors = vec![4.0, 5.0, 6.0];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }
    {
      let positions = vec![1.0, 2.0, 3.0];
      let colors = vec![4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
      let failed_result = generate_mesh(&positions, &colors);
      assert!(failed_result.is_err());
      println!("{:?}", failed_result);
    }
  }
}
