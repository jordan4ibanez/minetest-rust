///
/// I named these really weirdly so they do not conflict with
/// anything from SDL2.
///
/// PressingDown: The user is now pressing this key.
///
/// LiftedOff: The user is no longer pressing this key.
///
/// It is quite simple, but makes it just that much easier to
/// work with key events in SDL2.
///
pub enum KeyEvent {
  PressingDown,
  LiftedOff,
}

impl KeyEvent {
  pub fn is_up(&self) -> bool {
    matches!(self, KeyEvent::LiftedOff)
  }

  pub fn is_down(&self) -> bool {
    matches!(self, KeyEvent::PressingDown)
  }
}
