use clap::Parser;

///
/// This is the CLI object.
///* (C)ommand
///* (L)ine
///* (I)nterface
/// This is a completely modular struct which
/// will automatically parse and generate options
/// for the end user to utilize.

#[derive(Parser, Debug)]
#[command(about = 
"Welcome to the minetest help section.
Please see below for the list of available arguments.")]
#[command(author = "floof", version, long_about = None)]
pub struct CommandLineInterface {

  /// Run minetest as a server.
  #[arg(short, long,  default_value_t = false)]
  pub server: bool,

  /// Start minetest with a specific game.
  #[arg(short, long, default_value_t = String::from("minetest"))]
  pub game: String,

  /// Start the server on a specific port.
  #[arg(short, long, default_value_t = 30_001)]
  pub port: i32,
}