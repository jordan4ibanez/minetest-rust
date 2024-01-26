mod client;
mod delta_reporter;
mod lua_engine;
mod server;

use core::panic;
use std::{
  ops::Deref,
  sync::{Arc, RwLock},
  time::Duration,
};

use spin_sleep_util::{interval, Interval, RateReporter};

use crate::command_line::CommandLineInterface;

use self::{client::Client, delta_reporter::DeltaReporter, server::Server};

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
pub struct Game {
  should_close: Arc<RwLock<bool>>,

  goal_frames_per_second: f64,
  goal_ticks_per_second: f64,

  server: Option<Server>,
  client: Option<Client>,

  is_server: bool,
  is_client: bool,

  interval: Interval,
  fps_reporter: RateReporter,
  delta_reporter: DeltaReporter,

  delta: f64,
  current_fps: f64,

  // vsync can be:
  // off    - (0)
  // on     - (1)
  // double - (2)
  // triple - (3)
  vsync_mode: i8,
}

impl Game {
  pub fn new(cli: CommandLineInterface) -> Game {
    println!("Minetest initialized!");

    // Set up the environment logger.
    env_logger::init();

    // 60 FPS goal for the moment.
    let goal_frames_per_second = 60.0;

    // 20 Tick Per Second goal.
    let goal_ticks_per_second = 20.0;

    let loop_helper_goal = match cli.server {
      true => goal_ticks_per_second,
      false => goal_frames_per_second,
    };

    let interval = interval(Duration::from_secs_f64(1.0 / loop_helper_goal));
    let fps_reporter = RateReporter::new(Duration::from_secs(1));
    let delta_reporter = DeltaReporter::new();

    //todo: make this happen!
    println!("we need a minetest.conf parser for vsync!");

    let mut new_game = Game {
      should_close: Arc::new(RwLock::new(false)),

      goal_frames_per_second,
      goal_ticks_per_second,

      client: None,
      server: None,

      // Simply reverse these then we can plop in a server when
      // the player enters singleplayer.
      is_client: !cli.server,

      // If this is a server we don't do any client things.
      is_server: cli.server,

      interval,
      fps_reporter,
      delta_reporter,

      delta: 0.0,
      current_fps: 0.0,

      //todo: fix this when the minetest.conf parser is implemented
      vsync_mode: 0,
    };

    // We could parse the player's name instead from a file, or a first time ask. This is mutable after all.
    new_game.client = match cli.server {
      false => Some(Client::new(cli.client_name, cli.address.clone(), cli.port)),
      true => None,
    };

    // Can auto deploy server and treat this struct like a simplified dispatcher.
    new_game.server = match cli.server {
      true => Some(Server::new(cli.address, cli.port, cli.game)),
      false => None,
    };

    // Automatically elegantly stops the game when CTRL+C is hit or user terminates the process.

    let run_clone = new_game.should_close.clone();
    let _ = ctrlc::set_handler(move || {
      *run_clone.deref().write().unwrap() = true;
    });

    new_game
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

    self
      .interval
      .set_period(Duration::from_secs_f64(1.0 / new_goal));
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
  /// Stop the game loop in it's entirety.
  ///
  /// ! This shouldn't be used for anything but testing!
  ///
  pub fn shutdown_game(&mut self) {
    *self.should_close.deref().write().unwrap() = true;
  }

  ///
  /// The main loop of the game engine.
  ///
  fn main(&mut self) {
    //? Here is where the logic loop goes.

    self.delta = self.delta_reporter.report();

    // * Uncomment this to see the exact delta time.
    // println!("delta: {:.32}", self.delta);

    //* Begin server/client on_tick()

    if self.is_server {
      match &mut self.server {
        Some(server) => {
          server.on_tick(self.delta);

          if server.shutdown_is_approved() {
            self.shutdown_game()
          }
        }
        None => panic!("minetest: attempted to run a server that does not exist."),
      }
    }

    if self.is_client {
      match &mut self.client {
        Some(client) => {
          client.on_tick(self.delta);
          if client.should_quit() {
            self.shutdown_game();
          }
        }
        None => panic!("minetest: attempted to run a client that does not exist."),
      }
    }

    //* End server/client on_tick()

    //todo: make this a configuration for debugging.
    //todo: this can also be linked into the client struct to report
    //todo: the current framerate.

    if let Some(fps) = self.fps_reporter.increment_and_report() {
      self.current_fps = fps;
      let time_measurement = match self.is_client {
        true => "FPS",
        false => "TPS",
      };
      // println!("Debug {}: {}", time_measurement, self.current_fps)
      if let Some(client) = &mut self.client {
        let mut new_title = "minetest | ".to_string();
        new_title.push_str(format!("{:.1}", fps).as_str());
        new_title.push_str(" FPS");
        client.get_window_handler().set_title(&new_title);
      }
    }

    if self.vsync_mode == 0 || self.is_server {
      self.interval.tick();
    }
  }

  ///
  /// This is the actual entry point for the game.
  ///
  pub fn enter_main_loop(&mut self) {
    while !*self.should_close.deref().read().unwrap() {
      self.main();
    }
  }
}

impl Drop for Game {
  fn drop(&mut self) {
    // If this doesn't print, there's a memory leak with RC.
    println!("Minetest dropped!");
  }
}
