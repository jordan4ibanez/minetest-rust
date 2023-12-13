use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct Game {
  should_close: bool,
  goal_delta: f64,
  delta: f64,
  old_time: Instant
}

impl Game {

  pub fn new() -> Self {
    println!("Minetest initialized!");
    Game {
      should_close: false,
      // 60 FPS goal for the moment.
      goal_delta: 1.0 / 60.0,
      delta: 0.0,
      old_time: Instant::now(),
    }
  }

  pub fn enter_main_loop(&mut self) {
    while !self.should_close {
      self.main()
    }
  }

  pub fn main(&mut self) {
    // This is an extremely overkill way to get delta time.

    println!("{}", self.goal_delta);

    sleep(Duration::new(0,(self.goal_delta * 1_000_000_000.0).floor() as u32));

    self.delta = self.old_time.elapsed().as_secs_f64();

    println!("running | delta: {}", self.delta);


    self.old_time = Instant::now();
  }
}

impl Drop for Game {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}