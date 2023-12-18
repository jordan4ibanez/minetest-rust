use std::{fs, path::Path};

///
/// This module is a thin wrapper around the built in rust components.
/// It is done like this so we can have razor sharp components to bolt
/// into different elements of the engine.
///

///
/// Simply check if a directory exists.
///
pub fn dir_exists(path: &String) -> bool {
  Path::new(path).exists()
}

///
///! In the future: The game can hold a path to the .minetest folder.
///
/// Check if a game exists.
///
///* It is currently hardcoded to be run-in-place.
///
pub fn game_exists(game_name: &String) -> bool {
  // This can be modified in the future for run-in-place vs system

  let mut base_path = String::from("./games/");
  base_path.push_str(&game_name);

  dir_exists(&base_path)
}
