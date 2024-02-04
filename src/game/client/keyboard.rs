use ahash::AHashMap;

pub struct KeyboardController {
  keys: AHashMap<String, bool>,
}

impl KeyboardController {
  pub fn new() -> Self {
    KeyboardController {
      keys: AHashMap::new(),
    }
  }

  ///
  /// Simply dumps a key's state into the memory.
  ///
  pub fn set_key(&mut self, key_name: &str, pressed: bool) {
    self.keys.insert(key_name.to_owned(), pressed);

    println!("{} is pressed? {}", key_name, pressed);
  }

  ///
  /// Checks if a key is down, if it was never pressed, it's down.
  ///
  pub fn is_key_down(&self, key_name: &str) -> bool {
    match self.keys.get(key_name) {
      Some(key_down) => *key_down,
      None => false,
    }
  }

  // * future note: this can poll for key pressed. Simply store memory with an update.
}
