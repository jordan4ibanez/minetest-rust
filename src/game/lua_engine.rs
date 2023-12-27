mod lua_file_helpers;

use core::panic;
use std::{cell::RefCell, rc::Rc};

use configparser::ini::Ini;
use mlua::Lua;

use self::lua_file_helpers::{
  check_game, get_game_mod_folders, get_game_path, read_file_to_string,
};

use super::Game;

///
/// LuaEngine encapsulates the LuauJIT virtual machine.
/// It is done this way so we can utilize LuauJIT as
/// elegantly as possible.
///
pub struct LuaEngine<'game> {
  lua: Lua,
  output_code_string: bool,
  // ! Note: Might want to hold Server and Client instead of just game.
  // ! Note: Doing it like this might be overly verbose.
  // ! todo: See how this goes during development.
  game: Option<Rc<RefCell<Game<'game>>>>,
  server_vm: bool,
}

impl<'game> LuaEngine<'game> {
  pub fn new(reference: Rc<RefCell<Game<'game>>>, server_vm: bool) -> Self {
    let mut new_engine = LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
      game: None,
      server_vm,
    };

    new_engine.game = Some(reference);

    new_engine.generate_internal();

    new_engine
  }

  ///
  /// Run the global on_tick function in the LuauJIT VM environment.
  ///
  pub fn on_tick(&self, delta: f64) {
    self.run_code(format!("_G.engine_on_tick_function({})", delta))
  }

  ///
  /// Generates the on_tick(delta: number) function so it becomes a secret and hidden engine component.
  ///
  pub fn generate_internal(&self) {
    // We want the game to simply crash if the internals have problems.
    // You can't build upon what is fundamentally broken.
    if self.server_vm {
      // it's a server vm
      self.run_file("./api/server/__internal_server.lua").unwrap();
    } else {
      // it's a client vm
      self.run_file("./api/client/__internal_client.lua").unwrap();
    }
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
  /// ! We are letting LuauJIT be the sandbox. Look into this if
  /// ! we think we should implement better protection!
  ///
  pub fn run_file(&self, file_location: &str) -> Result<(), String> {
    let raw_code_string = read_file_to_string(file_location);

    if self.output_code_string {
      println!("{}", raw_code_string);
    }

    match self.lua.load(raw_code_string).exec() {
      Ok(_) => Ok(()),
      Err(err) => {
        // This needs some modification so we can post just these two elements:
        // 1.) Lua file
        // 2.) Line/Offset
        Err(format!(
          "minetest: encountered fatal mod error in {}: {}",
          file_location, err
        ))
      }
    }
  }

  ///
  /// Parses the game.conf file.
  /// A double check on the conf file's existence.
  ///
  fn parse_game_conf(&mut self, games_dir: &str, game_name: &String) {
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
  /// ? note: Due to the nature of LuauJIT, I'm not sure we
  /// ? actually need to sort anything.
  ///
  fn load_game_files(&self, game_name: String) {
    // Pretty much all of these functions come from lua_file_helpers.
  }

  ///
  /// Load up a game directly.
  ///
  /// This should _only_ be run on a server LuaEngine.
  /// ! **Never run this function on a Client!**
  ///
  pub fn load_game(&mut self, game_name: String) {
    // We _do not_ want a client to even attempt to load anything.
    // All required information should be sent by the Server to the Client.
    // Then it should be passed into the LuaEngine as needed.
    if !self.server_vm {
      panic!("minetest: tried to load game lua files on a client LuaEngine!")
    }

    // Todo: Maybe this can be a compile time const?
    // We can choose between run-in-place or system installed.
    let games_dir = String::from("./games");

    // Comes from lua_file_helpers.
    check_game(&games_dir, &game_name);

    //todo: mod conf parser to set game state variables.
    // Use this to set server variables.
    self.parse_game_conf(&games_dir, &game_name);

    // Now we finally load the actual game files into the LuaEngine.
    self.load_game_files(&games_dir, &game_name);
  }
}
