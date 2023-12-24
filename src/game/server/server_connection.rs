use std::{cell::RefCell, collections::HashMap, net::ToSocketAddrs, rc::Rc, time::Duration};

use message_io::{
  events::EventReceiver,
  network::{Endpoint, Transport},
  node::{self, NodeHandler, NodeTask, StoredNetEvent, StoredNodeEvent},
};

use super::Server;

///
/// ServerConnection and Server can be considered 1 entity.
///
/// This is why server_pointer is not an Option<>.
///
pub struct ServerConnection<'server> {
  address: String,
  port: i32,

  task: Option<NodeTask>,
  handler: Option<NodeHandler<()>>,
  event_receiver: Option<EventReceiver<StoredNodeEvent<()>>>,
  clients: HashMap<Endpoint, String>,

  server_pointer: Rc<RefCell<Server<'server>>>,
}

impl<'server> ServerConnection<'server> {
  pub fn new(server_pointer: Rc<RefCell<Server<'server>>>, address: String, port: i32) -> Self {
    let mut new_server_connection = ServerConnection {
      address,
      port,

      task: None,
      handler: None,
      event_receiver: None,
      clients: HashMap::new(),

      server_pointer,
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
  /// A procedure to react to a network event.
  ///
  pub fn event_reaction(&mut self, event: StoredNetEvent) {
    match event {
      node::StoredNetEvent::Connected(_, _) => {
        println!("minetest: client connection created.")
      }
      node::StoredNetEvent::Accepted(_, _) => {
        println!("minetest: client connection accepted.")
      }
      node::StoredNetEvent::Message(endpoint, raw_message) => {
        // todo: use https://github.com/serde-rs/bytes
        let receieved_string = match String::from_utf8(raw_message) {
          Ok(new_string) => new_string,
          Err(_) => {
            println!("minetest: message buffer attack detected, bailing on deserialization!");
            "".to_string()
          }
        };

        println!("minetest: received message: {}", receieved_string);
      }
      node::StoredNetEvent::Disconnected(_) => {
        println!("minetest: client disconnected.")
      }
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
      }
      None => panic!("minetest: ServerConnection listener does not exist!"),
    }
  }

  ///
  /// Internal initializer procedure automatically run on a new ServerConnection.
  ///
  fn initialize(&mut self) {
    let socket_address = self.get_socket().to_socket_addrs().unwrap().next().unwrap();
    let transport_protocol = Transport::Udp;

    let (handler, listener) = node::split::<()>();

    match handler.network().listen(transport_protocol, socket_address) {
      Ok((id, real_address)) => {
        println!(
          "minetest: connection created at id [{}], real address [{}]",
          id, real_address
        );
      }
      Err(e) => panic!("{}", e),
    }

    let (task, event_receiver) = listener.enqueue();
    self.handler = Some(handler);
    self.task = Some(task);
    self.event_receiver = Some(event_receiver);
  }
}

impl<'server> Drop for ServerConnection<'server> {
  fn drop(&mut self) {
    // Need to close server connection, maybe?
    // Might need to send out the disconnect signal.
    // todo: experiment with this.
    println!("ServerConnection dropped!")
  }
}
