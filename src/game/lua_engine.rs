use std::fs::read_to_string;

use mlua::Lua;

///
/// LuaEngine encapsulates the Luau virtual machine.
/// It is done this way so we can utilize Luau as
/// elegantly as possible.
///
pub struct LuaEngine {
  lua: Lua,
  output_code_string: bool,
}

impl LuaEngine {
  pub fn new() -> Self {
    let new_engine = LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
    };

    new_engine.generate_internal();

    return new_engine;
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
    let raw_code_string = read_to_string(&file_location).unwrap();

    if self.output_code_string {
      println!("{}", raw_code_string);
    }

    match self.lua.load(raw_code_string).exec() {
      Ok(_) => (),
      Err(err) => {
        // This needs some modification so we can post just these two elements:
        // 1.) Lua file
        // 2.) Line/Offset
        panic!("Fatal error in {}: {}", file_location, err)
      }
    }
  }

  ///
  /// Run the global on_step function in the LuauJIT VM environment.
  ///
  pub fn on_step(&self, delta: f64) {
    self.run_code(format!("_G.engine_on_step_function({})", delta))
  }
}
