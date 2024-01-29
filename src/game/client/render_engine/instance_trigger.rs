///
/// A very, very simple uniform which allows us to turn on or off instancing.
///
/// It utilizes 1 and 0 for true and false.
///
// We need this for Rust to store our data correctly for the shaders.
#[repr(C)]
// This is so we can store this in a buffer.
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceTriggerUniform {
  enabled: i32,
}

///
/// Handles and encapsulates the logic for the InstanceTriggerUniform.
///
pub struct InstanceTrigger {}

impl InstanceTrigger {
  pub fn new() -> Self {
    InstanceTrigger {}
  }
}
