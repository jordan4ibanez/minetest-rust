mod client;
mod lua_engine;
mod server;

use spin_sleep::LoopHelper;

use self::{client::Client, lua_engine::LuaEngine, server::Server};

pub struct Game {
  should_close: bool,
  goal_fps: f64,
  goal_tps: f64,
  server: Option<Server>,
  client: Option<Client>,
  loop_helper: LoopHelper,
  delta: f64,
  current_fps: f64,
  lua_engine: LuaEngine,
}

impl Game {
  pub fn new(is_client: bool) -> Self {
    println!("Minetest initialized!");

    // We could parse the player's name instead from a file, or a first time ask. This is mutable after all.
    let client = match is_client {
      true => Some(Client::new(String::from("singleplayer"))),
      false => None,
    };

    // 60 FPS goal for the moment.
    let goal_fps = 60.0;

    // 20 Tick Per Second goal.
    let goal_tps = 20.0;

    // Can auto deploy server and treat this struct like a simplified dispatcher.
    let (server, loop_helper_goal) = match is_client {
      false => (Some(Server::new()), goal_tps),
      true => (None, goal_fps),
    };

    let loop_helper = LoopHelper::builder()
      .report_interval_s(1.0)
      .build_with_target_rate(loop_helper_goal);

    Game {
      should_close: false,

      goal_fps,
      goal_tps,

      client,
      server,

      loop_helper,

      delta: 0.0,
      current_fps: 0.0,

      lua_engine: LuaEngine::new(),
    }
  }

  pub fn enter_main_loop(&mut self) {
    self.lua_engine.run_file("./api/api.luau".to_string());

    while !self.should_close {
      self.main()
    }
  }

  pub fn busy_work(&mut self) {
    for i in 0..1_000 {}
  }

  pub fn main(&mut self) {
    self.delta = self.loop_helper.loop_start_s();

    //? Here is where the logic loop goes.

    if let Some(fps) = self.loop_helper.report_rate() {
      self.current_fps = fps;
      println!("TPS: {}", self.current_fps)
    }

    self.loop_helper.loop_sleep();
  }
}

impl Drop for Game {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}
