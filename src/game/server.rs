mod server_connection;

use self::server_connection::ServerConnection;

use super::lua_engine::LuaEngine;

///
/// The Server component for the engine.
///
/// The Server component has 4 jobs:
/// 1.) Processes LuaEngine just as LuaJIT does in Minetest C++'s server.
/// 2.) Hold a ServerConnection component which will handle talking to clients.
/// * 3.) [in the future] Be the main handler for ServerAuthentication.
/// *  - ServerAuthentication does exactly what you think it does.
/// *  - It handles the client auth for the server.
/// ? 4.) Handle GameConfig as a component to be utilized during runtime.
/// ?  - Marked with ? because it's still being thought out at the moment.
///
pub struct Server {
  lua_engine: Option<LuaEngine>,
  connection: ServerConnection,
  shutdown_approved: bool,
}

impl Server {
  pub fn new(address: String, port: i32, game_name: String) -> Self {
    let mut new_server = Server {
      lua_engine: None,
      connection: ServerConnection::new(address, port),
      shutdown_approved: false,
    };

    // Automatically create a new Server LuaEngine.
    new_server.reset_lua_vm();

    // Automatically load up the requested game into memory.
    new_server.load_game(game_name);

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
    self.lua_engine = Some(LuaEngine::new(true));
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
  /// Chain initial game load into LuaEngine to clean up new() implemenetation.
  ///
  pub fn load_game(&mut self, game_name: String) {
    self.lua_engine.as_mut().unwrap().load_game(game_name)
  }

  ///
  /// Allows the game to check if the server has approved
  /// a shutdown request from a client.
  ///
  pub fn shutdown_is_approved(&self) -> bool {
    self.shutdown_approved
  }

  ///
  /// ! (will) [not implemented yet]
  /// todo: implement this somehow
  /// Automatically validate and accept/deny shutdown requests
  ///
  fn check_shutdown_requests(&mut self) {
    // Let's clear out the entire list so we don't cause a memory leak.
    // ! This NEEDS to check the database for a bad actor.
    while !self.connection.shutdown_requests.is_empty() {
      println!("looping");
      if let Some(shutdown_requester) = self.connection.shutdown_requests.pop() {
        println!(
          "minetest: shutdown requested by [{}]",
          shutdown_requester.addr()
        );
        self.shutdown_approved = true
      }
    }
  }

  ///
  /// Tick tock.
  ///
  /// Every time the game goes into the next main loop iteration
  /// this is run.
  ///
  /// This is referred to as on_step in C++ minetest.
  ///
  /// Returns shutdown signal.
  ///
  pub fn on_tick(&mut self, delta: f64) {
    // Process any incoming network traffic. (non blocking)

    self.connection.receive();

    self.check_shutdown_requests();
    if self.shutdown_approved {
      return;
    }

    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some() or if let
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Server LuaEngine does not exist!"),
    }
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    println!("Server dropped!");
  }
}
