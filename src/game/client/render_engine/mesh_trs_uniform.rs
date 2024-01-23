use glam::{Mat4, Vec3, Vec3A, Vec4};
use std::{cell::RefCell, rc::Rc};
use wgpu::util::DeviceExt;

use crate::game::client::window_handler::WindowHandler;

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4 {
  x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
  y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
  z_axis: Vec4::new(0.0, 0.0, 0.5, 0.5),
  w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
};

// We need this for Rust to store our data correctly for the shaders.
#[repr(C)]
// This is so we can store this in a buffer.
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ModelUniform {
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array.
  model_projection: [[f32; 4]; 4],
}
impl ModelUniform {
  pub fn new() -> Self {
    Self {
      model_projection: Mat4::IDENTITY.to_cols_array_2d(),
    }
  }
}

///
/// ! When rust 2024 comes out, test making this not interior mutable.
///
pub struct MeshTRSUniform {
  translation: Rc<RefCell<Vec3A>>,
  rotation: Rc<RefCell<Vec3A>>,
  scale: Rc<RefCell<Vec3A>>,

  model_uniform: Rc<RefCell<ModelUniform>>,

  // wgpu components.
  model_buffer: wgpu::Buffer,
  model_bind_group: wgpu::BindGroup,
}

impl MeshTRSUniform {
  pub fn new(device: &wgpu::Device) -> Self {
    // First up is the MeshTRSUniform's uniform.
    let model_uniform = ModelUniform::new();

    // Now we create the MeshTRSUniform's buffer.
    let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("model_trs_buffer"),
      contents: bytemuck::cast_slice(&model_uniform.model_projection),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Then the bind group.
    let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &MeshTRSUniform::get_wgpu_bind_group_layout(device),
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: model_buffer.as_entire_binding(),
      }],
      label: Some("model_trs_bind_group"),
    });

    MeshTRSUniform {
      translation: Rc::new(RefCell::new(Vec3A::new(0.0, 0.0, 0.0))),
      rotation: Rc::new(RefCell::new(Vec3A::new(0.0, 0.0, 0.0))),
      scale: Rc::new(RefCell::new(Vec3A::new(0.0, 0.0, 0.0))),

      model_uniform: Rc::new(RefCell::new(model_uniform)),

      // wgpu components.
      model_buffer,
      model_bind_group,
    }
  }

  ///
  /// Set the translation of the Mesh TRS Uniform.
  ///
  pub fn set_translation(&self, new_translation: &Vec3A) {
    self.translation.as_ref().borrow_mut().x = new_translation.x;
    self.translation.as_ref().borrow_mut().y = new_translation.y;
    self.translation.as_ref().borrow_mut().z = new_translation.z;
  }

  ///
  /// Set the rotation of the Mesh TRS Uniform.
  ///
  pub fn set_rotation(&self, new_rotation: &Vec3A) {
    self.rotation.as_ref().borrow_mut().x = new_rotation.x;
    self.rotation.as_ref().borrow_mut().y = new_rotation.y;
    self.rotation.as_ref().borrow_mut().z = new_rotation.z;
  }

  ///
  /// Set the scale of the Mesh TRS Uniform.
  ///
  pub fn set_scale(&self, new_scale: &Vec3A) {
    self.scale.as_ref().borrow_mut().x = new_scale.x;
    self.scale.as_ref().borrow_mut().y = new_scale.y;
    self.scale.as_ref().borrow_mut().z = new_scale.z;
  }

  ///
  /// The TRS model projection will automatically rebuild itself every time it is polled.
  ///
  fn build_model_projection_matrix(
    &self,
    device: &wgpu::Device,
    window_handler: &WindowHandler,
    queue: &wgpu::Queue,
  ) {
    let view_rotation = Mat4::from_euler(
      glam::EulerRot::XYZ,
      self.rotation.as_ref().borrow().x,
      self.rotation.as_ref().borrow().y,
      self.rotation.as_ref().borrow().z,
    );

    let view_translation = Mat4::from_translation(Vec3::from(*self.translation.as_ref().borrow()));

    self.model_uniform.as_ref().borrow_mut().model_projection =
      (OPENGL_TO_WGPU_MATRIX * view_rotation * view_translation).to_cols_array_2d();

    queue.write_buffer(
      self.get_buffer(),
      0,
      self.get_wgpu_raw_matrix().as_ref().borrow().as_slice(),
    );
  }

  ///
  /// Get the wgpu raw uniform contents to pass into the pipeline.
  ///
  fn get_wgpu_raw_matrix(&self) -> Rc<RefCell<Vec<u8>>> {
    let x = bytemuck::cast_slice(&self.model_uniform.as_ref().borrow().model_projection).to_owned();
    Rc::new(RefCell::new(x))
  }

  ///
  /// Get the MeshTRSUniform's wgpu bind group for rendering.
  ///
  pub fn get_bind_group(&self) -> &wgpu::BindGroup {
    &self.model_bind_group
  }

  ///
  /// Get the MeshTRSUniform's wgpu buffer.
  pub fn get_buffer(&self) -> &wgpu::Buffer {
    &self.model_buffer
  }

  ///
  /// Get the wgpu bind group layout to tell wgpu how to use the buffer.
  ///
  pub fn get_wgpu_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
      label: Some("model_trs_bind_group_layout"),
    })
  }
}
