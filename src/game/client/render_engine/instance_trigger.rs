use wgpu::util::DeviceExt;

///
/// A very, very simple uniform which allows us to turn on or off instancing.
///
/// It utilizes 1 and 0 for true and false.
///
/// I would have used a boolean, but that has alignment issues.
///
/// It also needs to slice, that's why it's wrappered in an array.
///
// We need this for Rust to store our data correctly for the shaders.
#[repr(C)]
// This is so we can store this in a buffer.
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceTriggerUniform {
  enabled: [i32; 1],
}

///
/// Handles and encapsulates the logic for the InstanceTriggerUniform.
///
pub struct InstanceTrigger {
  instance_trigger_uniform: InstanceTriggerUniform,
  instance_trigger_buffer: wgpu::Buffer,
}

impl InstanceTrigger {
  pub fn new(device: &wgpu::Device) -> Self {
    let instance_trigger_uniform = InstanceTriggerUniform { enabled: [0] };

    let instance_trigger_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("instance_trigger_buffer"),
      contents: bytemuck::cast_slice(&instance_trigger_uniform.enabled),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    InstanceTrigger {
      instance_trigger_uniform,
      instance_trigger_buffer,
    }
  }

  ///
  /// Turn on instancing in the shader.
  ///
  pub fn trigger_on(&mut self, queue: &wgpu::Queue) {
    self.instance_trigger_uniform.enabled[0] = 1;

    queue.write_buffer(self.get_buffer(), 0, self.get_raw_wgpu_data());
  }

  ///
  /// Turn off instancing in the shader.
  ///
  pub fn trigger_off(&mut self, queue: &wgpu::Queue) {
    self.instance_trigger_uniform.enabled[0] = 0;

    queue.write_buffer(self.get_buffer(), 0, self.get_raw_wgpu_data());
  }

  ///
  /// Grab the raw bytes to pass into wgpu.
  ///
  fn get_raw_wgpu_data(&self) -> &[u8] {
    bytemuck::cast_slice(&self.instance_trigger_uniform.enabled)
  }

  ///
  /// Get the InstanceTrigger's wgpu buffer.
  ///
  pub fn get_buffer(&self) -> &wgpu::Buffer {
    &self.instance_trigger_buffer
  }
}
