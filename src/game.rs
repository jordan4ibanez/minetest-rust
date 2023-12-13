use std::{time::{Duration, Instant}, os};
use std::thread::sleep;

pub struct Game {
  should_close: bool,
  delta: f64,
  old_time: Instant
}

impl Drop for Game {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}

impl Game {

  pub fn new() -> Self {
    println!("Minetest initialized!");
    Game {
      should_close: false,
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

    sleep(Duration::new(0,1_000_000));
    
    self.delta = self.old_time.elapsed().as_secs_f64();

    println!("running | delta: {}", self.delta);


    self.old_time = Instant::now();
  }
}