mod client_connection;
mod keyboard;
mod mouse;
mod render_engine;
mod window_handler;

use glam::Vec3A;

use self::{
  client_connection::ClientConnection, keyboard::KeyboardController, mouse::MouseController,
  render_engine::RenderEngine, window_handler::WindowHandler,
};

use super::lua_engine::LuaEngine;

///
/// The Client component for the engine.
///
/// The Client component has 5 jobs:
/// 1.) Hold a window.
/// 2.) Hold the render engine.
/// 3.) Hold a ClientConnection which handles talking to a server.
/// * 4.) [in the future] Be the main handler for ClientAuthentication.
/// *  - ClientAuthentication does exactly what you think it does.
/// *  - Maintains a client auth for itself when talking to the server.
/// ? 5.) Handle GameConfig as a component. This should be received from a server
/// ? 5 - Marked with ? because it's still being thought out at the moment.
///
pub struct Client {
  window_handler: WindowHandler,
  render_engine: RenderEngine,
  client_name: String,
  connection: Option<ClientConnection>,
  lua_engine: Option<LuaEngine>,

  mouse: MouseController,
  keyboard: KeyboardController,

  quit_received: bool,

  // ! TESTING
  spin_test: f64,
}

impl Client {
  pub fn new(client_name: String, address: String, port: i32) -> Self {
    // * This is testing for automatically locking the mouse in upon game engine start.
    let mut mouse = MouseController::new();
    let keyboard = KeyboardController::new();
    let window_handler = WindowHandler::new(&mut mouse);

    // Set up the render engine.
    let render_engine = RenderEngine::new(&window_handler);

    let mut new_client = Client {
      window_handler,
      render_engine,
      client_name,
      connection: None, //ClientConnection::new(address, port),
      lua_engine: None,

      mouse,
      keyboard,

      quit_received: false,

      // ! TESTING
      spin_test: 0.0,
    };

    new_client.reset_lua_vm();

    println!("minetest: player name is: {}", &new_client.client_name);

    new_client
  }

  ///
  /// Change the client's name.
  ///
  pub fn change_name(&mut self, new_client_name: String) {
    self.client_name = new_client_name;
  }

  ///
  /// Get the client's name.
  ///
  pub fn get_name(&self) -> String {
    // Just fire off new heap memory.
    self.client_name.clone()
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
    self.lua_engine = Some(LuaEngine::new(false));
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
  /// Send client quit event.
  ///
  pub fn quit(&mut self) {
    self.quit_received = true;
  }

  ///
  /// Retrieve if the client wants to quit.
  ///
  pub fn should_quit(&self) -> bool {
    self.quit_received
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
    // This is for the Mouse' Camera controls.
    self.mouse.reset_mouse_relative_position();

    // Update the SDL2 context.
    self
      .window_handler
      .update(delta, &mut self.mouse, &mut self.keyboard);

    // Poll any incoming network traffic. (non blocking)
    if let Some(connection) = &mut self.connection {
      connection.receive(delta);
    }

    //todo: probably should do user input here

    // We want this to throw a runtime panic if we make a logic error.
    // ! Never turn this into a silent bypass via: is_some() or if let
    match &self.lua_engine {
      Some(lua_engine) => lua_engine.on_tick(delta),
      None => panic!("minetest: Client LuaEngine does not exist!"),
    }

    //todo: should probably do side effects from lua here

    let camera = self.render_engine.get_camera();

    let mut camera_pos = *camera.get_position();

    // * A very simple test to check the buffer in the shader.
    if self.keyboard.is_key_down("A") {
      camera_pos.x += delta as f32;
    }
    if self.keyboard.is_key_down("D") {
      camera_pos.x -= delta as f32;
    }

    if self.keyboard.is_key_down("W") {
      camera_pos.z += delta as f32;
    }
    if self.keyboard.is_key_down("S") {
      camera_pos.z -= delta as f32;
    }

    if self.keyboard.is_key_down("Left Shift") {
      camera_pos.y += delta as f32;
    }
    if self.keyboard.is_key_down("Space") {
      camera_pos.y -= delta as f32;
    }

    // println!("camera pos {:?}", camera_pos);
    let mouse_relative = self.mouse.get_relative_position();
    if mouse_relative.length_squared() != 0 {
      println!("Mouse is moved!");
      let camera = self.render_engine.get_camera();
      let mut camera_rotation = *camera.get_rotation();

      camera_rotation.y += mouse_relative.x as f32 * self.mouse.get_sensitivity();
      camera_rotation.x += mouse_relative.y as f32 * self.mouse.get_sensitivity();

      camera.set_rotation(&camera_rotation);

      println!("{:?}", camera.get_rotation());
    }

    self.render_engine.get_camera().set_position(&camera_pos);

    // Update the RenderEngine with the WindowHandler.
    self.render_engine.update(&self.window_handler, delta);

    // Now render everything. 3 steps for now.

    self.spin_test += delta;

    self.render_engine.initialize_render(&self.window_handler);
    self.render_engine.render_mesh_unbatched(
      "debug",
      "tf.jpg",
      Vec3A::new(0.0, 0.0, 0.0),
      Vec3A::new(0.0, self.spin_test as f32, 0.0),
      Vec3A::new(0.0, 0.0, 0.0),
    );
    println!("spin  {}", self.spin_test);
    self.render_engine.process_render_calls();
    self.render_engine.finalize_render();

    // This will need to run a close event for the client engine and send out a close event to the internal server.
    if self.window_handler.should_quit() {
      self.quit();
    }
  }
}
