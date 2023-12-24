mod client_connection;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use self::client_connection::ClientConnection;

use super::{lua_engine::LuaEngine, Game};

pub struct Client<'client> {
  name: String,
  connection: Option<ClientConnection<'client>>,
  lua_engine: Option<LuaEngine<'client>>,
  game_pointer: Rc<RefCell<Game<'client>>>,
  client_pointer: Option<Rc<RefCell<Client<'client>>>>,
}

impl<'client> Client<'client> {
  pub fn new(game_pointer: Rc<RefCell<Game<'client>>>, client_name: String, address: String, port: i32) -> Rc<RefCell<Self>> {
    let new_client = Rc::new(RefCell::new(Client {
      name: client_name,
      connection: None,
      lua_engine: None,
      game_pointer: game_pointer.clone(),
      client_pointer: None,
    }));

    new_client.deref().borrow_mut().client_pointer = Some(new_client.clone());

    new_client.deref().borrow_mut().connection = Some(ClientConnection::new(new_client.clone(), address, port));

    new_client.deref().borrow_mut().reset_lua_vm();

    println!("minetest: player name is: {}", &new_client.deref().borrow().name);

    new_client
  }

  ///
  /// Change the client's name.
  ///
  pub fn change_name(&mut self, new_client_name: String) {
    self.name = new_client_name;
  }

  ///
  /// Get the client's name.
  ///
  pub fn get_name(&self) -> String {
    // Just fire off new heap memory.
    self.name.clone()
  }

  ///
  /// Deletes the lua VM.
  ///
  fn delete_lua_vm(&mut self) {
    self.lua_engine = None
  }

  ///
  /// Creates a new client lua VM.
  ///
  fn create_lua_vm(&mut self) {
    self.lua_engine = Some(LuaEngine::new(self.game_pointer.clone(), false));
  }

  ///
  /// Wipe the memory of the lua VM.
  /// Automatically regenerates a blank client VM.
  ///
  pub fn reset_lua_vm(&mut self) {
    self.delete_lua_vm();
    self.create_lua_vm();
  }

  ///
  /// Tick tock.
  ///
  /// Every time the game goes into the next main loop iteration
  /// this is run.
  ///
  /// This is referred to as on_step in C++ minetest.
  ///
  pub fn on_tick(&mut self, delta: f64) {
    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some()
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Client LuaEngine does not exist!"),
    }
  }
}
