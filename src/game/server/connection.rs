pub struct Connection {
  address: String,
  port: i32,
}

impl Connection {
  pub fn new() -> Self {
    Connection {
      address: "127.0.0.1".to_string(),
      port: 30_001,
    }
  }
}
