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
}
