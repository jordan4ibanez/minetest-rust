pub struct Server {
  // needs some kind of lock, maybe
}

impl Server {
  pub fn new() -> Self {
    Server {
      // todo
    }
  }

  pub fn on_tick(&mut self, delta: f64) {
    println!("server on tick! {}", delta);
  }
}
