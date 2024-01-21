use std::{fs, path::Path};

///
/// A micro helper function.
/// Simply check if a directory exists.
///
pub fn dir_exists(path: &str) -> bool {
  Path::new(path).exists()
}

///
/// This is the same as dir_exists.
/// It is only separate so we know explicitly if we're looking for
/// a file.
///
pub fn file_exists(path: &str) -> bool {
  Path::new(path).exists()
}

///
/// This is a very lazy function but it cleans up implementation.
///
fn panic_if_no_path(path: &str, read_to_type: &str) {
  if !file_exists(path) {
    panic!(
      "minetest: tried to read file [{}] into [{}] which doesn't exist!",
      path, read_to_type
    )
  }
}
  fs::read_to_string(path).unwrap().parse().unwrap()
}
