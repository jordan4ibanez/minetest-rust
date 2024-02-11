mod lua_file_helpers;

use core::panic;

use configparser::ini::Ini;
use mlua::Lua;

use crate::file_utilities::read_file_to_string;

use self::lua_file_helpers::{check_game, get_game_mod_folders, get_game_path};

///
/// LuaEngine encapsulates the LuauJIT virtual machine.
/// It is done this way so we can utilize LuauJIT as
/// elegantly as possible.
///
pub struct LuaEngine {
  lua: Lua,
  output_code_string: bool,
  server_vm: bool,
}

impl LuaEngine {
  pub fn new(server_vm: bool) -> Self {
    let new_engine = LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
      server_vm,
    };

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
      match self.run_file("./api/server/__internal_server.lua") {
        Ok(_) => (),
        Err(e) => panic!("LuaEngine: Failed to load Server API. {}", e),
      }
    } else {
      // it's a client vm
      match self.run_file("./api/client/__internal_client.lua") {
        Ok(_) => (),
        Err(e) => panic!("LuaEngine: Failed to load Client API. {}", e),
      }
    }
  }

  ///
  /// Completely unfiltered and unsandboxed code compiler/runner.
  ///
  pub fn run_code(&self, raw_code: String) {
    match self.lua.load(raw_code).exec() {
      Ok(_) => (),
      Err(err) => panic!("LuaEngine: A fatal error has occurred! {}", err),
    }
  }

  ///
  /// Completely unfiltered and unsandboxed file compiler/runner.
  ///
  /// ! We are letting LuauJIT be the sandbox. Look into this if
  /// ! we think we should implement better protection!
  ///
  pub fn run_file(&self, file_location: &str) -> Result<(), String> {
    let raw_code_string = match read_file_to_string(file_location) {
      Ok(raw_code) => raw_code,
      Err(e) => panic!("LuaEngine: {}", e),
    };

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
          "LuaEngine: encountered fatal mod error in {}: {}",
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

    let game_raw_config_string = match read_file_to_string(&base_path) {
      Ok(raw_config_string) => raw_config_string,
      Err(e) => panic!("LuaEngine: {}", e),
    };

    match config.read(game_raw_config_string) {
      Ok(_) => println!("LuaEngine: parsed [{}] game config.", game_name),
      Err(e) => panic!(
        "LuaEngine: error parsing [{}] game config! {} ",
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
  /// This function blindly accepts that check_game was already ran
  /// on this game.
  ///
  /// If you modified the source code and removed check_game() from load_game():
  /// _You're asking for trouble._
  ///
  fn load_game_files(&self, games_dir: &str, game_name: &str) {
    let game_mod_path = get_game_path(games_dir, game_name);

    for mod_directory in get_game_mod_folders(games_dir, game_name) {
      // ! this is a naive approach.
      // ! this might not work on windows!
      let mut mod_path = mod_directory.mod_path.clone();
      mod_path.push_str("/main.lua");

      println!(
        "--------------------\nLuaEngine: Server attempting to load mod file [{}]",
        &mod_path
      );

      // This simply panics for now, but in the future we can push errors to the GUI.
      match self.run_file(&mod_path) {
        Ok(_) => println!(
          "LuaEngine: Server loaded mod file [{}]\n--------------------",
          &mod_path
        ),
        Err(e) => panic!("{}", e),
      }
    }
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
      panic!("LuaEngine: tried to load game lua files on a client LuaEngine!")
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
