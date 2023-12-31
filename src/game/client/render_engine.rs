use std::{iter, mem::swap};

use glam::UVec2;
use sdl2::video::Window;
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use wgpu_sdl_linker::link_wgpu_to_sdl2;

use crate::file_utilities::read_file_to_string;

use super::window_handler::WindowHandler;

///
/// The main rendering engine for the game.
///
/// This is the meat and potatoes of the game
/// as far players are concerned.
///
/// Utilizes wgpu as the main driving force to render.
///
pub struct RenderEngine {
  instance: wgpu::Instance,
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  //todo: replace shader with a hashmap of shaders.
  shader: wgpu::ShaderModule,
  bind_group_layout: wgpu::BindGroupLayout,
  bind_group: wgpu::BindGroup,
  pipeline_layout: wgpu::PipelineLayout,
  render_pipeline: wgpu::RenderPipeline,
  surface_format: wgpu::TextureFormat,

  output: Option<SurfaceTexture>,
  command_encoder: Option<CommandEncoder>,
  texture_view: Option<TextureView>,

  config: wgpu::SurfaceConfiguration,
  size: UVec2,
  clear_color: wgpu::Color,
}

impl RenderEngine {
  pub fn new(window: &Window) -> Self {
    // This is written verbosely so you can read what's going on easier.

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      flags: wgpu::InstanceFlags::debugging(),
      dx12_shader_compiler: wgpu::Dx12Compiler::default(),
      gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    let surface = match link_wgpu_to_sdl2(&instance, window) {
      Ok(new_surface) => new_surface,
      Err(e) => panic!("{}", e),
    };

    // We must block the main thread while this completes or things can go crazy.
    // This is waiting for a future.
    let adapter_option =
      // ! Block on might cause a crash in WASM !
      pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      }));

    let adapter = match adapter_option {
      Some(new_adapter) => new_adapter,
      None => panic!("minetest: no graphics adapter found!"),
    };

    // We must block the main thread while this completes or things can go crazy.
    // This is waiting for a future.
    // ! Block on might cause a crash in WASM !
    let (device, queue) = match pollster::block_on(adapter.request_device(
      &wgpu::DeviceDescriptor {
        limits: wgpu::Limits::default(),
        label: Some("minetest_gpu"),
        features: wgpu::Features::default(),
      },
      None,
    )) {
      Ok(device_and_queue) => device_and_queue,
      Err(e) => panic!("{}", e),
    };

    // Load up the default shader source code.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("default_shader"),
      source: wgpu::ShaderSource::Wgsl(read_file_to_string("shaders/default_shader.wgsl").into()),
    });

    // Create bind group components.
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[],
      label: Some("bind_group_layout"),
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      entries: &[],
      label: Some("bind_group"),
    });

    // Create the pipeline layout.
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&bind_group_layout],
      label: None,
      push_constant_ranges: &[],
    });

    // And the pipeline, very important!.
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        buffers: &[],
        module: &shader,
        entry_point: "vs_main",
      },
      fragment: Some(wgpu::FragmentState {
        targets: &[Some(wgpu::ColorTargetState {
          format: wgpu::TextureFormat::Bgra8UnormSrgb,
          blend: None,
          write_mask: wgpu::ColorWrites::ALL,
        })],
        module: &shader,
        entry_point: "fs_main",
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Front),
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
      },
      depth_stencil: None,
      label: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None,
    });

    // Surface capabilities.
    let surface_caps = surface.get_capabilities(&adapter);

    // And the surface format.
    let surface_format = surface_caps
      .formats
      .iter()
      .copied()
      // This may not be thorough enough to get the format we want.
      .find(|f| f.is_srgb())
      .unwrap_or(surface_caps.formats[0]);

    // Need to get the window size to configure the surface.
    let (width, height) = window.size();

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width,
      height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      view_formats: Vec::default(),
    };

    // Then actually configure the surface with the config.
    surface.configure(&device, &config);

    let clear_color = wgpu::Color {
      r: 0.5,
      g: 0.0,
      b: 0.5,
      a: 1.0,
    };

    // Print out device and backend.
    println!(
      "RenderEngine: Chosen device name: {}",
      adapter.get_info().name
    );
    println!(
      "RenderEngine: Running on {} backend.",
      adapter.get_info().backend.to_str()
    );

    RenderEngine {
      // General implementation.
      instance,
      surface,
      adapter,
      device,
      queue,
      shader,
      bind_group_layout,
      bind_group,
      pipeline_layout,
      render_pipeline,
      surface_format,

      // Render state memory.
      output: None,
      command_encoder: None,
      texture_view: None,

      // General variables.
      config,
      size: UVec2::new(width, height),
      clear_color,
    }
  }

  ///
  /// Automatically resizes the surface from the passed through UVec2.
  ///
  /// We simply check if the size is different, then update the surface
  /// config and update the surface with it.
  ///
  fn update_size(&mut self, new_size: &UVec2) {
    if self.size != *new_size {
      // Update internal size
      self.size = *new_size;

      println!("RenderEngine: SURFACE UPDATE! {:?}", self.size);

      // Now update the config.
      self.config.width = self.size.x;
      self.config.height = self.size.y;

      // Finally, reconfigure the surface with the config.
      self.surface.configure(&self.device, &self.config);
    }
  }

  ///
  /// Initialize the render state.
  ///
  /// This simply sets everything up.
  ///
  pub fn initialize_render(&mut self) {
    self.output = Some(
      self
        .surface
        .get_current_texture()
        .expect("minetest: wgpu surface texture does not exist!"),
    );

    self.texture_view = Some(
      self
        .output
        .as_mut()
        .unwrap()
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default()),
    );

    self.command_encoder = Some(self.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor {
        label: Some("minetest_renderer"),
      },
    ));
  }

  ///
  /// Run the render procedure on the RenderEngine.
  ///
  /// todo: this will draw things in the future, probably automatically.
  ///
  pub fn render(&mut self) {
    // Do 3 very basic checks before attempting to render.
    if self.output.is_none() {
      panic!("RenderEngine: attempted to render with no output!");
    }

    if self.command_encoder.is_none() {
      panic!("RenderEngine: attempted render with no command encoder!");
    }

    if self.texture_view.is_none() {
      panic!("RenderEngine: attempted to render with no texture view!");
    }

    // Begin a wgpu render pass
    self
      .command_encoder
      .as_mut()
      .unwrap()
      .begin_render_pass(&wgpu::RenderPassDescriptor {
        // The label of this render pass.
        label: Some("minetest_render_pass"),

        // color attachments is a array of pipeline render pass color attachments.
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: self.texture_view.as_ref().unwrap(),
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.clear_color),
            store: wgpu::StoreOp::Store,
          },
        })],

        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
      });
  }

  ///
  /// Submits all commands and flushes the texture buffer into the SDL2 window.
  ///
  pub fn finalize_render(&mut self) {
    // First let's swap the command encoder out into a local variable. That's now flushed into None.

    let mut final_encoder: Option<CommandEncoder> = None;

    swap(&mut final_encoder, &mut self.command_encoder);

    self
      .queue
      .submit(iter::once(final_encoder.unwrap().finish()));

    // Next we simply swap the surface out into a local variable. We've just flushed the surface out into None.

    let mut final_output: Option<SurfaceTexture> = None;

    swap(&mut final_output, &mut self.output);

    final_output.unwrap().present();

    // Finally, the texture view is outdated, destroy it.

    self.texture_view = None;

    // For now, we'll ensure that this is unchanged. [ validation ]
    assert!(self.command_encoder.is_none());
    assert!(self.output.is_none());
    assert!(self.texture_view.is_none());
  }

  /// !remove me!
  ///
  /// A test of combining the window handler with the clear color.
  ///
  /// Simply changes the clear color based on the x and y of the mouse.
  ///
  /// !remove me!
  fn test_implementation(&mut self, window_handler: &WindowHandler) {
    let width = window_handler.get_size().x as f64;
    let progress_x = window_handler.get_mouse_position().x as f64;

    let height = window_handler.get_size().y as f64;
    let progress_y = window_handler.get_mouse_position().y as f64;

    let mut red_color = progress_x / width;
    if red_color.is_infinite() {
      red_color = 0.0;
    }

    let mut blue_color = progress_y / height;
    if blue_color.is_infinite() {
      blue_color = 0.0;
    }

    let old_clear = self.clear_color;

    self.clear_color.r = red_color;
    self.clear_color.b = blue_color;

    if old_clear != self.clear_color {
      println!("clear color updated! {:?}", self.clear_color);
    }
  }

  ///
  /// Run all required update procedures on the RenderEngine.
  ///
  pub fn update(&mut self, window_handler: &WindowHandler, delta: f64) {
    self.update_size(window_handler.get_size());

    self.test_implementation(window_handler);
  }
}
