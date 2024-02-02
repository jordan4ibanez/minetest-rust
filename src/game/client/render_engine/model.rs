use super::{mesh::Mesh, texture::Texture};

pub struct Model {
  pub meshes: Vec<Mesh>,
  pub materials: Vec<Texture>,
}
