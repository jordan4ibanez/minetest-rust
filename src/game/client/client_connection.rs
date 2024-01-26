use std::time::Duration;

use message_io::{
  events::EventReceiver,
  network::{Endpoint, ToRemoteAddr, Transport},
  node::{self, NodeHandler, NodeTask, StoredNetEvent, StoredNodeEvent},
};

///
/// ClientConnection and Client can be considered 1 entity.
///
/// This is the connection component for the Client.
///
pub struct ClientConnection {
  address: String,
  port: i32,

  connected: bool,

  handshake_timeout: f64,

  ping_resend_delta: f64,
  ping_waiting_receive: bool,
  ping_timeout: f64,

  lost_connection: bool,

  end_point: Endpoint,
  task: NodeTask,
  handler: NodeHandler<()>,
  event_receiver: EventReceiver<StoredNodeEvent<()>>,
}

impl ClientConnection {
  pub fn new(address: String, port: i32) -> Self {
    let remote_address = Self::get_socket(&address, port).to_remote_addr().unwrap();
    let transport_protocol = Transport::Udp;

    // todo: will need to be initialized by the gui component.

    let (handler, listener) = node::split();

    // todo: fixme: this is udp, why are we doing a match here?
    // todo: If this fails, the user probably doesn't have a network
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
    let end_point = server_id;

    // Can possibly be used as a handshake
    // ! Note: this literally is the handshake right now
    // self.send_data(self.end_point.unwrap(), "MINETEST_HAND_SHAKE");

    ClientConnection {
      address,
      port,

      connected: false,

      handshake_timeout: 0.0,

      ping_resend_delta: 0.0,
      ping_waiting_receive: false,
      ping_timeout: 0.0,

      lost_connection: false,

      end_point,
      task,
      handler,
      event_receiver,
    }
  }

  ///
  /// Get if the Client is connected to a server.
  ///
  pub fn is_connected(&self) -> bool {
    self.connected
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
  fn get_socket(address: &str, port: i32) -> String {
    let mut socket = address.to_owned();
    socket.push(':');
    socket.push_str(port.to_string().as_str());

    socket
  }

  ///
  /// Send raw data to the EndPoint (ServerConnection).
  ///
  fn send_data(&self, end_point: Endpoint, data: &str) {
    self.handler.network().send(end_point, data.as_bytes());
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

      match receieved_string.as_str() {
        "hi" => println!("minetest: The server says hi."),
        "MINETEST_HAND_SHAKE_CONFIRMED" => {
          // Received handshake with the server.
          if !self.connected {
            self.connected = true;
            self.handshake_timeout = 0.0;
            println!("minetest: ClientConnection received handshake from ServerConnection.");
          }

          // ! Do not enable this unless you want the server to
          // ! shutdown as soon as you connect.
          // self.send_data(end_point, "MINETEST_SHUT_DOWN_REQUEST");
        }
        "MINETEST_PING_CONFIRMATION" => {
          println!("minetest: ClientConnection ping received from ServerConnection.");
          self.ping_timeout = 0.0;
          self.ping_waiting_receive = false;
          self.ping_resend_delta = 0.0;
        }
        _ => (),
      }
    }
  }

  ///
  /// Will automatically calculate if the server has failed to provide a handshake.
  /// aka: the server is not online.
  ///
  fn check_handshake(&mut self, delta: f64) {
    // Handshake timeout, aka server connection timeout
    if !self.connected {
      self.handshake_timeout += delta;

      // 3 second timeout.
      // todo: make this not a panic.
      if self.handshake_timeout >= 3.0 {
        panic!("minetest: ClientConnection attempt to connect to server timed out.")
      }
    }
  }

  ///
  /// Will automatically calculate if the server has lost connection to the client.
  ///
  fn do_ping_timeout_logic(&mut self, delta: f64) {
    // If we're not connected, don't attempt to do this.
    if self.connected {
      if self.ping_waiting_receive {
        // We're waiting for the server to respond.
        self.ping_timeout += delta;

        // 3 second timeout.
        // todo: make this not a panic.
        if self.ping_timeout >= 3.0 {
          panic!("minetest: ClientConnection connection to server timed out.")
        }
      } else {
        // Wait 3 seconds before pinging the server again.
        self.ping_resend_delta += delta;

        if self.ping_resend_delta >= 3.0 {
          self.ping_waiting_receive = true;
          self.send_data(self.end_point, "MINETEST_PING_REQUEST");
        }
      }
    }
  }

  ///
  /// Non-blocking event receiver for network events.
  ///
  pub fn receive(&mut self, delta: f64) {
    if let Some(event) = self.event_receiver.receive_timeout(Duration::new(0, 0)) {
      match event {
        StoredNodeEvent::Network(new_event) => self.event_reaction(new_event),
        // todo: figure out what a signal is!
        StoredNodeEvent::Signal(_) => todo!(),
      }
    }

    self.check_handshake(delta);
    self.do_ping_timeout_logic(delta);
  }
}

impl Drop for ClientConnection {
  fn drop(&mut self) {
    // ClientConnection must stop the handler entity or the Client
    // will not shut down.
    println!("Clientconnection: Shutting down network handler.");
    NodeHandler::stop(&self.handler);
    println!("ClientConnection dropped!")
  }
}
