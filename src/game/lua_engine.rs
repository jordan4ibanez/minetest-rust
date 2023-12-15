
use std::path::Path;

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

  
}