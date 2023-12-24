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

  ///
  /// Change the address that the server connection will utilize.
  /// 
  pub fn set_address(&mut self, new_address: String) {
    self.address = new_address;
  }

  ///
  /// Change the port that the server connection will utilize.
  /// 
  pub fn set_port(&mut self, new_port: i32) {
    self.port = new_port;
  }

  ///
  /// Construct the address & port into a parsable string.
  /// 
  pub fn get_address(&self) -> String {
    let mut final_address = self.address.clone();
    final_address.push(':');
    final_address.push_str(self.port.to_string().as_str());

    final_address
  }
}
