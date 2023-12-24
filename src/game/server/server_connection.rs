use std::{rc::Rc, cell::RefCell};

use super::Server;

///
/// ServerConnection and Server can be considered 1 entity.
/// 
/// This is why server_pointer is not an Option<>.
/// 
pub struct ServerConnection<'server> {
  address: String,
  port: i32,
  server_pointer: Rc<RefCell<Server<'server>>>
}

impl<'server> ServerConnection<'server> {
  pub fn new(server_pointer: Rc<RefCell<Server<'server>>>,address: String, port: i32) -> Self {
    let mut new_server_connection = ServerConnection {
      address,
      port,
      server_pointer
    };

    new_server_connection.initialize();

    new_server_connection
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
  /// Construct the address & port into a parsable socket string.
  ///
  pub fn get_socket(&self) -> String {
    let mut socket = self.address.clone();
    socket.push(':');
    socket.push_str(self.port.to_string().as_str());

    socket
  }

  ///
  /// Internal initializer procedure automatically run on a new ServerConnection.
  /// 
  fn initialize(&mut self) {
    

  }
}

impl<'server> Drop for ServerConnection<'server> {
  fn drop(&mut self) {
    println!("ServerConnection dropped!")   
  }
}