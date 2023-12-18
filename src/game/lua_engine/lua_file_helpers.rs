///
/// This module is a thin wrapper around the built in rust components.
/// Made to be ultra-readable
///
/// It is done like this so we can have razor sharp components to bolt
/// into different elements of the engine.
///
/// It's also VERY specific to game-related components.
///
/// This flows down in complexity until you get the the public procedure.
///

use std::{path::Path, fs::{read_dir, ReadDir}};


///
/// A micro helper function.
/// Simply check if a directory exists.
///
fn dir_exists(path: &String) -> bool {
  Path::new(path).exists()
}

///
/// A micro helper function.
/// Ensure that the minetest/games/ folder exists.
///
fn check_games_folder(games_dir: &String) -> bool {
  dir_exists(&games_dir)
}

///
/// A micro helper function.
/// Generates a base path for a game.
///
fn get_game_path(games_dir: &String, game_name: &String) -> String {
  let mut base_path = games_dir.clone();
  base_path.push_str("/");
  base_path.push_str(&game_name);
  base_path
}

fn get_game_mod_path(games_dir: &String, game_name: &String) -> String {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/mods/");

  base_path
}

///
/// Check if a game exists.
///
fn game_exists(games_dir: &String, game_name: &String) -> bool {
  dir_exists(&get_game_path(&games_dir, game_name))
}

///
/// Check if a games mods folder exists.
///
fn game_mods_folder_exists(games_dir: &String, game_name: &String) -> bool {
  let mut base_path = get_game_path(&games_dir, game_name);
  base_path.push_str("/mods/");

  dir_exists(&base_path)
}


///
/// Ensure that the game has a mod.conf file.
/// 
fn game_has_conf_file(games_dir: &String, game_name: &String) -> bool {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/mod.conf");

  dir_exists(&base_path)
}


///
/// Get the mods folders inside of a game's dir.
/// 
fn get_game_mods_folders(games_dir: &String, game_name: &String) -> ReadDir {
  read_dir(get_game_mod_path(games_dir, game_name)).unwrap()
}

///
/// Ensure that the game's mods dir has at least one folder.
/// 
fn game_has_mods(games_dir: &String, game_name: &String) -> bool {
  let folders: ReadDir = get_game_mods_folders(games_dir, game_name);

  let mut folder_counter = 0;
  folders.for_each(|folder_result| {
    // We could chain these unwraps to tell the user they don't have access.
    // Use a match if this is decided upon.
    if folder_result.unwrap().file_type().unwrap().is_dir() {
      folder_counter += 1;
    }
  });

  folder_counter > 0
}


///
/// Runs all checks in one clean procedure.
///
pub fn check_game(games_dir: &String, game_name: &String) {
  if !check_games_folder(games_dir) {
    panic!("minetest: games folder [{}] does not exist.", games_dir);
  }

  if !game_exists(games_dir, game_name) {
    panic!("minetest: game {} does not exist!", game_name);
  }

  if !game_mods_folder_exists(games_dir, game_name) {
    panic!("minetest: game {} does not have a mods folder!", game_name);
  }

  if !game_has_conf_file(games_dir, game_name) {
    panic!("minetest: game {} does not have a mod.conf file!", game_name);
  }

  if !game_has_mods(games_dir, game_name) {
    panic!("minetest: game {} does not have any mods!", game_name);
  }
}
