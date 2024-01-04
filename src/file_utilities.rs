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
/// a file, or a dir.
///
pub fn file_exists(path: &str) -> bool {
  Path::new(path).exists()
}

///
/// A mini helper function.
/// This will first check if the file exists.
/// Next it will automatically parse it into a string.
///
pub fn read_file_to_string(path: &str) -> String {
  if !file_exists(path) {
    panic!("minetest: tried to read [{}] which doesn't exist!", path)
  }
  fs::read_to_string(path).unwrap().parse().unwrap()
}
