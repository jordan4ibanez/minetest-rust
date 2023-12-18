use std::{fs, path::Path};

///
/// This module is a thin wrapper around the built in rust components.
/// It is done like this so we can have razor sharp components to bolt
/// into different elements of the engine.
/// 



pub fn dir_exists(path: &String) -> bool {
  Path::new(path).exists()
}

