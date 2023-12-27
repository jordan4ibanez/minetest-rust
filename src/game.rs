mod client;
mod lua_engine;
mod server;

use core::panic;
use std::{cell::RefCell, ops::Deref, rc::Rc};

use spin_sleep::LoopHelper;

use crate::command_line::CommandLineInterface;

use self::{client::Client, server::Server};

///
/// The master container for the game.
///
/// The main architectural design pattern for the engine is:
/// * Composition over inheritance.
///
/// All code from this point downward should be straight forward
/// and as un-mystical and magicless as possible.
///
/// All logic and storage will branch off of this and flow
/// downwards like a tree. If you want to find something, simply
/// follow the components in the direction you think they are.
///
/// * Note: The only thing that should be higher up in the stack
/// * is the actual main() entry point of the program that's
/// * encapsulating this struct as a requirement in rust.
///
/// ! Do not create multiple instances of game. It's monolithic.
///
pub struct Game<'game> {
  should_close: bool,

  goal_frames_per_second: f64,
  goal_ticks_per_second: f64,

  server: Option<Rc<RefCell<Server<'game>>>>,
  client: Option<Rc<RefCell<Client<'game>>>>,

  is_server: bool,
  is_client: bool,

  loop_helper: LoopHelper,
  delta: f64,
  current_fps: f64,

  // vsync can be:
  // off    - (0)
  // on     - (1)
  // double - (2)
  // triple - (3)
  vsync_mode: i8,

  game_pointer: Option<Rc<RefCell<Game<'game>>>>,
}

impl<'game> Game<'game> {
  pub fn new(cli: CommandLineInterface) -> Rc<RefCell<Game<'game>>> {
    println!("Minetest initialized!");

    // 60 FPS goal for the moment.
    let goal_frames_per_second = 60.0;

    // 20 Tick Per Second goal.
    let goal_ticks_per_second = 20.0;

    let loop_helper_goal = match cli.server {
      true => goal_ticks_per_second,
      false => goal_frames_per_second,
    };

    let loop_helper = LoopHelper::builder()
      .report_interval_s(1.0)
      .build_with_target_rate(loop_helper_goal);

    //todo: make this happen!
    println!("we need a minetest.conf parser for vsync!");

    let new_game = Game {
      should_close: false,

      goal_frames_per_second,
      goal_ticks_per_second,

      client: None,
      server: None,

      // Simply reverse these then we can plop in a server when
      // the player enters singleplayer.
      is_client: !cli.server,

      // If this is a server we don't do any client things.
      is_server: cli.server,

      loop_helper,

      delta: 0.0,
      current_fps: 0.0,

      //todo: fix this when the minetest.conf parser is implemented
      vsync_mode: 0,

      game_pointer: None,
    };

    // We now transfer ownership of the entire Game into an ARC
    // with interior mutability with RefCell.

    // Interior mutability. Like a final java object.
    let new_game_pointer = Rc::new(RefCell::new(new_game));

    // We can simply dispatch the smart pointer to this struct by cloning it now.
    new_game_pointer.deref().borrow_mut().game_pointer = Some(new_game_pointer.clone());

    // We could parse the player's name instead from a file, or a first time ask. This is mutable after all.
    new_game_pointer.deref().borrow_mut().client = match cli.server {
      false => Some(Client::new(
        new_game_pointer.clone(),
        cli.client_name,
        cli.address.clone(),
        cli.port,
      )),
      true => None,
    };

    // Can auto deploy server and treat this struct like a simplified dispatcher.
    new_game_pointer.deref().borrow_mut().server = match cli.server {
      true => Some(Server::new(
        new_game_pointer.clone(),
        cli.address,
        cli.port,
        cli.game,
      )),
      false => None,
    };

    new_game_pointer
  }

  ///
  /// This does the actual work of updating the framerate goal.
  /// It also automatically decides which one to use if this is
  /// A client, server, or singleplayer.
  ///
  fn update_target_framerate_goal(&mut self) {
    let new_goal = match self.is_client {
      true => self.goal_frames_per_second,
      false => self.goal_ticks_per_second,
    };

    // Now create a new struct with the desired goal.
    self.loop_helper = LoopHelper::builder()
      .report_interval_s(1.0)
      .build_with_target_rate(new_goal);
  }

  ///
  /// This simply returns the current delta time.
  ///
  pub fn get_delta(&self) -> f64 {
    self.delta
  }

  ///
  /// Update the games' target FPS.
  ///! Only has side effects if this is a client/singleplayer.
  ///  
  pub fn set_frame_rate_target(&mut self, new_frames_per_second_goal: f64) {
    // This will silently kick the actual worker function on.
    // Written out like this so that server & client invokations do not
    // get mixed up.
    self.goal_frames_per_second = new_frames_per_second_goal;
    self.update_target_framerate_goal()
  }

  ///
  /// Update the games' target TPS.
  ///! Only has side effects if this is a server.
  ///  
  pub fn set_tick_rate_target(&mut self, new_ticks_per_second_goal: f64) {
    // This will silently kick the actual worker function on.
    // Written out like this so that server & client invokations do not
    // get mixed up.
    self.goal_ticks_per_second = new_ticks_per_second_goal;
    self.update_target_framerate_goal()
  }

  ///
  /// The main loop of the game engine.
  ///
  fn main(&mut self) {
    self.delta = self.loop_helper.loop_start_s();

    //? Here is where the logic loop goes.

    //* Begin server/client on_tick()

    if self.is_server {
      match &self.server {
        Some(server) => {
          server.deref().borrow_mut().on_tick(self.delta);
        }
        None => panic!("minetest: attempted to run a server that does not exist."),
      }
    }

    if self.is_client {
      match &mut self.client {
        Some(client) => client.deref().borrow_mut().on_tick(self.delta),
        None => panic!("minetest: attempted to run a client that does not exist."),
      }
    }

    //* End server/client on_tick()

    //todo: make this a configuration for debugging.
    //todo: this can also be linked into the client struct to report
    //todo: the current framerate.
    if let Some(fps) = self.loop_helper.report_rate() {
      self.current_fps = fps;
      let time_measurement = match self.is_client {
        true => "FPS",
        false => "TPS",
      };
      println!("Debug {}: {}", time_measurement, self.current_fps)
    }

    if self.vsync_mode == 0 || self.is_server {
      self.loop_helper.loop_sleep();
    }
  }

  ///
  /// This is the actual entry point for the game.
  ///
  pub fn enter_main_loop(&mut self) {
    while !self.should_close {
      self.main()
    }
  }
}

impl<'game> Drop for Game<'game> {
  fn drop(&mut self) {
    println!("Minetest dropped!");
  }
}
