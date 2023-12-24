use std::{cell::RefCell, rc::Rc};

use super::Client;

///
/// ClientConnection and Client can be considered 1 entity.
/// 
/// This is why client_pointer is not an Option<>.
/// 
pub struct ClientConnection<'client> {
  address: String,
  port: i32,

  client_pointer: Rc<RefCell<Client<'client>>>
}

impl<'client> ClientConnection<'client> {
  pub fn new(client_pointer: Rc<RefCell<Client<'client>>>) -> Self {
    let new_client_connection = ClientConnection {
      address: "127.0.0.1".to_string(),
      port: 30_001,

      client_pointer
    };

    new_client_connection
  }
}


impl<'client> Drop for ClientConnection<'client> {
  fn drop(&mut self) {
    println!("ClientConnection dropped!")
  }
}