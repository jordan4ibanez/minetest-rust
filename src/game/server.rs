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
}

impl Server {
  pub fn new(address: String, port: i32, game_name: String) -> Self {
    let mut new_server = Server {
      lua_engine: None,
      connection: ServerConnection::new(address, port),
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
  /// Tick tock.
  ///
  /// Every time the game goes into the next main loop iteration
  /// this is run.
  ///
  /// This is referred to as on_step in C++ minetest.
  ///
  /// Returns shutdown signal.
  ///
  pub fn on_tick(&mut self, delta: f64) -> bool {
    let mut shutdown = false;

    // Process any incoming network traffic. (non blocking)

    // ! todo: this absolutely needs to be checked for server privs!
    if let Some(end_point) = self.connection.receive() {
      shutdown = true
    }

    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some()
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Server LuaEngine does not exist!"),
    }

    shutdown
  }
}
