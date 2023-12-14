pub struct Server {
  // needs some kind of lock, maybe
  cool: bool
}

impl Server {
  pub fn new() {
    Server {
      cool: true
    }
  }
}