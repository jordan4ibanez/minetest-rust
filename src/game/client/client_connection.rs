use std::{cell::RefCell, rc::Rc, time::Duration, borrow::BorrowMut};

use message_io::{
  events::EventReceiver,
  network::{Transport, ToRemoteAddr, Endpoint},
  node::{self, NodeHandler, NodeTask, StoredNetEvent, StoredNodeEvent},
};

use super::Client;

///
/// ClientConnection and Client can be considered 1 entity.
///
/// This is why client_pointer is not an Option<>.
///
pub struct ClientConnection<'client> {
  address: String,
  port: i32,

  end_point: Option<Endpoint>,
  task: Option<NodeTask>,
  handler: Option<NodeHandler<()>>,
  event_receiver: Option<EventReceiver<StoredNodeEvent<()>>>,

  client_pointer: Rc<RefCell<Client<'client>>>,
}

impl<'client> ClientConnection<'client> {
  pub fn new(client_pointer: Rc<RefCell<Client<'client>>>, address: String, port: i32) -> Self {
    let mut new_client_connection = ClientConnection {
      address,
      port,

      end_point: None,
      task: None,
      handler: None,
      event_receiver: None,

      client_pointer,
    };

    new_client_connection.initialize();

    new_client_connection
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

  pub fn event_reaction(&mut self, event: StoredNetEvent) {
    match event {
      StoredNetEvent::Connected(endpoint, established) => {
        println!("connecting...");
        if !established {
          panic!("minetest: failed to establish a connection.");
        } else {
          println!("minetest: established a connection. [{}]", endpoint)
        }
      },
      StoredNetEvent::Accepted(a, b) => todo!(),
      StoredNetEvent::Message(_, _) => todo!(),
      StoredNetEvent::Disconnected(_) => todo!(),
    }
  }

  ///
  /// Non-blocking event receiver for network events.
  ///
  pub fn receive(&mut self) {
    match &mut self.event_receiver {
      Some(event_receiver) => {
        if let Some(event) = event_receiver.receive_timeout(Duration::new(0, 0)) {
          match event {
            StoredNodeEvent::Network(new_event) => self.event_reaction(new_event),
            // todo: figure out what a signal is!
            StoredNodeEvent::Signal(_) => todo!(),
          }
        }
      },
      None => panic!("minetest: ClientConnection listener does not exist!"),
    }    
  }


  ///
  /// Internal initializer procedure automatically run on a new ServerConnection.
  ///
  fn initialize(&mut self) {
    let remote_address = self.get_socket().to_remote_addr().unwrap();
    let transport_protocol = Transport::Udp;

    // todo: will need to do a handshake here.
    // todo: will need to be initialized by the gui component.

    let (handler, listener) = node::split();

    let (server_id, local_address) = match handler
      .network()
      .connect(transport_protocol, remote_address)
    {
      Ok((end_point, local_address)) => {
        println!(
          "minetest: established connection to server at id [{}], local address [{}]",
          end_point, local_address
        );
        (end_point, local_address)
      }
      Err(e) => panic!("{}", e),
    };

    let (task, event_receiver) = listener.enqueue();
    self.end_point = Some(server_id);
    self.handler = Some(handler);
    self.task = Some(task);
    self.event_receiver = Some(event_receiver);
  }
}

impl<'client> Drop for ClientConnection<'client> {
  fn drop(&mut self) {
    // Need to close client connection, maybe?
    // Might need to send out the disconnect signal.
    // todo: experiment with this.
    println!("ClientConnection dropped!")
  }
}
