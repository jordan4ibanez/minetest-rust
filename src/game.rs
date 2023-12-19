mod client;
mod lua_engine;
mod server;

use std::{cell::RefCell, ops::Deref, sync::Arc};

use spin_sleep::LoopHelper;

use self::{client::Client, lua_engine::LuaEngine, server::Server};

pub struct Game<'game> {
  should_close: bool,
  goal_frames_per_second: f64,
  goal_ticks_per_second: f64,
  server: Option<Server>,
  client: Option<Client>,
  is_server: bool,
  is_client: bool,
  loop_helper: LoopHelper,
  delta: f64,
  current_fps: f64,
  lua_engine: Option<LuaEngine<'game>>,
  smart_pointer: Option<Arc<RefCell<Game<'game>>>>,
}

impl<'game> Game<'game> {
  pub fn new(is_client: bool) -> Arc<RefCell<Game<'game>>> {
    println!("Minetest initialized!");

    // We could parse the player's name instead from a file, or a first time ask. This is mutable after all.
    let client = match is_client {
      true => Some(Client::new(String::from("singleplayer"))),
      false => None,
    };

    // 60 FPS goal for the moment.
    let goal_frames_per_second = 60.0;

    // 20 Tick Per Second goal.
    let goal_ticks_per_second = 20.0;

    // Can auto deploy server and treat this struct like a simplified dispatcher.
    let (server, loop_helper_goal) = match is_client {
      false => (Some(Server::new()), goal_ticks_per_second),
      true => (None, goal_frames_per_second),
    };

    let loop_helper = LoopHelper::builder()
      .report_interval_s(1.0)
      .build_with_target_rate(loop_helper_goal);

    let new_game = Game {
      should_close: false,

      goal_frames_per_second,
      goal_ticks_per_second,

      client,
      server,

      // Simply reverse these then we can plop in a server when
      // the player enters singleplayer.
      is_client,

      // If this is a server we don't do any client things.
      is_server: !is_client,

      loop_helper,

      delta: 0.0,
      current_fps: 0.0,

      lua_engine: None,

      smart_pointer: None,
    };

    // We now transfer ownership of the entire Game into an ARC
    // with interior mutability with RefCell.

    // Interior mutability. Like a final java object.
    let new_smart_pointer = Arc::new(RefCell::new(new_game));

    // We can simply dispatch the smart pointer to this struct by cloning it now.
    new_smart_pointer.deref().borrow_mut().smart_pointer = Some(new_smart_pointer.clone());

    new_smart_pointer.deref().borrow_mut().lua_engine =
      Some(LuaEngine::new(new_smart_pointer.clone()));

    new_smart_pointer
  }

  ///
  /// Allow self to distribute a clone of it's ARC smart pointer.
  /// It's written like this so it's more obvious what's going on.
  ///
  pub fn clone_smart_pointer(&self) -> Arc<RefCell<Game<'game>>> {
    self.smart_pointer.clone().unwrap()
  }

  ///
  /// This does the actual work of updating the framerate goal.
  /// It also automatically decides which one to use if this is
  /// A client, server, or singleplayer.
  ///
  fn update_target_framerate_goal(&mut self) {
    let new_goal = match self.is_client {
      true => self.goal_frames_per_second,
      false => self.goal_ticks_per_second,
    };

    // Now create a new struct with the desired goal.
    self.loop_helper = LoopHelper::builder()
      .report_interval_s(1.0)
      .build_with_target_rate(new_goal);
  }

  ///
  /// Update the games' target FPS.
  ///! Only has side effects if this is a client/singleplayer.
  ///  
  pub fn set_frame_rate_target(&mut self, new_frames_per_second_goal: f64) {
    // This will silently kick the actual worker function on.
    // Written out like this so that server & client invokations do not
    // get mixed up.
    self.goal_frames_per_second = new_frames_per_second_goal;
    self.update_target_framerate_goal()
  }

  ///
  /// Update the games' target TPS.
  ///! Only has side effects if this is a server.
  ///  
  pub fn set_tick_rate_target(&mut self, new_ticks_per_second_goal: f64) {
    // This will silently kick the actual worker function on.
    // Written out like this so that server & client invokations do not
    // get mixed up.
    self.goal_ticks_per_second = new_ticks_per_second_goal;
    self.update_target_framerate_goal()
  }

  ///
  /// The main loop of the game engine.
  ///
  fn main(&mut self) {
    self.delta = self.loop_helper.loop_start_s();

    //? Here is where the logic loop goes.
    // Might make this a client/server separated VM.
    // self.lua_engine.as_ref().unwrap().on_step(self.delta);

    if let Some(fps) = self.loop_helper.report_rate() {
      self.current_fps = fps;
      let time_measurement = match self.is_client {
        true => "FPS",
        false => "TPS",
      };
      println!("Debug {}: {}", time_measurement, self.current_fps)
    }

    self.loop_helper.loop_sleep();
  }

  ///
  /// This is the actual entry point for the game.
  ///
  pub fn enter_main_loop(&mut self) {
    //* testing

    let game_name = String::from("minetest");

    self.lua_engine.as_mut().unwrap().load_game(game_name);

    while !self.should_close {
      self.main()
    }
  }
}

impl<'game> Drop for Game<'game> {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}
