use std::fs::read_to_string;

use mlua::{Lua, Table, Function};

///
/// LuaEngine encapsulates the Luau virtual machine.
/// It is done this way so we can utilize Luau as
/// elegantly as possible.
///
pub struct LuaEngine {
  lua: Lua,
  output_code_string: bool
}

impl LuaEngine {
  pub fn new() -> Self {
    LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
    }
  }

  ///
  /// Completely unfiltered and unsandboxed file compiler/runner.
  ///
  pub fn run_file(&self, file_location: String) {
    let raw_code_string = read_to_string(&file_location).unwrap();

    if self.output_code_string {
      println!("{}", raw_code_string);
    }
    match self.lua.load(raw_code_string).exec() {
      Ok(_) => (),
      Err(err) => {
        panic!("Fatal error in {}: {}", file_location, err)
      }
    }
  }

  ///
  /// The Luau environment is pretty neat.
  /// 
  pub fn on_step(&self, delta: f64) {

  }
}
