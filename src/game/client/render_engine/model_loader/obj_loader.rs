use tobj::{Material, Mesh};

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
  }
}
