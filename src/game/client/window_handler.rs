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

  pub fn update(&mut self) {
    let mut event_pump = self
      .sdl_context
      .event_pump()
      .expect("minetest: SDL2 context has randomly dissappeared!");
    // poll_iter is going to keep calling poll_event until there are no more events. It's easy mode. :)
    for event in event_pump.poll_iter() {
      // I have allowed my IDE to create all possible events, so we can easily utilize them.
      match event {
        sdl2::event::Event::Quit { timestamp } => todo!(),
        sdl2::event::Event::AppTerminating { timestamp } => todo!(),
        sdl2::event::Event::AppLowMemory { timestamp } => todo!(),
        sdl2::event::Event::AppWillEnterBackground { timestamp } => todo!(),
        sdl2::event::Event::AppDidEnterBackground { timestamp } => todo!(),
        sdl2::event::Event::AppWillEnterForeground { timestamp } => todo!(),
        sdl2::event::Event::AppDidEnterForeground { timestamp } => todo!(),
        sdl2::event::Event::Display {
          timestamp,
          display_index,
          display_event,
        } => todo!(),
        sdl2::event::Event::Window {
          timestamp,
          window_id,
          win_event,
        } => todo!(),
        sdl2::event::Event::KeyDown {
          timestamp,
          window_id,
          keycode,
          scancode,
          keymod,
          repeat,
        } => todo!(),
        sdl2::event::Event::KeyUp {
          timestamp,
          window_id,
          keycode,
          scancode,
          keymod,
          repeat,
        } => todo!(),
        sdl2::event::Event::TextEditing {
          timestamp,
          window_id,
          text,
          start,
          length,
        } => todo!(),
        sdl2::event::Event::TextInput {
          timestamp,
          window_id,
          text,
        } => todo!(),
        sdl2::event::Event::MouseMotion {
          timestamp,
          window_id,
          which,
          mousestate,
          x,
          y,
          xrel,
          yrel,
        } => todo!(),
        sdl2::event::Event::MouseButtonDown {
          timestamp,
          window_id,
          which,
          mouse_btn,
          clicks,
          x,
          y,
        } => todo!(),
        sdl2::event::Event::MouseButtonUp {
          timestamp,
          window_id,
          which,
          mouse_btn,
          clicks,
          x,
          y,
        } => todo!(),
        sdl2::event::Event::MouseWheel {
          timestamp,
          window_id,
          which,
          x,
          y,
          direction,
          precise_x,
          precise_y,
        } => todo!(),
        sdl2::event::Event::JoyAxisMotion {
          timestamp,
          which,
          axis_idx,
          value,
        } => todo!(),
        sdl2::event::Event::JoyBallMotion {
          timestamp,
          which,
          ball_idx,
          xrel,
          yrel,
        } => todo!(),
        sdl2::event::Event::JoyHatMotion {
          timestamp,
          which,
          hat_idx,
          state,
        } => todo!(),
        sdl2::event::Event::JoyButtonDown {
          timestamp,
          which,
          button_idx,
        } => todo!(),
        sdl2::event::Event::JoyButtonUp {
          timestamp,
          which,
          button_idx,
        } => todo!(),
        sdl2::event::Event::JoyDeviceAdded { timestamp, which } => todo!(),
        sdl2::event::Event::JoyDeviceRemoved { timestamp, which } => todo!(),
        sdl2::event::Event::ControllerAxisMotion {
          timestamp,
          which,
          axis,
          value,
        } => todo!(),
        sdl2::event::Event::ControllerButtonDown {
          timestamp,
          which,
          button,
        } => todo!(),
        sdl2::event::Event::ControllerButtonUp {
          timestamp,
          which,
          button,
        } => todo!(),
        sdl2::event::Event::ControllerDeviceAdded { timestamp, which } => todo!(),
        sdl2::event::Event::ControllerDeviceRemoved { timestamp, which } => todo!(),
        sdl2::event::Event::ControllerDeviceRemapped { timestamp, which } => todo!(),
        sdl2::event::Event::ControllerTouchpadDown {
          timestamp,
          which,
          touchpad,
          finger,
          x,
          y,
          pressure,
        } => todo!(),
        sdl2::event::Event::ControllerTouchpadMotion {
          timestamp,
          which,
          touchpad,
          finger,
          x,
          y,
          pressure,
        } => todo!(),
        sdl2::event::Event::ControllerTouchpadUp {
          timestamp,
          which,
          touchpad,
          finger,
          x,
          y,
          pressure,
        } => todo!(),
        sdl2::event::Event::FingerDown {
          timestamp,
          touch_id,
          finger_id,
          x,
          y,
          dx,
          dy,
          pressure,
        } => todo!(),
        sdl2::event::Event::FingerUp {
          timestamp,
          touch_id,
          finger_id,
          x,
          y,
          dx,
          dy,
          pressure,
        } => todo!(),
        sdl2::event::Event::FingerMotion {
          timestamp,
          touch_id,
          finger_id,
          x,
          y,
          dx,
          dy,
          pressure,
        } => todo!(),
        sdl2::event::Event::DollarGesture {
          timestamp,
          touch_id,
          gesture_id,
          num_fingers,
          error,
          x,
          y,
        } => todo!(),
        sdl2::event::Event::DollarRecord {
          timestamp,
          touch_id,
          gesture_id,
          num_fingers,
          error,
          x,
          y,
        } => todo!(),
        sdl2::event::Event::MultiGesture {
          timestamp,
          touch_id,
          d_theta,
          d_dist,
          x,
          y,
          num_fingers,
        } => todo!(),
        sdl2::event::Event::ClipboardUpdate { timestamp } => todo!(),
        sdl2::event::Event::DropFile {
          timestamp,
          window_id,
          filename,
        } => todo!(),
        sdl2::event::Event::DropText {
          timestamp,
          window_id,
          filename,
        } => todo!(),
        sdl2::event::Event::DropBegin {
          timestamp,
          window_id,
        } => todo!(),
        sdl2::event::Event::DropComplete {
          timestamp,
          window_id,
        } => todo!(),
        sdl2::event::Event::AudioDeviceAdded {
          timestamp,
          which,
          iscapture,
        } => todo!(),
        sdl2::event::Event::AudioDeviceRemoved {
          timestamp,
          which,
          iscapture,
        } => todo!(),
        sdl2::event::Event::RenderTargetsReset { timestamp } => todo!(),
        sdl2::event::Event::RenderDeviceReset { timestamp } => todo!(),
        sdl2::event::Event::User {
          timestamp,
          window_id,
          type_,
          code,
          data1,
          data2,
        } => todo!(),
        sdl2::event::Event::Unknown { timestamp, type_ } => todo!(),
      }
    }
  }
}
