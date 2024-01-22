// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorUniform {
  rgb: [f32; 4],
}

impl ColorUniform {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    ColorUniform {
      rgb: [r, g, b, 1.0],
    }
  }

  pub fn get_r(&self) -> f32 {
    self.rgb[0]
  }

  pub fn set_r(&mut self, r: f32) {
    self.rgb[0] = r;
  }

  pub fn get_g(&self) -> f32 {
    self.rgb[1]
  }

  pub fn set_g(&mut self, g: f32) {
    self.rgb[1] = g;
  }

  pub fn get_b(&self) -> f32 {
    self.rgb[2]
  }

  pub fn set_b(&mut self, b: f32) {
    self.rgb[2] = b;
  }

  pub fn get_wgpu_raw_data(&self) -> &[u8] {
    bytemuck::cast_slice(&self.rgb)
  }

  ///
  /// Get the wgpu bind group layout to tell wgpu how to use the buffer.
  ///
  pub fn get_wgpu_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
      label: Some("color_bind_group_layout"),
    })
  }
}
