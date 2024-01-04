use sdl2::{
  hint,
  video::{FullscreenType, Window},
  Sdl, VideoSubsystem,
};

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

  ///
  /// Borrow the WindowHandler's actual window.
  ///
  pub fn borrow_window(&self) -> &Window {
    &self.window
  }

  ///
  /// Make the window visible.
  ///
  pub fn show(&mut self) {
    self.visible = true;
    self.window.show();
  }

  ///
  /// Hide the window
  ///
  pub fn hide(&mut self) {
    self.visible = true;
    self.window.show();
  }

  ///
  /// Set the window title.
  ///
  pub fn set_title(&mut self, new_title: &str) {
    // If something goes wrong, let it crash.
    self.window.set_title(new_title).unwrap();
  }

  ///
  /// Get the window title.
  ///
  pub fn get_title(&self) -> &str {
    self.window.title()
  }

  ///
  /// Get if the window is in real fullscreen mode.
  ///
  pub fn is_fullscreen_real_mode(&mut self) -> bool {
    self.window.fullscreen_state() == FullscreenType::True
  }

  ///
  /// Get if the window is in fake borderless fullscreen mode.
  ///
  pub fn is_fullscreen_borderless_mode(&mut self) -> bool {
    self.window.fullscreen_state() == FullscreenType::Desktop
  }

  ///
  /// Get if the window is in ANY fullscreen mode.
  ///
  pub fn is_fullscreen_any_mode(&mut self) -> bool {
    matches!(
      self.window.fullscreen_state(),
      FullscreenType::True | FullscreenType::Desktop
    )
  }

  ///
  /// Set the window to real fullscreen mode.
  ///
  pub fn set_fullscreen_real_mode(&mut self) {
    self.window.set_fullscreen(FullscreenType::True).unwrap()
  }

  ///
  /// Set the window to fake borderless fullscreen mode.
  ///
  pub fn set_fullscreen_borderless_mode(&mut self) {
    self.window.set_fullscreen(FullscreenType::Desktop).unwrap()
  }

  ///
  /// Set the window to normal windowed mode. (not fullscreen)
  ///
  pub fn set_windowed_mode(&mut self) {
    self.window.set_fullscreen(FullscreenType::Off).unwrap()
  }
}
