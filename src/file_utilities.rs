use std::{
  convert::Infallible,
  fs::{self, File},
  io::BufReader,
  path::Path,
};

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
fn panic_if_no_path(path: &str, read_to_type: &str) {}

///
/// Get the file name from the path provided.
///
pub fn file_name_from_path(path: &str) -> Result<&str, &str> {
  let new_path = Path::new(path);

  if !new_path.exists() {
    return Err("Path does not exist.");
  }

  match new_path.file_name() {
    Some(os_str) => match os_str.to_str() {
      Some(final_str) => Ok(final_str),
      None => Err("Failed to convert OsStr to str."),
    },
    None => Err("Failed to parse OS Path str."),
  }
}

///
/// Get the file extension from the path provided.
///
pub fn file_extension_from_path(path: &str) -> Result<&str, &str> {
  let new_path = Path::new(path);

  if !new_path.exists() {
    return Err("Path does not exist.");
  }

  match new_path.extension() {
    Some(extension_os_str) => match extension_os_str.to_str() {
      Some(os_str) => Ok(os_str),
      None => Err("Failed to convert OsStr to str."),
    },
    None => Err("Failed to parse OS Path str."),
  }
}

///
/// This will first check if the file exists.
///
/// Next it will automatically parse the file into a String.
///
pub fn read_file_to_string(path: &str) -> String {
  panic_if_no_path(path, "String");
  fs::read_to_string(path).unwrap().parse().unwrap()
}

///
/// This will attempt to parse the file into a string.
///
pub fn read_file_to_string_result(path: &str) -> Result<String, Infallible> {
  fs::read_to_string(path).unwrap().parse()
}

///
/// This will first check if the file exists.
///
/// Next it will automatically parse the file into a byte Vec.
///
pub fn read_file_to_byte_vec(path: &str) -> Vec<u8> {
  panic_if_no_path(path, "bytes");
  fs::read(path).unwrap()
}

///
/// This will first check if the file exists.
///
/// Next it will automatically parse the file into a BufReader<File>
///
pub fn read_path_to_buf_read(path: &str) -> BufReader<File> {
  panic_if_no_path(path, "BufRead");
  BufReader::new(File::open(path).unwrap())
}
