///
/// This module is a thin wrapper around the built in rust components.
/// Made to be as readable as possible.
///
/// This is specifically written for the LuaEngine.
///
/// This flows down in complexity until you get the the public procedures
/// these are: check_game, get_game_mod_folders
///
use std::fs::{read_dir, ReadDir};

use crate::file_utilities::{dir_exists, file_exists};

///
/// Makes the implementation of working with mod folders easier
/// to follow when reading it.
///
pub struct ModDirectory {
  pub mod_name: String,
  pub mod_path: String,
}

///
/// Makes the possible error at the end of check_game() easier to read.
///
struct CheckGameError {
  mod_name: String,
  conf_or_main: String,
}

///
/// A micro helper function.
/// Ensure that the minetest/games/ folder exists.
///
fn check_games_folder(games_dir: &str) -> bool {
  dir_exists(games_dir)
}

///
/// A micro helper function.
/// Generates a base path for a game.
///
pub fn get_game_path(games_dir: &str, game_name: &str) -> String {
  let mut base_path = games_dir.to_owned().clone();
  base_path.push('/');
  base_path.push_str(game_name);
  base_path
}

fn get_game_mod_path(games_dir: &str, game_name: &str) -> String {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/mods/");

  base_path
}

///
/// Check if a game exists.
///
fn game_exists(games_dir: &str, game_name: &str) -> bool {
  dir_exists(&get_game_path(games_dir, game_name))
}

///
/// Check if a games mods folder exists.
///
fn game_mods_folder_exists(games_dir: &str, game_name: &str) -> bool {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/mods/");

  dir_exists(&base_path)
}

///
/// Ensure that the game has a game.conf file.
///
fn game_has_conf_file(games_dir: &str, game_name: &str) -> bool {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/game.conf");

  dir_exists(&base_path)
}

///
/// Get the raw files inside of a game's dir.
///
fn get_game_mods_dir_raw_files(games_dir: &str, game_name: &str) -> ReadDir {
  match read_dir(get_game_mod_path(games_dir, game_name)) {
    Ok(data) => data,
    Err(e) => panic!("LuaHelpers: Failed to read raw directory files. {}", e),
  }
}

///
/// Ensure that the game's mods dir has at least one folder.
///
fn game_has_mods(games_dir: &str, game_name: &str) -> bool {
  let folders: ReadDir = get_game_mods_dir_raw_files(games_dir, game_name);

  let mut folder_counter = 0;
  for folder_result in folders {
    match folder_result {
      Ok(folder) => match folder.file_type() {
        Ok(file_type) => {
          if file_type.is_dir() {
            folder_counter += 1;
          }
        }
        Err(e) => panic!(
          "LuaHelpers: Failed to get file type for folder [{:?}]. {}",
          folder, e
        ),
      },
      Err(e) => println!(
        "LuaHelpers: Failed to get Directory Entry for file in game [{}]. File name {}",
        game_name, e
      ),
    }
  }

  folder_counter > 0
}

///
/// Ensure that each of the game's mods has a main.lua and a mod.conf file.
///
fn game_mods_have_main_and_conf(games_dir: &str, game_name: &str) -> Result<(), CheckGameError> {
  // Iterate each file in game's /mods/ folder.
  for mod_directory in get_game_mod_folders(games_dir, game_name) {
    //* First we check main.lua

    let mut main_lua_file = mod_directory.mod_path.clone();
    main_lua_file.push_str("/main.lua");

    if !file_exists(&main_lua_file) {
      //todo: We should have a conf parser to get the mod name.
      // We'll just use the folder name for now.
      return Err(CheckGameError {
        mod_name: mod_directory.mod_name,
        conf_or_main: "main.lua".to_string(),
      });
    }

    //* Then we check mod.conf

    let mut mod_conf_file = mod_directory.mod_path.clone();
    mod_conf_file.push_str("/mod.conf");

    if !file_exists(&mod_conf_file) {
      //todo: We should have a conf parser to get the mod name.
      // We'll just use the folder name for now.
      return Err(CheckGameError {
        mod_name: mod_directory.mod_name,
        conf_or_main: "mod.conf".to_string(),
      });
    }
  }

  Ok(())
}

///
/// Automatically get the mod folders in a game's directory as a vector of ModDirectory.
///
pub fn get_game_mod_folders(games_dir: &str, game_name: &str) -> Vec<ModDirectory> {
  let mut container = Vec::<ModDirectory>::new();

  let raw_files: ReadDir = get_game_mods_dir_raw_files(games_dir, game_name);

  // Let's get the ModDirectory with extreme safety.
  for directory_result in raw_files {
    match directory_result {
      Ok(directory) => match directory.file_type() {
        Ok(file_type) => {
          if file_type.is_dir() {
            let mod_name = match directory.file_name().to_str() {
              Some(file_name_str) => file_name_str.to_string(),
              None => panic!("LuaHelpers: Failed to get file name for [{:?}].", file_type),
            };

            let mod_path = match directory.path().to_str() {
              Some(mod_path_str) => mod_path_str.to_string(),
              None => panic!(
                "LuaHelpers: Failed to get directory &str for [{:?}].",
                file_type
              ),
            };

            container.push(ModDirectory { mod_name, mod_path })
          }
        }
        Err(e) => panic!(
          "LuaHelpers: Failed to get file type for folder [{:?}]. {}",
          directory, e
        ),
      },
      Err(e) => panic!(
        "LuaHelpers: Failed to get directory entry in game [{}]. {}",
        game_name, e
      ),
    }
  }

  container
}

///
/// Runs all checks in one clean procedure.
///
pub fn check_game(games_dir: &str, game_name: &str) {
  if !check_games_folder(games_dir) {
    panic!("LuaHelpers: games folder [{}] does not exist.", games_dir);
  }

  if !game_exists(games_dir, game_name) {
    panic!("LuaHelpers: game [{}] does not exist!", game_name);
  }

  if !game_mods_folder_exists(games_dir, game_name) {
    panic!(
      "LuaHelpers: game [{}] does not have a mods folder!",
      game_name
    );
  }

  if !game_has_conf_file(games_dir, game_name) {
    panic!(
      "LuaHelpers: game [{}] does not have a mod.conf file!",
      game_name
    );
  }

  if !game_has_mods(games_dir, game_name) {
    panic!("LuaHelpers: game [{}] does not have any mods!", game_name);
  }

  match game_mods_have_main_and_conf(games_dir, game_name) {
    Ok(_) => (),
    Err(check_game_error) => panic!(
      "LuaHelpers: mod [{}] in game [{}] has no [{}]!",
      check_game_error.mod_name, game_name, check_game_error.conf_or_main
    ),
  }
}
