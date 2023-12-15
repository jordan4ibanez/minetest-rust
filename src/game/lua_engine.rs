
use std::fs::read_to_string;

use mlua::Lua;


///
/// LuaEngine encapsulates the Luau virtual machine.
/// It is done this way so we can utilize Luau as
/// elegantly as possible.
/// 
pub struct LuaEngine {
  lua: Lua
}

impl LuaEngine {

  pub fn new() -> Self {
    LuaEngine {
      lua: Lua::new()
    }
  }

  ///
  /// Completely unfiltered and unsandboxed file compiler/runner.
  /// 
  pub fn run_file(&self, file_location: String) {
    let raw_code_string = read_to_string(file_location).unwrap();
    println!("{}", raw_code_string);
    let _ = self.lua.load(raw_code_string).exec();
  }
  
}