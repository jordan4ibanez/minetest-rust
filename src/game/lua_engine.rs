use std::fs::read_to_string;

use mlua::{Lua, Table, Function};

///
/// LuaEngine encapsulates the Luau virtual machine.
/// It is done this way so we can utilize Luau as
/// elegantly as possible.
///
pub struct LuaEngine {
  lua: Lua,
  output_code_string: bool,
  // We create this during bootup. So it's an option. :D
  on_step: Option<Function<'static>>
}

impl LuaEngine {
  pub fn new() -> Self {
    let mut new_engine = LuaEngine {
      lua: Lua::new(),
      output_code_string: false,
      on_step: None
    };

    new_engine.attach_internal_functions();

    new_engine
  }

  ///
  /// Allows this to be a "passthrough" in the constructor.
  /// 
  fn attach_internal_functions<'life>(&'life mut self) /*-> Function<'life>*/ {

    // We can make more advanced features in the future that are hidden from normal execution via:
    // This is just prototyping.
    // Pass ownership of on_step straight into LuaEngine.
    let on_step: Function<'_> = self.lua.create_function(|l: &Lua, delta: f64| -> Result<Option<bool>, mlua::prelude::LuaError> {
      println!("on_step: {}", delta);

      let globals = l.globals();
      let key: &str = "on_step";

      match globals.contains_key(key) {
        Ok(truth) => match truth {
            true => {
              let thing: Table = globals.get(key).unwrap();
              println!("LUA ON_STEP READY!");
            },
            false => println!("minetest: on_step not ready."),
        },
        Err(err) => println!("this is that error{}", err),
      }
  
      // this is:
      // Result -> Some(0.0)/None
      // If you don't do this, it will ALWAYS be an error!
      // We just use bool because it's easy to ignore it.
      // These are the same. (in this simple example)
      // let result: Option<bool> = None;
      // let result: Option<bool> = Some(false);
      
      Ok(Some(true))
    }).unwrap();

    self.on_step = Some(on_step)

    // self.on_step = Some(on_step);

    //? This is kept in for documentation.

    // let result: Result<bool, mlua::prelude::LuaError> = on_step.call::<_, bool>(0.0);

    // let cool = result.clone().unwrap();

    // println!("cool: {}", cool);

    // match result {
    //   Ok(_) => println!("it was okay"),
    //   Err(_) => println!("It was an ERROR AHH"),
    // }

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
