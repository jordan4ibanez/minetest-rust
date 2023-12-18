///
/// This module is a thin wrapper around the built in rust components.
/// Made to be as readable as possible.
///
/// This is specifically written for the LuaEngine.
///
/// This flows down in complexity until you get the the public procedure.
///
use std::{
  fs::{read_dir, ReadDir},
  path::Path,
};

///
/// A micro helper function.
/// Simply check if a directory exists.
///
fn dir_exists(path: &String) -> bool {
  Path::new(path).exists()
}

///
/// This is the same as dir_exists.
/// It is only separate so we know explicitly if we're looking for
/// a file, or a dir.
///
fn file_exists(path: &String) -> bool {
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
/// Ensure that the game has a game.conf file.
///
fn game_has_conf_file(games_dir: &String, game_name: &String) -> bool {
  let mut base_path = get_game_path(games_dir, game_name);
  base_path.push_str("/game.conf");

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
  for folder_result in folders {
    // We could chain these unwraps to tell the user they don't have access.
    // Use a match if this is decided upon.
    if folder_result.unwrap().file_type().unwrap().is_dir() {
      folder_counter += 1;
    }
  }

  folder_counter > 0
}

///
/// Ensure that each of the game's mods has a main.lua and a mod.conf file.
///
/// Result<(), (mod name, mod.conf/main.lua)>
///
fn game_mods_have_main_and_conf_lua(
  games_dir: &String,
  game_name: &String,
) -> Result<(), (String, String)> {
  // Iterate each file in game's /mods/ folder.
  for folder_result in get_game_mods_folders(games_dir, game_name) {
    let current_mod_result = folder_result.unwrap();

    // In case dumping random files into the /mods/ folder ever
    // causes this to try to iterate it, skip it.
    if !current_mod_result.file_type().unwrap().is_dir() {
      continue;
    }

    let current_mod_dir = String::from(current_mod_result.path().to_str().unwrap());

    //* First we check main.lua

    let mut main_lua_file = current_mod_dir.clone();
    main_lua_file.push_str("/main.lua");

    if !file_exists(&main_lua_file) {
      //todo: We should have a conf parser to get the mod name.
      // We'll just use the folder name for now.
      let current_mod_name = String::from(current_mod_result.file_name().to_str().unwrap());

      return Err((current_mod_name, "main.lua".to_string()));
    }

    //* Then we check mod.conf

    let mut mod_conf_file = current_mod_dir.clone();
    mod_conf_file.push_str("/mod.conf");

    if !file_exists(&mod_conf_file) {
      //todo: We should have a conf parser to get the mod name.
      // We'll just use the folder name for now.
      let current_mod_name = String::from(current_mod_result.file_name().to_str().unwrap());

      return Err((current_mod_name, "mod.conf".to_string()));
    }
  }

  Ok(())
}

///
/// Runs all checks in one clean procedure.
///
pub fn check_game(games_dir: &String, game_name: &String) {
  if !check_games_folder(games_dir) {
    panic!("minetest: games folder [{}] does not exist.", games_dir);
  }

  if !game_exists(games_dir, game_name) {
    panic!("minetest: game [{}] does not exist!", game_name);
  }

  if !game_mods_folder_exists(games_dir, game_name) {
    panic!(
      "minetest: game [{}] does not have a mods folder!",
      game_name
    );
  }

  if !game_has_conf_file(games_dir, game_name) {
    panic!(
      "minetest: game [{}] does not have a mod.conf file!",
      game_name
    );
  }

  if !game_has_mods(games_dir, game_name) {
    panic!("minetest: game [{}] does not have any mods!", game_name);
  }

  match game_mods_have_main_and_conf_lua(games_dir, game_name) {
    Ok(_) => (),
    Err(error_tuple) => panic!(
      "minetest: mod [{}] in game [{}] has no [{}]!",
      error_tuple.0, game_name, error_tuple.1
    ),
  }
}
