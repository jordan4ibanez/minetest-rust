use sdl2::{hint, video::Window, Sdl, VideoSubsystem};

///
/// SDL2 window controller.
///
/// This is a wrapper around SDL2.
///
/// This also implements state management for SDL2 components
/// that don't have retrievable state.
///
/// This can be renamed to SdlWrapper if we find ourselves
/// using more components of it than originally intended.
///
pub struct WindowHandler {
  sdl_context: Sdl,
  video_subsystem: VideoSubsystem,
  window: Window,

  visible: bool,
}

impl WindowHandler {
  pub fn new() -> Self {
    // We're going to do this line by line,
    // in case any of this fails.

    // We want to use wgpu as our rendering multiplexer, disable OpenGL.
    hint::set("SDL_VIDEO_EXTERNAL_CONTEXT", "1");

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
      .window("minetest", 512, 512)
      .resizable()
      .position_centered()
      .allow_highdpi()
      .build()
      .unwrap();

    let mut new_window_handler = WindowHandler {
      sdl_context,
      video_subsystem,
      visible: false,
      window,
    };

    new_window_handler.show();

    new_window_handler
  }

  pub fn show(&mut self) {
    self.visible = true;
    self.window.show();
  }

  pub fn hide(&mut self) {
    self.visible = true;
    self.window.show();
  }
}
