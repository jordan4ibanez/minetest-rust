pub struct ClientConnection {
  address: String,
  port: i32,
}

impl ClientConnection {
  pub fn new() -> Self {
    ClientConnection {
      address: "127.0.0.1".to_string(),
      port: 30_001,
    }
  }
}
