use wgpu::util::DeviceExt;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorRGBA {
  rgb: [f32; 4],
}

///
/// ColorRGBA encapsulates all utility for working with the color uniform.
///
impl ColorRGBA {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    ColorRGBA {
      rgb: [r, g, b, 1.0],
    }
  }
}

pub struct ColorUniform {
  color_rgba: ColorRGBA,
  color_buffer: wgpu::Buffer,
  color_bind_group: wgpu::BindGroup,
}
impl ColorUniform {
  pub fn new(r: f32, g: f32, b: f32, device: &wgpu::Device) -> Self {
    let color_rgba = ColorRGBA::new(r, g, b);

    // Now we create the Color buffer.
    let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("color_buffer"),
      contents: bytemuck::cast_slice(&color_rgba.rgb),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &ColorUniform::get_wgpu_bind_group_layout(device),
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: color_buffer.as_entire_binding(),
      }],
      label: Some("color_bind_group"),
    });

    ColorUniform {
      color_rgba,
      color_buffer,
      color_bind_group,
    }
  }

  ///
  /// Get the red channel. [0.0-1.0] f32.
  ///
  pub fn get_r(&self) -> f32 {
    self.color_rgba.rgb[0]
  }

  ///
  /// Set the red channel. [0.0-1.0] f32.
  ///
  pub fn set_r(&mut self, r: f32) {
    self.color_rgba.rgb[0] = r;
  }

  ///
  /// Get the green channel. [0.0-1.0] f32.
  ///
  pub fn get_g(&self) -> f32 {
    self.color_rgba.rgb[1]
  }

  ///
  /// Set the green channel. [0.0-1.0] f32.
  ///
  pub fn set_g(&mut self, g: f32) {
    self.color_rgba.rgb[1] = g;
  }

  ///
  /// Get the blue channel. [0.0-1.0] f32.
  ///
  pub fn get_b(&self) -> f32 {
    self.color_rgba.rgb[2]
  }

  ///
  /// Set the blue channel. [0.0-1.0] f32.
  ///
  pub fn set_b(&mut self, b: f32) {
    self.color_rgba.rgb[2] = b;
  }

  ///
  /// ! Internal only get RGB data array as bytes for wgpu.
  ///
  fn get_wgpu_raw_data(&self) -> &[u8] {
    bytemuck::cast_slice(&self.color_rgba.rgb)
  }

  ///
  /// Write the RGBA color memory in to wgpu.
  ///
  pub fn write_buffer_to_wgpu(&self, queue: &wgpu::Queue) {
    queue.write_buffer(&self.color_buffer, 0, self.get_wgpu_raw_data());
  }

  ///
  /// Get the wgpu bind group for rendering.
  ///
  pub fn get_bind_group(&self) -> &wgpu::BindGroup {
    &self.color_bind_group
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
