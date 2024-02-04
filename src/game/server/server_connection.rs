use std::{net::ToSocketAddrs, time::Duration};

use ahash::AHashMap;
use message_io::{
  events::EventReceiver,
  network::{Endpoint, Transport},
  node::{self, NodeHandler, NodeTask, StoredNetEvent, StoredNodeEvent},
};

///
/// ServerConnection and Server can be considered 1 entity.
///
/// This is the connection component for the Server.
///
pub struct ServerConnection {
  address: String,
  port: i32,

  task: NodeTask,
  handler: NodeHandler<()>,
  event_receiver: EventReceiver<StoredNodeEvent<()>>,
  pub clients: AHashMap<Endpoint, String>,

  // Multiple shutdown requests from valid endpoints can be sent in the same tick.
  // We want to process them all.
  pub shutdown_requests: Vec<Endpoint>,
}

impl ServerConnection {
  pub fn new(address: String, port: i32) -> Self {
    let socket_address = Self::get_socket(&address, port)
      .to_socket_addrs()
      .unwrap()
      .next()
      .unwrap();
    let transport_protocol = Transport::Udp;

    let (handler, listener) = node::split::<()>();

    // todo: fixme: this is udp, why are we doing a match here?
    // todo: If this fails, the server probably doesn't have a network
    // todo: adapter! Why is it a server?!
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

    ServerConnection {
      address,
      port,

      task,
      handler,
      event_receiver,
      clients: AHashMap::new(),

      shutdown_requests: vec![],
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
  /// Construct the address & port into a parsable socket string.
  ///
  pub fn get_socket(address: &str, port: i32) -> String {
    let mut socket = address.to_owned();
    socket.push(':');
    socket.push_str(port.to_string().as_str());

    socket
  }

  ///
  /// Send raw data to an EndPoint (ClientConnection).
  ///
  fn send_data(&self, end_point: Endpoint, data: &str) {
    self.handler.network().send(end_point, data.as_bytes());
  }

  ///
  /// A procedure to react to a network event.
  ///
  /// Returns if the connection received the shutdown signal from a client
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

      println!("minetest: Server received message: {}", receieved_string);

      match receieved_string.as_str() {
        "hi" => self.send_data(end_point, "hi there!"),
        "MINETEST_HAND_SHAKE" => self.send_data(end_point, "MINETEST_HAND_SHAKE_CONFIRMED"),
        "MINETEST_PING_REQUEST" => {
          println!("minetest: ServerConnection got ping request, sending confirmation to ClientConnection.");
          self.send_data(end_point, "MINETEST_PING_CONFIRMATION")
        }
        "MINETEST_SHUT_DOWN_REQUEST" => self.shutdown_requests.push(end_point),
        _ => (),
      }
    }
  }

  ///
  /// Non-blocking event receiver for network events.
  ///
  /// Returns the EndPoint (ClientConnection) that requested to shut
  /// down the server.
  ///
  pub fn receive(&mut self) {
    let mut has_new_event = true;

    // We want to grind through ALL the events.
    while has_new_event {
      if let Some(event) = self.event_receiver.receive_timeout(Duration::new(0, 0)) {
        match event {
          StoredNodeEvent::Network(new_event) => {
            self.event_reaction(new_event.clone());
          }
          // todo: figure out what a signal is!
          StoredNodeEvent::Signal(_) => todo!(),
        }
      } else {
        has_new_event = false;
      }
    }
  }
}

impl Drop for ServerConnection {
  fn drop(&mut self) {
    // ServerConnection must stop the handler entity or the Server
    // will not shut down.
    println!("ServerConnection: Shutting down network handler.");
    NodeHandler::stop(&self.handler);
    println!("ServerConnection dropped!");
  }
}
