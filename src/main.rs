pub mod game;

use game::*;

fn main() {
  // Game is one big ol' ARC smart pointer.
  //* - (A)tomically
  //* - (R)eference
  //* - (C)ounted
  // That's why this is written like this.
  // The main loop is literally borrowing it for the
  // lifetime of the game.
  (*Game::new(true)).borrow_mut().enter_main_loop()
}
