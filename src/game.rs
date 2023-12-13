pub struct Game {
  should_close: bool,
  goal_delta: f64,
}

impl Game {

  pub fn new() -> Self {
    println!("Minetest initialized!");
    Game {
      should_close: false,
      // 60 FPS goal for the moment.
      goal_delta: 1.0 / 60.0,
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


  }
}

impl Drop for Game {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}