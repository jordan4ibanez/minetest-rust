pub mod command_line;
pub mod file_utilities;
pub mod game;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use clap::Parser;
use command_line::CommandLineInterface;
use game::*;

///
/// ! main()'s sole purpose is to encapsulate and initialize
/// ! the Game struct. Nothing else should appear in here.
///
fn main() {
  // Game is held in an RC smart pointer.
  //* - (R)eference
  //* - (C)ounted
  // That's why this is written like this.
  // The entry point is literally borrowing the game struct
  // for the lifetime of the game.
  Rc::new(RefCell::new(Game::new(CommandLineInterface::parse())))
    .deref()
    .borrow_mut()
    .enter_main_loop();

  println!("minetest: shutdown procedure completed.");
}
