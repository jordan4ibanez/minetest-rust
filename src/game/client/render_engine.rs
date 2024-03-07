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
use glam::{UVec2, Vec3A, Vec4};
use log::error;

use unique_64::Unique64;
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
  instanced_render_matrix::{
    InstanceMatrixRGBA, InstancedMeshRenderData, InstancedModelRenderData,
  },
  mesh_trs_uniform::MeshTRSUniform,
  model::Model,
  render_call::{MeshRenderCall, ModelRenderCall},
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
  render_command_count: u64,

  // General variables.
  config: wgpu::SurfaceConfiguration,
  size: UVec2,
  clear_color: wgpu::Color,

  // Not instanced render queues. (Individual render calls)
  mesh_render_queue: VecDeque<MeshRenderCall>,
  model_render_queue: VecDeque<ModelRenderCall>,

  // Instanced render queues and buffer.
  instanced_mesh_render_queue: AHashMap<u64, InstancedMeshRenderData>,
  instanced_model_render_queue: AHashMap<u64, InstancedModelRenderData>,
  instance_buffer: Option<wgpu::Buffer>,
  instance_trigger: InstanceTrigger,

  // ID dispatcher for wgpu. Acts like the OpenGL ID dispatcher.
  id_dispatcher: Unique64,

  // Containers for wgpu data.
  mesh_name_to_id: AHashMap<String, u64>,
  meshes: AHashMap<u64, Mesh>,

  texture_name_to_id: AHashMap<String, u64>,
  textures: AHashMap<u64, Texture>,
  model_name_to_id: AHashMap<String, u64>,
  models: AHashMap<u64, Model>,

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
      // This is the portion where you can select the backend.
      // Vulkan, OpenGL, Metal, DX11, DX12, and WebGPU.
      // It automatically selects based on your hardware for now.
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
      None => panic!("RenderEngine: no graphics adapter found!"),
    };

    // We must block the main thread while this completes or things can go crazy.
    // This is waiting for a future.
    // ! Block on might cause a crash in WASM !
    let (device, queue) = match pollster::block_on(adapter.request_device(
      &wgpu::DeviceDescriptor {
        limits: wgpu::Limits::default(),
        label: Some("minetest_gpu"),
        // * this was: wgpu::Features::default()
        features: /*wgpu::Features::POLYGON_MODE_LINE | */ wgpu::Features::DEPTH_CLIP_CONTROL,
      },
      None,
    )) {
      Ok(device_and_queue) => device_and_queue,
      Err(e) => panic!("{}", e),
    };

    // Load up the default shader source code.
    let shader_code = match read_file_to_string("shaders/default_shader.wgsl") {
      Ok(shader_code) => shader_code,
      Err(e) => panic!("RenderEngine: {}", e),
    };
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("default_shader"),
      source: wgpu::ShaderSource::Wgsl(shader_code.into()),
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
    let surface_format = match surface_caps
      .formats
      .iter()
      .copied()
      // This may not be thorough enough to get the format we want.
      .find(|f| f.is_srgb())
    {
      Some(found_surface) => found_surface,
      None => surface_caps.formats[0],
    };

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
          InstanceMatrixRGBA::get_wgpu_descriptor(),
        ],
        module: &shader,
        entry_point: "vs_main",
      },
      fragment: Some(wgpu::FragmentState {
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
              src_factor: wgpu::BlendFactor::SrcAlpha,
              dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
              operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
          }),
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
        cull_mode: None, //Some(wgpu::Face::Back),
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
      instanced_model_render_queue: AHashMap::new(),
      instance_buffer: None,
      instance_trigger,

      // ID dispatcher for wgpu. Acts like the OpenGL ID dispatcher.
      id_dispatcher: Unique64::new(),

      // Containers for wgpu data.
      mesh_name_to_id: AHashMap::new(),
      meshes: AHashMap::new(),

      texture_name_to_id: AHashMap::new(),
      textures: AHashMap::new(),
      model_name_to_id: AHashMap::new(),
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

      new_render_engine.create_texture("./prototype_textures/tf.png");

      // ? BEGIN DEBUGGING MODEL LOADER ?

      // ! CHAIR - OBJ

      let chair_model = match ModelLoader::load_model(
        "./prototype_models/chair.obj",
        &new_render_engine.device,
        &new_render_engine.queue,
      ) {
        Ok(chair) => chair,
        Err(e) => panic!("RenderEngine: {}", e),
      };

      new_render_engine.store_model(&chair_model.name.clone(), chair_model);

      new_render_engine.create_texture("./prototype_textures/chair.png");

      // ! SNOWMAN - OBJ

      let snowman = match ModelLoader::load_model(
        "./prototype_models/snowman.obj",
        &new_render_engine.device,
        &new_render_engine.queue,
      ) {
        Ok(snowman) => snowman,
        Err(e) => panic!("RenderEngine: {}", e),
      };

      new_render_engine.store_model(&snowman.name.clone(), snowman);

      new_render_engine.create_texture("./prototype_textures/snowman.png");

      // ! MINETEST SAM - GLTF

      let minetest_sam = match ModelLoader::load_model(
        "./prototype_models/minetest_sam.gltf",
        &new_render_engine.device,
        &new_render_engine.queue,
      ) {
        Ok(sam) => sam,
        Err(e) => panic!("RenderEngine: {}", e),
      };

      new_render_engine.store_model(&minetest_sam.name.clone(), minetest_sam);

      new_render_engine.create_texture("./prototype_textures/minetest_sam.png");

      // ! SNOWMAN - GLTF

      let snowman_gltf = match ModelLoader::load_model(
        "./prototype_models/snowman.gltf",
        &new_render_engine.device,
        &new_render_engine.queue,
      ) {
        Ok(snowman_gltf) => snowman_gltf,
        Err(e) => panic!("RenderEngine: {}", e),
      };

      new_render_engine.store_model(&snowman_gltf.name.clone(), snowman_gltf);

      // ! SIMPLE_SKIN - GLTF

      let simple_skin = match ModelLoader::load_model(
        "./prototype_models/simple_skin.gltf",
        &new_render_engine.device,
        &new_render_engine.queue,
      ) {
        Ok(simple) => simple,
        Err(e) => panic!("RenderEngine: {}", e),
      };

      new_render_engine.store_model(&simple_skin.name.clone(), simple_skin);

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
    match self.surface.get_current_texture() {
      Ok(texture) => self.output = Some(texture),
      Err(e) => panic!("RenderEngine: Surface texture error. {}", e),
    }

    match self.output.as_mut() {
      Some(output) => {
        self.texture_view = Some(
          output
            .texture
            // ? If this comes up as an error in vscode, you need to switch
            // ? to a rust-analyzer pre-release version.
            .create_view(&wgpu::TextureViewDescriptor::default()),
        );
      }
      None => panic!("RenderEngine: Tried to generate a framebuffer with no output."),
    }

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
    // Begin a wgpu render pass
    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!("RenderEngine: Attempted to clear buffers without command encoder."),
    };

    let texture_view = match self.texture_view.as_ref() {
      Some(view) => view,
      None => panic!("RenderEngine: Attempted to clear buffers without texture view."),
    };

    let depth_buffer = match self.depth_buffer.as_ref() {
      Some(buffer) => buffer,
      None => panic!("RenderEngine: Attempted to clear buffers without depth buffer."),
    };

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

    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      // The label of this render pass.
      label: Some("minetest_clear_buffers_render_pass"),

      // color attachments is a array of pipeline render pass color attachments.
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: clear_color,
          store: wgpu::StoreOp::Store,
        },
      })],

      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_buffer.get_view(),
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
  fn process_not_instanced_mesh_render_call(
    &mut self,
    not_instanced_mesh_render_call: MeshRenderCall,
  ) {
    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!("RenderEngine: Attempted to process not instanced mesh render call without command encoder."),
    };

    let texture_view = match self.texture_view.as_ref() {
      Some(view) => view,
      None => {
        panic!("RenderEngine: Attempted to not instanced mesh render call without texture view.")
      }
    };

    let depth_buffer = match self.depth_buffer.as_ref() {
      Some(buffer) => buffer,
      None => {
        panic!("RenderEngine: Attempted to not instanced mesh render call without depth buffer.")
      }
    };

    // * Begin not instanced render calls. [MESH]
    // Begin a wgpu render pass
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      // The label of this render pass.
      label: Some("minetest_not_instanced_mesh_render_pass"),

      // color attachments is a array of pipeline render pass color attachments.
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
      })],

      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_buffer.get_view(),
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
    let blank_data = InstanceMatrixRGBA::get_blank_data();
    self.instance_buffer = Some(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("instance_buffer"),
        contents: bytemuck::cast_slice(&blank_data),
        usage: wgpu::BufferUsages::VERTEX,
      },
    ));

    // Disable instancing in shader.
    self.instance_trigger.trigger_off(&self.queue);

    let mesh_id = not_instanced_mesh_render_call.get_mesh_id();

    match self.meshes.get(&mesh_id) {
      Some(mesh) => {
        let texture_id = not_instanced_mesh_render_call.get_texture_id();

        match self.textures.get(&texture_id) {
          Some(texture) => {
            // Now activate the used texture's bind group.
            render_pass.set_bind_group(0, texture.get_wgpu_diffuse_bind_group(), &[]);

            self
              .mesh_trs_uniform
              .set_translation(not_instanced_mesh_render_call.get_translation());
            self
              .mesh_trs_uniform
              .set_rotation(not_instanced_mesh_render_call.get_rotation());
            self
              .mesh_trs_uniform
              .set_scale(not_instanced_mesh_render_call.get_scale());

            self
              .mesh_trs_uniform
              .build_mesh_projection_matrix(&self.device, &self.queue);

            // Now we're going to bind the pipeline to the Mesh and draw it.

            render_pass.set_vertex_buffer(0, mesh.get_wgpu_vertex_buffer().slice(..));

            let instance_buffer = match self.instance_buffer.as_ref() {
              Some(buffer) => buffer,
              None => panic!("RenderEngine: Attempted to render Mesh with no instance buffer."),
            };
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

            render_pass.set_index_buffer(
              mesh.get_wgpu_index_buffer().slice(..),
              wgpu::IndexFormat::Uint32,
            );

            render_pass.draw_indexed(0..mesh.get_number_of_indices(), 0, 0..1);
          }
          None => error!(
            "render_engine: ID {} is not a stored Texture. [not instanced]",
            texture_id
          ),
        }
      }
      None => error!(
        "render_engine: ID {} is not a stored Mesh. [not instanced]",
        mesh_id
      ),
    }
  }

  ///
  /// Process and run all Mesh render calls.
  ///
  fn process_not_instanced_mesh_render_calls(&mut self) {
    while let Some(not_instanced_mesh_render_call) = self.mesh_render_queue.pop_front() {
      self.initialize_render();
      self.process_not_instanced_mesh_render_call(not_instanced_mesh_render_call);
      self.submit_render();
    }
  }

  ///
  /// Processes the next available render call in the Model queue.
  ///
  fn process_not_instanced_model_render_call(
    &mut self,
    not_instanced_model_render_call: ModelRenderCall,
  ) {
    // * Begin not instanced render calls. [MODEL]
    // ? note: if you can find a way to draw all this in one render pass, open a PR immediately.

    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!(
        "RenderEngine: Tried to process not instanced Model render call without a command encoder."
      ),
    };

    let texture_view = match self.texture_view.as_ref() {
      Some(view) => view,
      None => {
        panic!("RenderEngine: Attempted to not instanced Model render call without texture view.")
      }
    };

    let depth_buffer = match self.depth_buffer.as_ref() {
      Some(buffer) => buffer,
      None => {
        panic!("RenderEngine: Attempted to not instanced Model render call without depth buffer.")
      }
    };

    // Begin a wgpu render pass
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      // The label of this render pass.
      label: Some("minetest_not_instanced_model_render_pass"),

      // color attachments is a array of pipeline render pass color attachments.
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
      })],

      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_buffer.get_view(),
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
    let blank_data = InstanceMatrixRGBA::get_blank_data();
    self.instance_buffer = Some(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("instance_buffer"),
        contents: bytemuck::cast_slice(&blank_data),
        usage: wgpu::BufferUsages::VERTEX,
      },
    ));

    let mesh_id = not_instanced_model_render_call.get_model_id();

    match self.models.get(&mesh_id) {
      Some(model) => {
        let meshes = &model.meshes;
        let texture_ids = not_instanced_model_render_call.get_texture_ids();

        // todo: in the future make this just insert some default texture.
        let meshes_length = meshes.len();
        let textures_length = texture_ids.len();
        if meshes.len() != texture_ids.len() {
          error!("RenderEngine: Attempted not instanced render on model ID [{}] with unmatched texture to model buffers.
          Required: [{}]
          Received: [{}]", model.name, meshes_length, textures_length);

          // Do not attempt to do this.
          return;
        }

        // We want to iterate them at the same time, zip it.
        for (mesh, texture_id) in meshes.iter().zip(texture_ids) {
          match self.textures.get(texture_id) {
            Some(texture) => {
              // Now activate the used texture's bind group.
              render_pass.set_bind_group(0, texture.get_wgpu_diffuse_bind_group(), &[]);

              self
                .mesh_trs_uniform
                .set_translation(not_instanced_model_render_call.get_translation());
              self
                .mesh_trs_uniform
                .set_rotation(not_instanced_model_render_call.get_rotation());
              self
                .mesh_trs_uniform
                .set_scale(not_instanced_model_render_call.get_scale());

              self
                .mesh_trs_uniform
                .build_mesh_projection_matrix(&self.device, &self.queue);

              // Now we're going to bind the pipeline to the Mesh and draw it.

              // match &model.animations {
              //   Some(vec_animations) => match vec_animations.first() {
              //     Some(animation) => {
              //       println!("{} is animated.", model.name);

              //       match animation.timestamps.get(1) {
              //         Some(time_stamp) => {
              //           println!("timestamp: {}", time_stamp);
              //           println!("animation name: {}", animation.name);
              //           match &animation.keyframes {
              //             Keyframes::Translation(_) => todo!(),
              //             Keyframes::Rotation(_) => todo!(),
              //             Keyframes::Scale(_) => todo!(),
              //             Keyframes::Weights(_) => todo!(),
              //             Keyframes::Other => todo!(),
              //           }
              //         }

              //         None => println!("{} is BROKEN!", model.name),
              //       };
              //     }
              //     None => println!("{} is broken.", model.name),
              //   },
              //   None => println!("{} is not animated.", model.name),
              // };

              render_pass.set_vertex_buffer(0, mesh.get_wgpu_vertex_buffer().slice(..));

              let instance_buffer = match self.instance_buffer.as_ref() {
                Some(buffer) => buffer,
                None => panic!("RenderEngine: Attempted to render Model with no instance buffer."),
              };

              render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

              render_pass.set_index_buffer(
                mesh.get_wgpu_index_buffer().slice(..),
                wgpu::IndexFormat::Uint32,
              );

              render_pass.draw_indexed(0..mesh.get_number_of_indices(), 0, 0..1);
            }
            None => error!(
              "render_engine: ID {} is not a stored Texture. [not instanced]",
              texture_id
            ),
          }
        }
      }
      None => error!(
        "render_engine: ID {} is not a stored Mesh. [not instanced]",
        mesh_id
      ),
    }
  }

  ///
  /// Process and run all not instanced Model render calls.
  ///
  fn process_not_instanced_model_render_calls(&mut self) {
    while let Some(model_not_instanced_render_call) = self.model_render_queue.pop_front() {
      self.initialize_render();
      self.process_not_instanced_model_render_call(model_not_instanced_render_call);
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
  /// Process out an instanced Mesh render call.
  ///
  /// Due to the implementation nature, this needs to be run on each
  /// Mesh in sequence.
  ///
  fn process_instanced_mesh_render_call(
    &mut self,
    mesh_id: u64,
    texture_id: u64,
    instance_data: &[InstanceMatrixRGBA],
  ) {
    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!(
        "RenderEngine: Tried to process instanced Mesh render call without a command encoder."
      ),
    };

    let texture_view = match self.texture_view.as_ref() {
      Some(view) => view,
      None => {
        panic!("RenderEngine: Attempted to instanced Mesh render call without texture view.")
      }
    };

    let depth_buffer = match self.depth_buffer.as_ref() {
      Some(buffer) => buffer,
      None => {
        panic!("RenderEngine: Attempted to instanced Mesh render call without depth buffer.")
      }
    };

    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!(
        "RenderEngine: Tried to process instanced Mesh render call without a command encoder."
      ),
    };

    // Begin a wgpu render pass
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      // The label of this render pass.
      label: Some("minetest_instanced_mesh_render_pass"),

      // color attachments is a array of pipeline render pass color attachments.
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
      })],

      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_buffer.get_view(),
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
    match self.meshes.get(&mesh_id) {
      Some(mesh) => {
        match self.textures.get(&texture_id) {
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

            let instance_buffer = match self.instance_buffer.as_ref() {
              Some(buffer) => buffer,
              None => panic!("RenderEngine: Attempted to render Mesh with no instance buffer."),
            };

            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

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
              texture_id
            );
          }
        }
      }
      None => {
        error!(
          "render_engine: {} is not a stored Mesh. [instanced]",
          mesh_id
        );
      }
    }
  }

  ///
  /// Completely wipes out the instanced Mesh render queue and returns the current data to you.
  ///
  fn take_mesh_instanced_data(&mut self) -> AHashMap<u64, InstancedMeshRenderData> {
    let mut temporary = AHashMap::new();
    swap(&mut self.instanced_mesh_render_queue, &mut temporary);
    temporary
  }

  ///
  /// Process and submit all the instanced Mesh render calls.
  ///
  pub fn process_instanced_mesh_render_calls(&mut self) {
    // ! This is an absolute brute force method. Perhaps there's a more elegant way?
    let instanced_key_value_set = self.take_mesh_instanced_data();

    // Iterate through all the instanced data.
    for (mesh_name, instance_data) in instanced_key_value_set {
      self.initialize_render();
      self.process_instanced_mesh_render_call(
        mesh_name,
        instance_data.get_texture_id(),
        instance_data.borrow_data(),
      );
      self.submit_render();
    }
  }

  ///
  /// Process out an instanced Model render call.
  ///
  /// Due to the implementation nature, this needs to be run on each
  /// Model in sequence, and then for each Model Mesh Buffer in sequence.
  ///
  fn process_instanced_model_render_call(
    &mut self,
    model_id: u64,
    texture_ids: &[u64],
    instance_data: &[InstanceMatrixRGBA],
  ) {
    let command_encoder = match self.command_encoder.as_mut() {
      Some(encoder) => encoder,
      None => panic!(
        "RenderEngine: Tried to process instanced Model render call without a command encoder."
      ),
    };

    let texture_view = match self.texture_view.as_ref() {
      Some(view) => view,
      None => {
        panic!("RenderEngine: Attempted to instanced Model render call without texture view.")
      }
    };

    let depth_buffer = match self.depth_buffer.as_ref() {
      Some(buffer) => buffer,
      None => {
        panic!("RenderEngine: Attempted to instanced Model render call without depth buffer.")
      }
    };

    // Begin a wgpu render pass
    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      // The label of this render pass.
      label: Some("minetest_instanced_model_render_pass"),

      // color attachments is a array of pipeline render pass color attachments.
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
      })],

      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_buffer.get_view(),
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
    match self.models.get(&model_id) {
      Some(model) => {
        let meshes = &model.meshes;

        // todo: in the future make this just insert some default texture.
        let meshes_length = meshes.len();
        let textures_length = texture_ids.len();

        if meshes_length != textures_length {
          error!("RenderEngine: Attempted not instanced render on Model [{}] with unmatched texture to Model buffers.
          Required: [{}]
          Received: [{}]", model.name, meshes_length, textures_length);

          // Do not attempt to do this.
          return;
        }

        // We only have to set the instance Buffer once.
        self.instance_buffer = Some(self.device.create_buffer_init(
          &wgpu::util::BufferInitDescriptor {
            label: Some("instance_buffer"),
            contents: bytemuck::cast_slice(instance_data),
            usage: wgpu::BufferUsages::VERTEX,
          },
        ));

        for (mesh, texture_id) in meshes.iter().zip(texture_ids) {
          match self.textures.get(texture_id) {
            Some(texture) => {
              // Now activate the used texture's bind group.
              render_pass.set_bind_group(0, texture.get_wgpu_diffuse_bind_group(), &[]);

              // Now we're going to bind the pipeline to the Mesh and draw it.

              self
                .mesh_trs_uniform
                .build_mesh_projection_matrix(&self.device, &self.queue);

              render_pass.set_vertex_buffer(0, mesh.get_wgpu_vertex_buffer().slice(..));

              let instance_buffer = match self.instance_buffer.as_ref() {
                Some(buffer) => buffer,
                None => panic!("RenderEngine: Attempted to render Model with no instance buffer."),
              };

              render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

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
                "render_engine: ID {} is not a stored Texture. [instanced]",
                texture_id
              );
            }
          }
        }
      }
      None => {
        error!(
          "render_engine: ID {} is not a stored Model. [instanced]",
          model_id
        );
      }
    }
  }

  ///
  /// Completely wipes out the instanced Model render queue and returns the current data to you.
  ///
  fn take_model_instanced_data(&mut self) -> AHashMap<u64, InstancedModelRenderData> {
    let mut temporary = AHashMap::new();
    swap(&mut self.instanced_model_render_queue, &mut temporary);
    temporary
  }

  ///
  /// Process and submit all the instanced Model render calls.
  ///
  pub fn process_instanced_model_render_calls(&mut self) {
    // ! This is an absolute brute force method. Perhaps there's a more elegant way?
    let instanced_key_value_set = self.take_model_instanced_data();

    // Iterate through all the instanced data.
    for (mesh_id, instance_data) in instanced_key_value_set {
      self.initialize_render();
      self.process_instanced_model_render_call(
        mesh_id,
        instance_data.borrow_texture_names(),
        instance_data.borrow_data(),
      );
      self.submit_render();
    }
  }

  ///
  /// Submits all commands into wgpu.
  ///
  fn submit_render(&mut self) {
    // Let's swap the command encoder out into a local variable.
    // It has now become flushed into None.
    let mut final_encoder_option: Option<CommandEncoder> = None;

    swap(&mut final_encoder_option, &mut self.command_encoder);

    let final_encoder = match final_encoder_option {
      Some(encoder) => encoder,
      None => panic!("RenderEngine: Tried to submit render with no command encoder."),
    };

    self.queue.submit(iter::once(final_encoder.finish()));
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

    let mut final_output_option: Option<SurfaceTexture> = None;

    swap(&mut final_output_option, &mut self.output);

    let final_output = match final_output_option {
      Some(output) => output,
      None => panic!("RenderEngine: Attempted to show a framebuffer that doesn't exist."),
    };

    final_output.present();

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
  /// Returns the Mesh ID.
  ///
  pub fn store_mesh(&mut self, name: &str, mesh: Mesh) -> u64 {
    let new_id = self.id_dispatcher.get_next();
    self.mesh_name_to_id.insert(name.to_owned(), new_id);
    self.meshes.insert(new_id, mesh);
    new_id
  }

  ///
  /// Store a Model into the render engine for usage.
  ///
  /// Returns the Model ID.
  ///
  pub fn store_model(&mut self, name: &str, model: Model) -> u64 {
    let new_id = self.id_dispatcher.get_next();
    self.model_name_to_id.insert(name.to_owned(), new_id);
    self.models.insert(new_id, model);
    new_id
  }

  ///
  /// Automatically create a texture in the RenderEngine from a path.
  ///
  /// Returns the Texture ID.
  ///
  pub fn create_texture(&mut self, path: &str) -> u64 {
    self.store_texture(Texture::new(path, &self.device, &self.queue))
  }

  ///
  /// Store a Texture into the render engine for usage.
  ///
  fn store_texture(&mut self, texture: Texture) -> u64 {
    let new_id = self.id_dispatcher.get_next();
    let name = texture.get_name().clone();
    self.texture_name_to_id.insert(name.clone(), new_id);
    self.textures.insert(new_id, texture);
    new_id
  }

  ///
  /// Get a Mesh ID from the literal &str representation.
  ///
  /// Will panic if it doesn't exist.
  ///
  pub fn get_mesh_id(&self, name: &str) -> u64 {
    match self.mesh_name_to_id.get(name) {
      Some(found_mesh_id) => *found_mesh_id,
      None => panic!("RenderEngine: Mesh [{}] does not exist!", name),
    }
  }

  ///
  /// Get Model ID from the literal &str representation.
  ///
  /// Will panic if it doesn't exist.
  ///
  pub fn get_model_id(&self, name: &str) -> u64 {
    match self.model_name_to_id.get(name) {
      Some(found_model_id) => *found_model_id,
      None => panic!("RenderEngine: Model [{}] does not exist!", name),
    }
  }

  ///
  /// Get Texture ID from the literal &str representation.
  ///
  /// Will panic if it doesn't exist.
  ///
  pub fn get_texture_id(&self, name: &str) -> u64 {
    match self.texture_name_to_id.get(name) {
      Some(found_texture_id) => *found_texture_id,
      None => panic!("RenderEngine: Texture [{}] does not exist!", name),
    }
  }

  ///
  /// Render a mesh not instanced.
  ///
  pub fn render_mesh(
    &mut self,
    mesh_id: u64,
    texture_id: u64,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) {
    self.mesh_render_queue.push_back(MeshRenderCall::new(
      mesh_id,
      texture_id,
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
    model_id: u64,
    texture_ids: Vec<u64>,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
  ) {
    self.model_render_queue.push_back(ModelRenderCall::new(
      model_id,
      texture_ids,
      translation,
      rotation,
      scale,
    ))
  }

  ///
  /// Push one instance call into the Mesh instance queue.
  ///
  /// This is less efficient than render_mesh_instanced because
  /// it needs to check if the key exists every time.
  ///
  /// If this Mesh instance has already been called, it ignores your texture.
  ///
  pub fn render_mesh_instanced_single(
    &mut self,
    mesh_id: u64,
    texture_id: u64,
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
    rgba: Vec4,
  ) {
    // If the key does not exist, we create it.
    let current_mesh_instance_render_data = self
      .instanced_mesh_render_queue
      .entry(mesh_id)
      .or_insert(InstancedMeshRenderData::new(texture_id));

    // Now push one into the struct.
    current_mesh_instance_render_data.push_single(translation, rotation, scale, rgba);
  }

  ///
  /// Push multiple Mesh instance calls into the instance queue.
  ///
  /// If this Mesh instance has already been called, it ignores your texture.
  ///
  pub fn render_mesh_instanced(
    &mut self,
    mesh_id: u64,
    texture_ids: u64,
    instancing: &Vec<InstanceMatrixRGBA>,
  ) {
    // If the key does not exist, we create it.
    let current_mesh_instance_render_data = self
      .instanced_mesh_render_queue
      .entry(mesh_id)
      .or_insert(InstancedMeshRenderData::new(texture_ids));

    // Now extend multiple into the struct.
    current_mesh_instance_render_data.push(instancing);
  }

  ///
  /// Push one instance call into the Model instance queue.
  ///
  /// This is less efficient than render_model_instanced because
  /// it needs to check if the key exists every time.
  ///
  /// If this Model instance has already been called, it ignores your texture.
  ///
  pub fn render_model_instanced_single(
    &mut self,
    model_id: u64,
    texture_ids: &[u64],
    translation: Vec3A,
    rotation: Vec3A,
    scale: Vec3A,
    rgba: Vec4,
  ) {
    // If the key does not exist, we create it.
    let current_model_instance_render_data = self
      .instanced_model_render_queue
      .entry(model_id)
      .or_insert(InstancedModelRenderData::new(texture_ids));

    // Now push one into the struct.
    current_model_instance_render_data.push_single(translation, rotation, scale, rgba);
  }

  ///
  /// Push multiple Mesh instance calls into the instance queue.
  ///
  /// If this Model instance has already been called, it ignores your texture.
  ///
  pub fn render_model_instanced(
    &mut self,
    model_id: u64,
    texture_ids: &[u64],
    instancing: &Vec<InstanceMatrixRGBA>,
  ) {
    // If the key does not exist, we create it.
    let current_model_instance_render_data = self
      .instanced_model_render_queue
      .entry(model_id)
      .or_insert(InstancedModelRenderData::new(texture_ids));

    // Now extend multiple into the struct.
    current_model_instance_render_data.push(instancing);
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
