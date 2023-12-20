pub struct Client {
  name: String,
}

impl Client {
  pub fn new(player_name: String) -> Self {
    Client { name: player_name }
  }

  pub fn change_name(&mut self, new_player_name: String) {
    self.name = new_player_name;
  }

  pub fn get_name(&self) -> String {
    // Just fire off new heap memory.
    self.name.clone()
  }

  pub fn on_tick(&mut self, delta: f64) {
    println!("client on tick! {}", delta);
  }
}
