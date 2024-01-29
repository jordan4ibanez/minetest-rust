mod camera;
mod color_uniform;
mod instance_trigger;
pub mod instanced_render_matrix;
mod mesh;
mod mesh_trs_uniform;
mod render_call;
mod texture;
mod trs_projection_data;

use std::{
  collections::{HashMap, VecDeque},
  iter,
  mem::swap,
};

use glam::{UVec2, Vec3A};
use log::error;

use wgpu::{util::DeviceExt, CommandEncoder, SurfaceTexture, TextureView};
use wgpu_sdl_linker::link_wgpu_to_sdl2;

use crate::{
  file_utilities::read_file_to_string,
  game::client::render_engine::{
    instance_trigger::InstanceTrigger,
    mesh::{Mesh, Vertex},
    texture::Texture,
  },
};

use self::{
  camera::Camera, color_uniform::ColorUniform, instanced_render_matrix::InstancedRenderData,
  mesh_trs_uniform::MeshTRSUniform, render_call::RenderCall,
};

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
  camera: Camera,

  // General implementation.
  instance: wgpu::Instance,
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  //todo: replace shader with a hashmap of shaders.
  shader: wgpu::ShaderModule,

  render_pipeline_layout: wgpu::PipelineLayout,
  render_pipeline: wgpu::RenderPipeline,
  surface_format: wgpu::TextureFormat,

  // Render state memory.
  output: Option<SurfaceTexture>,
  command_encoder: Option<CommandEncoder>,
  texture_view: Option<TextureView>,
  render_command_count: u32,

  // General variables.
  config: wgpu::SurfaceConfiguration,
  size: UVec2,
  clear_color: wgpu::Color,

  // Not instanced render queue. (Individual render calls)
  render_queue: VecDeque<RenderCall>,

  // Instanced render queue and buffer.
  instanced_render_queue: HashMap<String, Vec<InstancedRenderData>>,
  instance_buffer: Option<wgpu::Buffer>,

  // Containers for wgpu data.
  meshes: HashMap<String, Mesh>,
  textures: HashMap<String, Texture>,

  mesh_trs_uniform: MeshTRSUniform,

  instance_trigger: InstanceTrigger,

  // ! TESTING VARIABLES
  color_uniform: ColorUniform,
  channel: i8,
  up: bool,
  // ! END TESTING VARIABLES
}

