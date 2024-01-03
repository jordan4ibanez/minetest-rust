use sdl2::Sdl;

///
/// SDL2 window controller.
///
/// This is a wrapper around SDL2.
///
/// This can be renamed to SdlWrapper if we find ourselves
/// using more components of it than originally intended.
///
pub struct WindowHandler {
  sdl_context: Option<Sdl>,
}
