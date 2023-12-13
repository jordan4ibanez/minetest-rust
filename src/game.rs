use spin_sleep::LoopHelper;

pub struct Game {
  should_close: bool,
  goal_fps: f64,
  goal_tps: f64,
  loop_helper: LoopHelper,
  delta: f64,
  current_fps: f64
}

impl Game {

  pub fn new() -> Self {
    println!("Minetest initialized!");
    Game {
      should_close: false,
      // 60 FPS goal for the moment.
      goal_fps: 60.0,
      // 20 Tick Per Second goal.
      goal_tps: 20.0,
      loop_helper: LoopHelper::builder()
        .report_interval_s(1.0)
        .build_with_target_rate(20.0),
      delta: 0.0,
      current_fps: 0.0
    }
  }

  pub fn enter_main_loop(&mut self) {
    while !self.should_close {
      self.main()
    }
  }

  pub fn busy_work(&mut self) {
    for i in 0..1_000 {

    }
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