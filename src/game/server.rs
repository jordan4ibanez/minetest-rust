mod server_connection;

use std::{cell::RefCell, ops::Deref, rc::Rc, borrow::BorrowMut};

use self::server_connection::ServerConnection;

use super::{lua_engine::LuaEngine, Game};

pub struct Server<'server> {
  lua_engine: Option<LuaEngine<'server>>,
  connection: Option<ServerConnection>,
  game_pointer: Rc<RefCell<Game<'server>>>,
  server_pointer: Option<Rc<RefCell<Server<'server>>>>,
}

impl<'server> Server<'server> {
  pub fn new(
    game_pointer: Rc<RefCell<Game<'server>>>,
    address: String,
    port: i32,
  ) -> Rc<RefCell<Self>> {
    let new_server = Rc::new(RefCell::new(Server {
      lua_engine: None,
      connection: Some(ServerConnection::new(address, port)),
      game_pointer: game_pointer.clone(),
      server_pointer: None,
    }));

    match &new_server.deref().borrow_mut().connection {
        Some(connection) => {
          println!("minetest: running on socket: {} (match)", connection.get_socket());
        },
        None => (),
    }
    println!(
      "minetest: running on socket: {}",
      new_server.deref().borrow_mut().connection.as_ref().unwrap().get_socket()
    );

    new_server.deref().borrow_mut().server_pointer = Some(new_server.clone());

    new_server.deref().borrow_mut().reset_lua_vm();

    new_server
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
    self.lua_engine = Some(LuaEngine::new(self.game_pointer.clone(), true));
  }

  ///
  /// Wipe the memory of the lua VM.
  /// Automatically regenerates a blank server VM.
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
      None => panic!("minetest: Server LuaEngine does not exist!"),
    }
  }
}
