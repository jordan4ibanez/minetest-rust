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
  /// You can thank the creator of spin_sleep alexheretic for helping me micro opt this!
  ///
  pub fn report(&mut self) -> f64 {
    let now = Instant::now();
    let delta = now.duration_since(self.old_time).as_secs_f64();
    self.old_time = now;
    delta
  }
}
