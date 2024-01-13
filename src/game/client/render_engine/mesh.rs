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
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
  position: [f64; POSITION_COMPONENTS],
  color: [f64; COLOR_COMPONENTS],
}

impl Vertex {
  pub fn new(position: [f64; POSITION_COMPONENTS], color: [f64; COLOR_COMPONENTS]) -> Self {
    Vertex { position, color }
  }
}

///
/// A Mesh is the container that holds the data which makes up a model.
///
pub struct Mesh {
  data: Vec<Vertex>,
}

impl Mesh {
  pub fn new() -> Self {
    Mesh { data: vec![] }
  }

  ///
  /// Push raw vertex data into the Mesh.
  ///
  pub fn push_vertex(&mut self, vertex: Vertex) {
    self.data.push(vertex);
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
pub fn generate_mesh(positions: &Vec<f64>, colors: &Vec<f64>) -> Result<(), String> {
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
  let mut mesh = Mesh::new();

  // Can use one range iterator, they are all supposed to be equal.
  for i in 0..positions_components {
    // Instead of unwrapping this in the future, we should match.
    println!("{}", i);

    let position_base_offset = i * POSITION_COMPONENTS;

    let position_slice: [f64; 3] = positions
      [position_base_offset..position_base_offset + POSITION_COMPONENTS]
      .try_into()
      .unwrap();

    let color_base_offset = i * COLOR_COMPONENTS;

    let color_slice: [f64; 3] = colors[color_base_offset..color_base_offset + COLOR_COMPONENTS]
      .try_into()
      .unwrap();

    mesh.push_vertex(Vertex {
      position: position_slice,
      color: color_slice,
    });
  }

  Ok(())
}
