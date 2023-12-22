use std::{cell::RefCell, rc::Rc};

use super::{lua_engine::LuaEngine, Game};

pub struct Client<'client> {
  name: String,
  lua_engine: Option<LuaEngine<'client>>,
  game_pointer: Rc<RefCell<Game<'client>>>,
}

impl<'client> Client<'client> {
  pub fn new(game_pointer: Rc<RefCell<Game<'client>>>, player_name: String) -> Self {
    let mut new_client = Client {
      name: player_name,
      lua_engine: None,
      game_pointer: game_pointer.clone(),
    };

    new_client.lua_engine = Some(LuaEngine::new(game_pointer, false));

    new_client
  }

  pub fn change_name(&mut self, new_player_name: String) {
    self.name = new_player_name;
  }

  pub fn get_name(&self) -> String {
    // Just fire off new heap memory.
    self.name.clone()
  }

  pub fn on_tick(&mut self, delta: f64) {
    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some()
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_step(delta),
      None => panic!("minetest: client LuaEngine does not exist!"),
    }
  }
}
