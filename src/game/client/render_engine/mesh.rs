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

///
/// !This is a highly experimental function. This might get replaced with something
///
/// Generate an array of Vertex data from raw lists.
///
/// todo: Instead of a () this needs to return the Mesh container when it's made on Ok(())!
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
    return Err("generate_mesh: send a blank colors vector!".to_string());
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

  Ok(())
}
