use std::{
  fs::{self, File},
  io::BufReader,
  path::Path,
};

///
/// A micro helper function.
/// Simply check if a directory exists.
///
pub fn dir_exists(path: &str) -> bool {
  Path::new(path).is_dir()
}

///
/// This is the same as dir_exists.
/// It is only separate so we know explicitly if we're looking for
/// a file.
///
pub fn file_exists(path: &str) -> bool {
  Path::new(path).is_file()
}

///
/// Get a file name from the path provided.
///
pub fn file_name_from_path(path: &str) -> Result<&str, &str> {
  let new_path = Path::new(path);

  if !new_path.exists() {
    return Err("File name from file path. Path does not exist.");
  }

  match new_path.file_name() {
    Some(os_str) => match os_str.to_str() {
      Some(final_str) => Ok(final_str),
      None => Err("File name from file path. Failed to convert OsStr to str."),
    },
    None => Err("File name from file path. Failed to parse OS Path str."),
  }
}

///
/// Get a file extension from the path provided.
///
pub fn file_extension_from_path(path: &str) -> Result<&str, &str> {
  let new_path = Path::new(path);

  if !new_path.exists() {
    return Err("Extension from file path. Path does not exist.");
  }

  match new_path.extension() {
    Some(extension_os_str) => match extension_os_str.to_str() {
      Some(os_str) => Ok(os_str),
      None => Err("Extension from file path. Failed to convert OsStr to str."),
    },
    None => Err("Extension from file path. Failed to parse OS Path str."),
  }
}

///
/// Automatically parse a file path into a String.
///
pub fn read_file_to_string(path: &str) -> Result<String, String> {
  match fs::read_to_string(path) {
    Ok(data) => Ok(data),
    Err(e) => Err(format!("Path to String read failure. {}", e)),
  }
}

///
/// Automatically parse a file path into a byte Vec.
///
pub fn read_file_to_byte_vec(path: &str) -> Result<Vec<u8>, String> {
  match fs::read(path) {
    Ok(data) => Ok(data),
    Err(e) => Err(format!("Path to byte Vec read failure. {}", e)),
  }
}

///
/// Automatically parse a file path into a BufReader<File>.
///
pub fn read_path_to_buf_read(path: &str) -> Result<BufReader<File>, String> {
  match File::open(path) {
    Ok(file) => Ok(BufReader::new(file)),
    Err(e) => Err(format!("Path to BufReader failure. {}", e)),
  }
}
