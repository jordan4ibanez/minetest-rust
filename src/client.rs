pub struct Client {
  name: String,
}

impl Client {
  pub fn new(player_name: String) -> Self {
    Client { name: player_name }
  }
}
