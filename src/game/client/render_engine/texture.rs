use image::GenericImageView;

use crate::file_utilities::read_file_to_byte_vec;

pub struct Texture {}

pub fn blah() {
  let diffuse_bytes = read_file_to_byte_vec("happy-tree.png");

  let diffuse_image = image::load_from_memory(diffuse_bytes.as_slice()).unwrap();
  let diffuse_rgba = diffuse_image.to_rgba8();
}
