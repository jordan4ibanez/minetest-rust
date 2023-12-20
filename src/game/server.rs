use std::{cell::RefCell, rc::Rc};

use super::{lua_engine::LuaEngine, Game};

pub struct Server<'server> {
  lua_engine: Option<LuaEngine<'server>>,
  game_pointer: Rc<RefCell<Game<'server>>>,
}

impl<'server> Server<'server> {
  pub fn new(game_pointer: Rc<RefCell<Game<'server>>>) -> Self {
    let mut new_server = Server {
      lua_engine: None,
      game_pointer: game_pointer.clone(),
    };

    new_server.lua_engine = Some(LuaEngine::new(game_pointer.clone(), true));

    new_server
  }

  pub fn delete_lua_vm(&mut self) {
    self.lua_engine = None
  }

  pub fn create_lua_vm(&mut self) {
    self.lua_engine = Some(LuaEngine::new(self.game_pointer.clone(), true));
  }

  pub fn on_tick(&mut self, delta: f64) {
    println!("server on tick! {}", delta);

    // This insanity destroys the lua vm one frame
    // then creates a new one, server internal parse and all.
    if self.lua_engine.is_some() {
      self.lua_engine.as_ref().unwrap().on_step(delta);
      self.delete_lua_vm()
    } else {
      self.create_lua_vm()
    }
  }
}
