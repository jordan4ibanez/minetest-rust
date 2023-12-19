pub mod game;
pub mod command_line;

use std::ops::Deref;

use clap::Parser;
use command_line::CommandLineInterface;
use game::*;

fn main() {
  // Game is one big ol' ARC smart pointer.
  //* - (A)tomically
  //* - (R)eference
  //* - (C)ounted
  // That's why this is written like this.
  // The entry point is literally borrowing the game struct
  // for the lifetime of the game.
  Game::new(CommandLineInterface::parse()).deref().borrow_mut().enter_main_loop()
}
