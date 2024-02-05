use std::mem::size_of;

use glam::{Mat4, Quat, Vec3A};

///
/// A InstancedRenderUniform is an instanced render call optimized to draw
/// many instances of the same mesh at once. This is much faster than regular RenderCall when
/// attempting to draw things like items and mobs, so please use it as so.
///
/// * This may look like TRSProjectionData, but it's not.
///
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceMatrix {
  matrix: [[f32; 4]; 4],
}

impl InstanceMatrix {
  pub fn new(translation: Vec3A, rotation: Vec3A, scale: Vec3A) -> Self {
    let rotation = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    let matrix = Mat4::from_scale_rotation_translation(scale.into(), rotation, translation.into())
      .to_cols_array_2d();
    InstanceMatrix { matrix }
  }

  pub fn get_wgpu_descriptor() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: size_of::<InstanceMatrix>() as wgpu::BufferAddress,
      // We need to switch from using a step mode of Vertex to Instance
      // This means that our shaders will only change to use the next
      // instance when the shader starts processing a new instance
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
        // for each vec4. We'll have to reassemble the mat4 in the shader.
        wgpu::VertexAttribute {
          offset: 0,
          // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
          // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
      ],
    }
  }

  pub fn get_blank_data() -> Vec<f32> {
    vec![
      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ]
  }
}

///
/// Holds instancing data for a Mesh.
///
/// Works on a first come first serve basis.
///
/// You can add. But you cannot remove.
///
/// Once the Texture is set, it cannot be changed.
///
pub struct InstancedMeshRenderData {
  matrices: Vec<InstanceMatrix>,
  texture: String,
}

impl InstancedMeshRenderData {
  pub fn new(texture: &str) -> Self {
    InstancedMeshRenderData {
      matrices: vec![],
      texture: texture.to_owned(),
    }
  }

  ///
  /// Push one new piece of instance data into the container.
  ///
  /// This is less efficient than push.
  ///
  /// Simply added to be more modular.
  ///
  pub fn push_single(&mut self, translation: Vec3A, rotation: Vec3A, scale: Vec3A) {
    self
      .matrices
      .push(InstanceMatrix::new(translation, rotation, scale));
  }

  ///
  /// Push new instance data into the container.
  ///  
  pub fn push(&mut self, instancing: &Vec<InstanceMatrix>) {
    self.matrices.extend(instancing);
  }

  ///
  /// When the RenderEngine is finally ready, it will borrow the data and complete
  /// the usecase for this struct.
  ///
  pub fn borrow_data(&self) -> &Vec<InstanceMatrix> {
    &self.matrices
  }

  ///
  /// Borrow the texture name for rendering.
  ///
  pub fn borrow_texture_name(&self) -> &String {
    &self.texture
  }
}

///
/// Instance data for rendering Models.
///
/// It's first come first server, data can be added, but not removed.
///
pub struct InstancedModelRenderData {
  matrices: Vec<InstanceMatrix>,
  textures: Vec<String>,
}

impl InstancedModelRenderData {
  pub fn new(textures: &[String]) -> Self {
    InstancedModelRenderData {
      matrices: vec![],
      textures: textures.to_vec(),
    }
  }
}
