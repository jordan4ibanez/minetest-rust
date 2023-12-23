pub struct ServerConnection {
  address: String,
  port: i32,
}

impl ServerConnection {
  pub fn new() -> Self {
    ServerConnection {
      address: "127.0.0.1".to_string(),
      port: 30_001,
    }
  }
}
