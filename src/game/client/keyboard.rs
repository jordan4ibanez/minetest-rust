use std::collections::HashMap;

pub struct KeyboardController {
  keys: HashMap<String, bool>,
}

impl KeyboardController {
  pub fn new() -> Self {
    KeyboardController {
      keys: HashMap::new(),
    }
  }
}
