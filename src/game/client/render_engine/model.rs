use super::mesh::Mesh;

///
/// ! Fixme: this should be immutable, don't change models during runtime.
/// ! use encapsulation to stop this from getting changed with a lockout.
///
pub struct Model {
  pub name: String,
  pub meshes: Vec<Mesh>,
  pub number_of_texture_buffers: u32,
  // todo: use this to lockout the model from changing and be readonly.
  // todo: You should have to completely regenerate a new model.
  pub lock: bool,
}
