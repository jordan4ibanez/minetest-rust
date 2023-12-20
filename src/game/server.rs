use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use super::{lua_engine::LuaEngine, Game};

pub struct Server<'server> {
  lua_engine: Option<LuaEngine<'server>>,
  game_pointer: Rc<RefCell<Game<'server>>>
}

impl<'server> Server<'server> {
  pub fn new(game_pointer: Rc<RefCell<Game<'server>>>) -> Self {
    let mut new_server = Server {
      lua_engine: None,
      game_pointer: game_pointer.clone()
    };

    new_server.lua_engine = Some(LuaEngine::new(game_pointer.clone(), true));

    new_server
  }

  }

  pub fn on_tick(&mut self, delta: f64) {
    println!("server on tick! {}", delta);
  }
}
