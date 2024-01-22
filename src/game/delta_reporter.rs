use std::time::Instant;

///
/// DeltaTime is a micro struct which encapsulates logic
/// for tracking the delta time between loop ticks.
///
pub struct DeltaReporter {
  old_time: Instant,
}

impl DeltaReporter {
  pub fn new() -> Self {
    DeltaReporter {
      old_time: Instant::now(),
    }
  }

  ///
  /// Get the time in seconds as f64 since the last loop.
  ///
  pub fn report(&mut self) -> f64 {
    let delta = self.old_time.elapsed().as_secs_f64();
    self.old_time = Instant::now();
    delta
  }
}
