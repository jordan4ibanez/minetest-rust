mod camera;
mod color_uniform;
mod depth_buffer;
mod instance_trigger;
pub mod instanced_render_matrix;
mod mesh;
mod mesh_trs_uniform;
mod model;
mod model_loader;
mod render_call;
mod texture;
mod trs_projection_data;

use std::{collections::VecDeque, iter, mem::swap};

use ahash::AHashMap;
use glam::{UVec2, Vec3A};
use log::error;

use wgpu::{util::DeviceExt, CommandEncoder, SurfaceTexture, TextureView};
use wgpu_sdl_linker::link_wgpu_to_sdl2;

use crate::{
  file_utilities::read_file_to_string,
  game::client::render_engine::{
    instance_trigger::InstanceTrigger,
    mesh::{Mesh, Vertex},
    model_loader::ModelLoader,
    texture::Texture,
  },
};

use self::{
  camera::Camera,
  color_uniform::ColorUniform,
  depth_buffer::DepthBuffer,
  instanced_render_matrix::InstancedRenderData,
  mesh_trs_uniform::MeshTRSUniform,
  model::Model,
  render_call::{ModelRenderCall, RenderCall},
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
  //todo: replace shader with a AHashMap of shaders.
  shader: wgpu::ShaderModule,

  render_pipeline_layout: wgpu::PipelineLayout,
  render_pipeline: wgpu::RenderPipeline,
  surface_format: wgpu::TextureFormat,

  // Render state memory.
  output: Option<SurfaceTexture>,
  command_encoder: Option<CommandEncoder>,
  texture_view: Option<TextureView>,
  depth_buffer: Option<DepthBuffer>,
  render_command_count: u32,

  // General variables.
  config: wgpu::SurfaceConfiguration,
  size: UVec2,
  clear_color: wgpu::Color,

  // Not instanced render queues. (Individual render calls)
  mesh_render_queue: VecDeque<RenderCall>,
  model_render_queue: VecDeque<ModelRenderCall>,

  // Instanced render queues and buffer.
  instanced_mesh_render_queue: AHashMap<String, Vec<InstancedRenderData>>,
  instance_buffer: Option<wgpu::Buffer>,
  instance_trigger: InstanceTrigger,

  // Containers for wgpu data.
  meshes: AHashMap<String, Mesh>,
  textures: AHashMap<String, Texture>,

  models: AHashMap<String, Model>,

  mesh_trs_uniform: MeshTRSUniform,

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
        // Backface culling.
        cull_mode: Some(wgpu::Face::Back),
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
      },
      depth_stencil: Some(wgpu::DepthStencilState {
        format: DepthBuffer::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
      }),
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
      depth_buffer: None,
      render_command_count: 0,

      // General variables.
      config,
      size: UVec2::new(width, height),
      clear_color,

      // Not instanced render queues. (Individual render calls)
      mesh_render_queue: VecDeque::new(),
      model_render_queue: VecDeque::new(),

      // Instanced render queues and buffer.
      instanced_mesh_render_queue: AHashMap::new(),
      instance_buffer: None,
      instance_trigger,

      // Containers for wgpu data.
      meshes: AHashMap::new(),
      textures: AHashMap::new(),
      models: AHashMap::new(),

      mesh_trs_uniform,

      // ! TESTING VARIABLES
      color_uniform,
      channel: 0,
      up: true,
      // ! END TESTING VARIABLES
    };

    // ! THIS IS TEMPORARY MESH DEBUGGING !
    {
      let mut new_mesh = Mesh::new("debug");
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
        "prototype_textures/tf.webp",
        &new_render_engine.device,
        &new_render_engine.queue,
      );

      new_render_engine.store_texture(&new_texture.get_name().clone(), new_texture);

      // ? BEGIN DEBUGGING MODEL LOADER ?

      // ! CHAIR

      let chair_model = ModelLoader::load_model(
        "./prototype_models/chair.obj",
        &new_render_engine.device,
        &new_render_engine.queue,
      )
      .unwrap();

      new_render_engine
        .models
        .insert(chair_model.name.clone(), chair_model);

      let chair_texture = Texture::new(
        "prototype_textures/chair.png",
        &new_render_engine.device,
        &new_render_engine.queue,
      );

      new_render_engine.store_texture(&chair_texture.get_name().clone(), chair_texture);

      // ! SNOWMAN

      let snowman = ModelLoader::load_model(
        "./prototype_models/snowman.obj",
        &new_render_engine.device,
        &new_render_engine.queue,
      )
      .unwrap();

      new_render_engine
        .models
        .insert(snowman.name.clone(), snowman);

      let snowman_texture = Texture::new(
        "./prototype_textures/snowman.png",
        &new_render_engine.device,
        &new_render_engine.queue,
      );

      new_render_engine.store_texture(&snowman_texture.get_name().clone(), snowman_texture);

      // ? END DEBUGGING MODEL LOADER ?
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

      // println!("RenderEngine: SURFACE UPDATE! {:?}", self.size);

      // Now update the config.
      self.config.width = self.size.x;
      self.config.height = self.size.y;

      // Finally, reconfigure the surface with the config.
      self.surface.configure(&self.device, &self.config);
    }
  }

  ///
  /// This simply updates the Camera's uniform projection matrix.
  ///
  pub fn update_camera_matrix(&mut self, window_handler: &WindowHandler) {
    // First update the camera in cpu and wgu memory.
    self
      .camera
      .build_view_projection_matrix(&self.device, window_handler, &self.queue);

    // Next we will write the color buffer into memory.
    // ! TODO: this might be needed in the uninstanced/instanced loop. Test this.
    self.color_uniform.write_buffer_to_wgpu(&self.queue);
  }

  ///
  /// Generate the texture used to output the pixel data into the window.
  ///
  /// Aka, the framebuffer.
  ///
  pub fn generate_frame_buffer(&mut self) {
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

    self.depth_buffer = Some(DepthBuffer::new(&self.device, &self.config, "depth_buffer"));
  }

  ///
  /// Initialize the render state.
  ///
  /// This simply sets everything up.
  ///
  pub fn initialize_render(&mut self) {
    self.command_encoder = Some(self.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor {
        label: Some("minetest_command_encoder"),
      },
    ));
  }

  ///
  /// Internal raw command to clear the buffers.
  ///
  /// Done like this because polonius wasn't out when this was written.
  ///
  fn clear_buffers_raw_command(&mut self, depth: bool, color: bool) {
    // Do 4 very basic checks before attempting to render.
    if self.output.is_none() {
      panic!("RenderEngine: attempted to render with no output!");
    }

    if self.command_encoder.is_none() {
      panic!("RenderEngine: attempted render with no command encoder!");
    }

    if self.texture_view.is_none() {
      panic!("RenderEngine: attempted to render with no texture view!");
    }

    if self.depth_buffer.is_none() {
      panic!("RenderEngine: attempted to render with no depth buffer!");
    }

    let clear_color = if color {
      wgpu::LoadOp::Clear(self.clear_color)
    } else {
      wgpu::LoadOp::Load
    };

    let clear_depth = if depth {
      wgpu::LoadOp::Clear(1.0)
    } else {
      wgpu::LoadOp::Load
    };

    // Begin a wgpu render pass
    let mut render_pass =
      self
        .command_encoder
        .as_mut()
        .unwrap()
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          // The label of this render pass.
          label: Some("minetest_clear_buffers_render_pass"),

          // color attachments is a array of pipeline render pass color attachments.
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: self.texture_view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
              load: clear_color,
              store: wgpu::StoreOp::Store,
            },
          })],

          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: self.depth_buffer.as_ref().unwrap().get_view(),
            depth_ops: Some(wgpu::Operations {
              load: clear_depth,
              store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
          }),
          occlusion_query_set: None,
          timestamp_writes: None,
        });

    render_pass.set_pipeline(&self.render_pipeline);
  }

  ///
  /// You can clear the depth buffer and the color buffer with this.
  ///
  /// One or the other, or both. Or none, if you're feeling ridiculous.
  ///
  pub fn clear_buffers(&mut self, depth: bool, color: bool) {
    self.initialize_render();
    self.clear_buffers_raw_command(true, true);
    self.submit_render();
  }

  ///
  /// Processes the next available render call in the Mesh queue.
  ///
  fn process_not_instanced_mesh_render_call(&mut self) {
    // Do 4 very basic checks before attempting to render.
    if self.output.is_none() {
      panic!("RenderEngine: attempted to render with no output!");
    }

    if self.command_encoder.is_none() {
      panic!("RenderEngine: attempted render with no command encoder!");
    }

    if self.texture_view.is_none() {
      panic!("RenderEngine: attempted to render with no texture view!");
    }

    if self.depth_buffer.is_none() {
      panic!("RenderEngine: attempted to render with no depth buffer!");
    }

    // * Begin not instanced render calls. [MESH]
    // Begin a wgpu render pass
    let mut render_pass =
      self
        .command_encoder
        .as_mut()
        .unwrap()
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          // The label of this render pass.
          label: Some("minetest_not_instanced_mesh_render_pass"),

          // color attachments is a array of pipeline render pass color attachments.
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: self.texture_view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],

          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: self.depth_buffer.as_ref().unwrap().get_view(),
            depth_ops: Some(wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
          }),
          occlusion_query_set: None,
          timestamp_writes: None,
        });

    render_pass.set_pipeline(&self.render_pipeline);

    // Activate the camera's bind group.
    render_pass.set_bind_group(1, self.camera.get_bind_group(), &[]);

    // Activate the color bind group.
    render_pass.set_bind_group(2, self.color_uniform.get_bind_group(), &[]);

    // We set the instance buffer to be nothing for not instanced render calls.
    // This blank_data must match our lifetime.
    let blank_data = InstancedRenderData::get_blank_data();
    self.instance_buffer = Some(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("instance_buffer"),
        contents: bytemuck::cast_slice(&blank_data),
        usage: wgpu::BufferUsages::VERTEX,
      },
    ));

    // Disable instancing in shader.
    self.instance_trigger.trigger_off(&self.queue);

    let not_instanced_render_call = self.mesh_render_queue.pop_front().unwrap();

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
          None => error!(
            "render_engine: {} is not a stored Texture. [not instanced]",
            texture_name
          ),
        }
      }
      None => error!(
        "render_engine: {} is not a stored Mesh. [not instanced]",
        mesh_name
      ),
    }
  }

  ///
  /// Process and run all Mesh render calls.
  ///
  fn process_not_instanced_mesh_render_calls(&mut self) {
    while !self.mesh_render_queue.is_empty() {
      self.initialize_render();
      self.process_not_instanced_mesh_render_call();
      self.submit_render();
    }
  }

  ///
  /// Processes the next available render call in the Model queue.
  ///
  fn process_not_instanced_model_render_call(&mut self) {
    // * Begin not instanced render calls. [MODEL]
    // ? note: if you can find a way to draw all this in one render pass, open a PR immediately.
    // Begin a wgpu render pass
    let mut render_pass =
      self
        .command_encoder
        .as_mut()
        .unwrap()
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          // The label of this render pass.
          label: Some("minetest_not_instanced_model_render_pass"),

          // color attachments is a array of pipeline render pass color attachments.
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: self.texture_view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],

          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: self.depth_buffer.as_ref().unwrap().get_view(),
            depth_ops: Some(wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
          }),
          occlusion_query_set: None,
          timestamp_writes: None,
        });

    render_pass.set_pipeline(&self.render_pipeline);

    // Activate the camera's bind group.
    render_pass.set_bind_group(1, self.camera.get_bind_group(), &[]);

    // Activate the color bind group.
    render_pass.set_bind_group(2, self.color_uniform.get_bind_group(), &[]);

    // We set the instance buffer to be nothing for not instanced render calls.
    // This blank_data must match our lifetime.
    let blank_data = InstancedRenderData::get_blank_data();
    self.instance_buffer = Some(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("instance_buffer"),
        contents: bytemuck::cast_slice(&blank_data),
        usage: wgpu::BufferUsages::VERTEX,
      },
    ));

    let not_instanced_render_call = self.model_render_queue.pop_front().unwrap();

    let mesh_name = not_instanced_render_call.get_mesh_name();

    match self.models.get(mesh_name) {
      Some(model) => {
        let meshes = &model.meshes;
        let texture_names = not_instanced_render_call.get_texture_name();

        // todo: in the future make this just insert some default texture.
        let meshes_length = meshes.len();
        let textures_length = texture_names.len();
        if meshes.len() != texture_names.len() {
          error!("RenderEngine: Attempted not instanced render on model [{}] with unmatched texture to model buffers.
          Required: [{}]
          Received: [{}]", model.name, meshes_length, textures_length);
        }

        // We want to iterate them at the same time, zip it.
        for (mesh, texture_name) in meshes.iter().zip(texture_names) {
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
            None => error!(
              "render_engine: {} is not a stored Texture. [not instanced]",
              texture_name
            ),
          }
        }
      }
      None => error!(
        "render_engine: {} is not a stored Mesh. [not instanced]",
        mesh_name
      ),
    }
  }

  ///
  /// Process and run all not instanced Model render calls.
  ///
  fn process_not_instanced_model_render_calls(&mut self) {
    while !self.model_render_queue.is_empty() {
      self.initialize_render();
      self.process_not_instanced_model_render_call();
      self.submit_render();
    }
  }

  ///
  /// Process and run all not instanced render calls.
  ///
  pub fn process_not_instanced_render_calls(&mut self) {
    self.process_not_instanced_mesh_render_calls();
    self.process_not_instanced_model_render_calls();
  }

  ///
  /// Process out a batched Mesh render call.
  ///
  /// Due to the implementation nature, this needs to be run on each
  /// mesh in sequence.
  ///
  fn process_instanced_mesh_render_call(
    &mut self,
    mesh_name: &String,
    instance_data: &Vec<InstancedRenderData>,
  ) {
    // Do 4 very basic checks before attempting to render.
    if self.output.is_none() {
      panic!("RenderEngine: attempted to render with no output!");
    }

    if self.command_encoder.is_none() {
      panic!("RenderEngine: attempted render with no command encoder!");
    }

    if self.texture_view.is_none() {
      panic!("RenderEngine: attempted to render with no texture view!");
    }

    if self.depth_buffer.is_none() {
      panic!("RenderEngine: attempted to render with no depth buffer!");
    }

    // Begin a wgpu render pass
    let mut render_pass =
      self
        .command_encoder
        .as_mut()
        .unwrap()
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          // The label of this render pass.
          label: Some("minetest_instanced_mesh_render_pass"),

          // color attachments is a array of pipeline render pass color attachments.
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: self.texture_view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],

          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: self.depth_buffer.as_ref().unwrap().get_view(),
            depth_ops: Some(wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
          }),
          occlusion_query_set: None,
          timestamp_writes: None,
        });

    render_pass.set_pipeline(&self.render_pipeline);

    // Activate the camera's bind group.
    render_pass.set_bind_group(1, self.camera.get_bind_group(), &[]);

    // Activate the color bind group.
    render_pass.set_bind_group(2, self.color_uniform.get_bind_group(), &[]);

    // Enable instancing in shader.
    self.instance_trigger.trigger_on(&self.queue);

    // * Begin instanced render call.
    match self.meshes.get(mesh_name) {
      Some(mesh) => {
        // ! NOTE: THIS IS WHERE EVERYTHING BROKE!
        // ! REMINDER: THE PLACEHOLDER HAD TO BE REMOVED!
        error!("fix the tf.webp placeholder");
        let texture_name = "tf.webp";
        // let texture_name = mesh.get_default_texture();
        match self.textures.get(texture_name) {
          Some(texture) => {
            // Now activate the used texture's bind group.
            render_pass.set_bind_group(0, texture.get_wgpu_diffuse_bind_group(), &[]);

            // Now we're going to bind the pipeline to the Mesh and draw it.

            self.instance_buffer = Some(self.device.create_buffer_init(
              &wgpu::util::BufferInitDescriptor {
                label: Some("instance_buffer"),
                contents: bytemuck::cast_slice(instance_data),
                usage: wgpu::BufferUsages::VERTEX,
              },
            ));
            self
              .mesh_trs_uniform
              .build_mesh_projection_matrix(&self.device, &self.queue);

            render_pass.set_vertex_buffer(0, mesh.get_wgpu_vertex_buffer().slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.as_ref().unwrap().slice(..));

            render_pass.set_index_buffer(
              mesh.get_wgpu_index_buffer().slice(..),
              wgpu::IndexFormat::Uint32,
            );

            render_pass.draw_indexed(
              0..mesh.get_number_of_indices(),
              0,
              0..(instance_data.len() as u32),
            );
          }
          None => {
            error!(
              "render_engine: {} is not a stored Texture. [instanced]",
              texture_name
            );
          }
        }
      }
      None => {
        error!(
          "render_engine: {} is not a stored Mesh. [instanced]",
          mesh_name
        );
      }
    }
  }

  ///
  /// Completely wipes out the instanced Mesh render queue and returns the current data to you.
  ///
  fn take_mesh_instanced_data(&mut self) -> AHashMap<String, Vec<InstancedRenderData>> {
    let mut temporary = AHashMap::new();
    swap(&mut self.instanced_mesh_render_queue, &mut temporary);
    temporary
  }

  ///
  /// Process and submit all the instanced Mesh render calls.
  ///
  pub fn process_instanced_render_calls(&mut self) {
    // ! This is an absolute brute force method. Perhaps there's a more elegant way?
    let instanced_key_value_set = self.take_mesh_instanced_data();

    // Iterate through all the instanced data.
    for (mesh_name, instance_data) in instanced_key_value_set {
      self.initialize_render();
      self.process_instanced_mesh_render_call(&mesh_name, &instance_data);
      self.submit_render();
    }
  }

  ///
  /// Submits all commands into wgpu.
  ///
  fn submit_render(&mut self) {
    // Let's swap the command encoder out into a local variable.
    // It has now become flushed into None.
    let mut final_encoder: Option<CommandEncoder> = None;

    swap(&mut final_encoder, &mut self.command_encoder);

    self
      .queue
      .submit(iter::once(final_encoder.unwrap().finish()));
  }

  ///
  /// !ONLY TO BE RAN AFTER ALL COMMANDS ARE COMPLETED!
  ///
  /// Flushes the texture buffer into the SDL2 window.
  ///
  /// Destroys the old context.
  ///
  pub fn show_and_destroy_frame_buffer(&mut self) {
    // Next we simply swap the surface out into a local variable. We've just flushed the surface out into None.

    let mut final_output: Option<SurfaceTexture> = None;

    swap(&mut final_output, &mut self.output);

    final_output.unwrap().present();

    // Destroy the depth buffer.
    self.depth_buffer = None;

    // Finally, the texture view is outdated, destroy it.
    self.texture_view = None;

    // For now, we'll ensure that this is unchanged. [ validation ]
    assert!(self.command_encoder.is_none());
    assert!(self.output.is_none());
    assert!(self.texture_view.is_none());
    assert!(self.depth_buffer.is_none());
  }

  ///
  /// Store a Mesh into the render engine for usage.
  ///
  pub fn store_mesh(&mut self, name: &str, mesh: Mesh) {
    self.meshes.insert(name.to_owned(), mesh);
  }

  ///
  /// Store a Model into the render engine for usage.
  ///
  pub fn store_model(&mut self, name: &str, model: Model) {
    self.models.insert(name.to_owned(), model);
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
    self.mesh_render_queue.push_back(RenderCall::new(
      mesh_name,
      texture_name,
      translation,
      rotation,
      scale,
    ))
  }

  ///
  /// Render a model, not instanced.
  ///
  pub fn render_model(
    &mut self,
    model_name: &str,
    texture_names: Vec<String>,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) {
    self.model_render_queue.push_back(ModelRenderCall::new(
      model_name,
      texture_names,
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
      .instanced_mesh_render_queue
      .entry(mesh_name.to_string())
      .or_default();

    // Now push one into the vector.
    current_vec.push(InstancedRenderData::new(translation, rotation, scale));
  }

  ///
  /// Push multiple instance calls into the instance queue.
  ///
  pub fn render_mesh_instanced(&mut self, mesh_name: &str, instancing: &Vec<InstancedRenderData>) {
    // If the key does not exist, we create it.
    let current_vec = self
      .instanced_mesh_render_queue
      .entry(mesh_name.to_string())
      .or_default();

    // Now extend multiple into the vector.
    current_vec.extend(instancing);
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
