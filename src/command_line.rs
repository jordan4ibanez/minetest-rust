use clap::Parser;

///
/// This is the CLI struct.
///
///* (C)ommand
///* (L)ine
///* (I)nterface
///
/// This is a completely modular struct which
/// will automatically parse and generate options
/// for the end user to utilize.
///

#[derive(Parser, Debug)]
#[command(about = "Welcome to the minetest help section.
Please see below for the list of available options.")]
#[command(author, version, long_about = None)]
pub struct CommandLineInterface {
  /// Run minetest as a server.
  #[arg(short, long, default_value_t = false)]
  pub server: bool,

  /// Start server with a specific game.
  #[arg(short, long, default_value_t = String::from("minetest"))]
  pub game: String,

  /// Start the server on a specific address.
  #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
  pub address: String,

  /// Start server on a specific port.
  #[arg(short, long, default_value_t = 30_001)]
  pub port: i32,

  /// The default name for your player. (this is a placholder)
  #[arg(short, long, default_value_t = String::from("singleplayer"))]
  pub client_name: String,
}
