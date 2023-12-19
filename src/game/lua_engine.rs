mod lua_file_helpers;

use core::panic;
use std::{cell::RefCell, sync::Arc};

use configparser::ini::Ini;
use mlua::Lua;

use self::lua_file_helpers::{check_game, get_game_path, read_file_to_string};

use super::Game;

///
/// LuaEngine encapsulates the Luau virtual machine.
/// It is done this way so we can utilize Luau as
/// elegantly as possible.
///
pub struct LuaEngine<'a> {
  lua: Lua,
  output_code_string: bool,
  game: Option<Arc<RefCell<Game<'a>>>>,
}

impl<'a> LuaEngine<'a> {
  pub fn new(reference: Arc<RefCell<Game<'a>>>) -> Self {
    let mut new_engine = LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
      game: None,
    };

    new_engine.game = Some(reference);
    // game: Some(reference),

    new_engine.generate_internal();

    return new_engine;
  }

  ///
  /// Run the global on_step function in the LuauJIT VM environment.
  ///
  pub fn on_step(&self, delta: f64) {
    self.run_code(format!("_G.engine_on_step_function({})", delta))
  }

  ///
  /// Generates the on_step(delta: number) function so it becomes a secret and hidden engine component.
  ///
  pub fn generate_internal(&self) {
    self.run_file("./api/__internal.lua".to_string())
  }

  ///
  /// Completely unfiltered and unsandboxed code compiler/runner.
  ///
  pub fn run_code(&self, raw_code: String) {
    match self.lua.load(raw_code).exec() {
      Ok(_) => (),
      Err(err) => panic!("minetest: A fatal error has occurred! {}", err),
    }
  }

  ///
  /// Completely unfiltered and unsandboxed file compiler/runner.
  ///
  pub fn run_file(&self, file_location: String) {
    let raw_code_string = read_file_to_string(&file_location);

    if self.output_code_string {
      println!("{}", raw_code_string);
    }

    match self.lua.load(raw_code_string).exec() {
      Ok(_) => (),
      Err(err) => {
        // This needs some modification so we can post just these two elements:
        // 1.) Lua file
        // 2.) Line/Offset
        panic!(
          "minetest: encountered fatal mod error in {}: {}",
          file_location, err
        )
      }
    }
  }

  ///
  /// Parses the game.conf file.
  /// A double check on the conf file's existence.
  ///
  fn parse_game_conf(&mut self, games_dir: &String, game_name: &String) {
    let mut base_path = get_game_path(games_dir, game_name);
    base_path.push_str("/game.conf");

    let mut config = Ini::new();

    let game_raw_config_string = read_file_to_string(&base_path);

    match config.read(game_raw_config_string) {
      Ok(_) => println!("minetest: parsed [{}] game config.", game_name),
      Err(e) => panic!(
        "minetest: error parsing [{}] game config! {} ",
        game_name, e
      ),
    }

    let real_game_name = match config.get("info", "name") {
      Some(val) => val,
      None => panic!("minetest [{}] is missing [name] in game.conf!", game_name),
    };

    println!("we got: {}", real_game_name);
  }

  ///
  /// Load up each mod in a game.
  /// todo: use dependency hierarchy [topological sorting (reverse postorder)topological sorting (reverse postorder)] <- luatic
  ///
  fn load_game_mods(&self, game_name: String) {}

  ///
  /// Load up a game directly.
  ///
  pub fn load_game(&mut self, game_name: String) {
    // Todo: Maybe this can be a compile time const?
    // We can choose between run-in-place or system installed
    let games_dir = String::from("./games");

    check_game(&games_dir, &game_name);

    //todo: mod conf parser to set game state variables.
    // Use this to set server variables.
    self.parse_game_conf(&games_dir, &game_name);
  }
}