impl RenderEngine {
  pub fn new(window_handler: &WindowHandler) -> Self {
    // This is written verbosely so you can read what's going on easier.

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      flags: wgpu::InstanceFlags::debugging(),
      dx12_shader_compiler: wgpu::Dx12Compiler::default(),
      gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    let surface = match link_wgpu_to_sdl2(&instance, window_handler.borrow_window()) {
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
        // * this was: wgpu::Features::default()
        features: wgpu::Features::POLYGON_MODE_LINE | wgpu::Features::DEPTH_CLIP_CONTROL,
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

    // Create the pipeline layout.
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("render_pipeline_layout"),
      bind_group_layouts: &[
        // * Group tells you directly the @group(x) in the shader.
        // Group 0.
        &Texture::get_wgpu_bind_group_layout(&device),
        // Group 1.
        &Camera::get_wgpu_bind_group_layout(&device),
        // Group 2.
        &ColorUniform::get_wgpu_bind_group_layout(&device),
      ],
      push_constant_ranges: &[],
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
    let (width, height) = window_handler.borrow_window().size();

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width,
      height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      view_formats: Vec::default(),
    };

    // And the pipeline, very important!.
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        buffers: &[
          Mesh::get_wgpu_descriptor(),
          InstancedRenderData::get_wgpu_descriptor(),
        ],
        module: &shader,
        entry_point: "vs_main",
      },
      fragment: Some(wgpu::FragmentState {
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        module: &shader,
        entry_point: "fs_main",
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: None, //Some(wgpu::Face::Back),
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

    // Then actually configure the surface with the config.
    surface.configure(&device, &config);

    let clear_color = wgpu::Color {
      r: 0.1,
      g: 0.1,
      b: 0.1,
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

    let mesh_trs_uniform = MeshTRSUniform::new(&device);
    let instance_trigger = InstanceTrigger::new(&device);

    // Initial creation and updating of the Camera.
    let mut camera = Camera::new(
      Vec3A::new(0.0, 0.0, -2.0),
      65.0,
      &device,
      window_handler,
      mesh_trs_uniform.get_buffer(),
      instance_trigger.get_buffer(),
    );
    camera.build_view_projection_matrix(&device, window_handler, &queue);

    // ! TESTING
    let color_uniform = ColorUniform::new(1.0, 1.0, 1.0, &device);
    // ! END TESTING

    let mut new_render_engine = RenderEngine {
      camera,

      // General implementation.
      instance,
      surface,
      adapter,
      device,
      queue,
      shader,

      render_pipeline_layout,
      render_pipeline,
      surface_format,

      // Render state memory.
      output: None,
      command_encoder: None,
      texture_view: None,
      render_command_count: 0,

      // General variables.
      config,
      size: UVec2::new(width, height),
      clear_color,

      // Not instanced render queue. (Individual render calls)
      render_queue: VecDeque::new(),

      // Instanced render queue and buffer.
      instanced_render_queue: HashMap::new(),
      instance_buffer: None,

      // Containers for wgpu data.
      meshes: HashMap::new(),
      textures: HashMap::new(),

      mesh_trs_uniform,

      instance_trigger,

      // ! TESTING VARIABLES
      color_uniform,
      channel: 0,
      up: true,
      // ! END TESTING VARIABLES
    };

    // ! THIS IS TEMPORARY MESH DEBUGGING !
    {
      let mut new_mesh = Mesh::new("debug", "tf.jpg");
      new_mesh.push_vertex_vec(&mut vec![
        Vertex {
          position: [-0.0868241, 0.49240386, 0.0],
          texture_coordinates: [0.4131759, 0.00759614],
          color: [1.0, 0.0, 0.0],
        }, // A
        Vertex {
          position: [-0.49513406, 0.06958647, 0.0],
          texture_coordinates: [0.0048659444, 0.43041354],
          color: [0.0, 1.0, 0.0],
        }, // B
        Vertex {
          position: [-0.21918549, -0.44939706, 0.0],
          texture_coordinates: [0.28081453, 0.949397],
          color: [0.0, 0.0, 1.0],
        }, // C
        Vertex {
          position: [0.35966998, -0.3473291, 0.0],
          texture_coordinates: [0.85967, 0.84732914],
          color: [1.0, 1.0, 0.0],
        }, // D
        Vertex {
          position: [0.44147372, 0.2347359, 0.0],
          texture_coordinates: [0.9414737, 0.2652641],
          color: [1.0, 0.0, 1.0],
        }, // E
      ]);

      new_mesh.push_index_vec(&mut vec![0, 1, 4, 1, 2, 4, 2, 3, 4]);

      // * Passes in the entire device as a mutable ref to finalize the Mesh.
      new_mesh.generate_wgpu_buffers(&mut new_render_engine.device);

      // * Then we store the newly generated Mesh into our render engine.
      // * It's now owned by the render engine.
      new_render_engine.store_mesh(&new_mesh.get_name().clone(), new_mesh);

      let new_texture = Texture::new(
        "prototype_textures/tf.jpg",
        &new_render_engine.device,
        &new_render_engine.queue,
      );

      new_render_engine.store_texture(&new_texture.get_name().clone(), new_texture);
    }
    // ! END TEMPORARY MESH DEBUGGING !

    new_render_engine
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
  /// Also, the Camera's uniform is updated here.
  ///
  pub fn initialize_render(&mut self, window_handler: &WindowHandler) {
    // First update the camera in cpu and wgu memory.
    self
      .camera
      .build_view_projection_matrix(&self.device, window_handler, &self.queue);

    // Next we will write the color buffer into memory.
    // ! TODO: this might be needed in the uninstanced/instanced loop. Test this.
    self.color_uniform.write_buffer_to_wgpu(&self.queue);

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
        // If this comes up as an error in vscode, you need to switch
        // to a rust-analyzer pre-release version!
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
  /// This flushes out all draw calls and actively runs them.
  ///
  /// ! This is still a prototype !
  ///
  pub fn process_render_calls(&mut self) {
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
    let mut render_pass =
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

    render_pass.set_pipeline(&self.render_pipeline);

    // We set the instance buffer to be nothing for not instanced render calls.
    // This blank_data must match our lifetime.
    let blank_data = InstancedRenderData::get_blank_data();
    self.instance_buffer = Some(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&blank_data),
        usage: wgpu::BufferUsages::VERTEX,
      },
    ));

    // Activate the camera's bind group.
    render_pass.set_bind_group(1, self.camera.get_bind_group(), &[]);

    // Activate the color bind group.
    render_pass.set_bind_group(2, self.color_uniform.get_bind_group(), &[]);

    while !self.render_queue.is_empty() {
      let not_instanced_render_call = self.render_queue.pop_front().unwrap();

      let mesh_name = not_instanced_render_call.get_mesh_name();

      match self.meshes.get(mesh_name) {
        Some(mesh) => {
          let texture_name = not_instanced_render_call.get_texture_name();

          match self.textures.get(texture_name) {
            Some(texture) => {
              // Now activate the used texture's bind group.
              render_pass.set_bind_group(0, texture.get_wgpu_diffuse_bind_group(), &[]);

              self
                .mesh_trs_uniform
                .set_translation(not_instanced_render_call.get_translation());
              self
                .mesh_trs_uniform
                .set_rotation(not_instanced_render_call.get_rotation());
              self
                .mesh_trs_uniform
                .set_scale(not_instanced_render_call.get_scale());

              self
                .mesh_trs_uniform
                .build_mesh_projection_matrix(&self.device, &self.queue);

              // Now we're going to bind the pipeline to the Mesh and draw it.

              render_pass.set_vertex_buffer(0, mesh.get_wgpu_vertex_buffer().slice(..));
              render_pass.set_vertex_buffer(1, self.instance_buffer.as_ref().unwrap().slice(..));

              render_pass.set_index_buffer(
                mesh.get_wgpu_index_buffer().slice(..),
                wgpu::IndexFormat::Uint32,
              );

              render_pass.draw_indexed(0..mesh.get_number_of_indices(), 0, 0..1);
            }
            None => error!("render_engine: {} is not a stored Texture.", texture_name),
          }
        }
        None => error!("render_engine: {} is not a stored Mesh.", mesh_name),
      }
    }
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

    // Clear out the instance call hashmap memory to prevent a memory leak.
    self.instanced_render_queue.clear();

    // Finally, the texture view is outdated, destroy it.

    self.texture_view = None;

    // For now, we'll ensure that this is unchanged. [ validation ]
    assert!(self.command_encoder.is_none());
    assert!(self.output.is_none());
    assert!(self.texture_view.is_none());
  }

  ///
  /// Store a Mesh into the render engine for usage.
  ///
  pub fn store_mesh(&mut self, name: &str, mesh: Mesh) {
    self.meshes.insert(name.to_owned(), mesh);
  }

  ///
  /// Store a Texture into the render engine for usage.
  ///
  pub fn store_texture(&mut self, name: &str, texture: Texture) {
    self.textures.insert(name.to_owned(), texture);
  }

  ///
  /// Render a mesh not instanced.
  ///
  pub fn render_mesh(
    &mut self,
    mesh_name: &str,
    texture_name: &str,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) {
    self.render_queue.push_back(RenderCall::new(
      mesh_name,
      texture_name,
      translation,
      rotation,
      scale,
    ))
  }

  ///
  /// Push one instance call into the instance queue.
  ///
  /// This is less efficient than render_mesh_instanced because
  /// it needs to check if the key exists every time.
  ///
  pub fn render_mesh_instanced_single(
    &mut self,
    mesh_name: &str,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) {
    // If the key does not exist, we create it.
    let current_vec = self
      .instanced_render_queue
      .entry(mesh_name.to_string())
      .or_default();

    // Now push one into the vector.
    current_vec.push(InstancedRenderData::new(translation, rotation, scale));
  }

  ///
  /// Push multiple instance calls into the instance queue.
  ///
  pub fn render_mesh_instanced(
    &mut self,
    mesh_name: &str,
    instancing: &mut Vec<InstancedRenderData>,
  ) {
    // If the key does not exist, we create it.
    let current_vec = self
      .instanced_render_queue
      .entry(mesh_name.to_string())
      .or_default();

    // Now append multiple into the vector.
    current_vec.append(instancing);
  }

  ///
  /// Grab the Camera mutably to do things with it.
  ///
  pub fn get_camera(&mut self) -> &mut Camera {
    &mut self.camera
  }

  ///
  /// Run all required update procedures on the RenderEngine.
  ///
  pub fn update(&mut self, window_handler: &WindowHandler, delta: f64) {
    self.update_size(window_handler.get_size());
    // self.trollface_rave(delta);
    // self.test_implementation(window_handler);
  }
}
