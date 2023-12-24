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

  client_pointer: Rc<RefCell<Client<'client>>>,
}

impl<'client> ClientConnection<'client> {
  pub fn new(client_pointer: Rc<RefCell<Client<'client>>>, address: String, port: i32) -> Self {
    let new_client_connection = ClientConnection {
      address,
      port,

      client_pointer,
    };

    new_client_connection
  }
}

impl<'client> Drop for ClientConnection<'client> {
  fn drop(&mut self) {
    println!("ClientConnection dropped!")
  }
}
