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

  pub fn get_address(&self) -> String {
    let mut final_address = self.address.clone();
    final_address.push(':');
    final_address.push_str(self.port.to_string().as_str());

    println!("{}", final_address);

    final_address
  }
}
