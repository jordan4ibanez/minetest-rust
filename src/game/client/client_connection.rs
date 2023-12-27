use std::{cell::RefCell, rc::Rc, time::Duration};

use message_io::{
  events::EventReceiver,
  network::{Endpoint, ToRemoteAddr, Transport},
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

  connected: bool,
  timeout: f64,
  test_remove_this: i32,

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

      connected: false,
      timeout: 0.0,
      test_remove_this: 0,

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

  ///
  /// Send raw data to the EndPoint (ServerConnection).
  ///
  fn send_data(&self, end_point: Endpoint, data: &str) {
    self
      .handler
      .clone()
      .unwrap()
      .network()
      .send(end_point, data.as_bytes());
  }

  ///
  /// A procedure to react to a network event.
  ///
  pub fn event_reaction(&mut self, event: StoredNetEvent) {
    // We don't need to match, we're using UDP which is connectionless.
    if let StoredNetEvent::Message(end_point, raw_message) = event {
      // todo: use https://github.com/serde-rs/bytes
      let receieved_string = match String::from_utf8(raw_message) {
        Ok(new_string) => new_string,
        Err(_) => {
          println!("minetest: message buffer attack detected, bailing on deserialization!");
          "".to_string()
        }
      };

      // Attempt to handshake with the server
      if !self.connected && receieved_string == "MINETEST_HAND_SHAKE_CONFIRMED" {
        self.connected = true;
        self.timeout = 0.0;
        println!("minetest: Handshake received!");
      }
    }
  }

  ///
  /// Test implementation is literally sending the server
  /// the word testing followed by the counter.
  ///
  /// Why yes, this will cause a panic in debug mode if you
  /// decide to run it for a few weeks stright.
  ///
  fn test_implementation(&mut self) {
    self.handler.clone().unwrap().network().send(
      self.end_point.unwrap(),
      format!("testing: {}", self.test_remove_this).as_bytes(),
    );
    self.test_remove_this += 1;
  }

  ///
  /// Non-blocking event receiver for network events.
  ///
  pub fn receive(&mut self, delta: f64) {
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
      None => panic!("minetest: ClientConnection listener does not exist!"),
    }

    // Handshake timeout, aka server connection timeout
    if !self.connected {
      self.timeout += delta;

      // 3 second timeout.
      // todo: make this not a panic.
      if self.timeout >= 3.0 {
        panic!("minetest: attempt to connect to server timed out.")
      }
    } else {
      // we're testing the implementation here.
      self.test_implementation();
    }
  }

  ///
  /// Internal initializer procedure automatically run on a new ServerConnection.
  ///
  fn initialize(&mut self) {
    let remote_address = self.get_socket().to_remote_addr().unwrap();
    let transport_protocol = Transport::Udp;

    // todo: will need to be initialized by the gui component.

    let (handler, listener) = node::split();

    // todo: fixme: this is udp, why are we doing a match here?
    // todo: If this fails, the use probably doesn't have a network
    // todo: adapter!
    let (server_id, local_address) = match handler
      .network()
      .connect(transport_protocol, remote_address)
    {
      Ok((end_point, local_address)) => {
        // UDP is connectionless, but it's still good to know it's working.
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

    // Can possibly be used as a handshake
    // ! Note: this literally is the handshake right now
    self.send_data(self.end_point.unwrap(), "MINETEST_HAND_SHAKE");
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
