use glam::UVec2;
use image::{GenericImageView, ImageBuffer, Rgba};

use crate::file_utilities::{file_name_from_path, read_file_to_byte_vec};

pub struct Texture {
  name: String,
  dimensions: UVec2,

  diffuse_bind_group: wgpu::BindGroup,

  texture: wgpu::Texture,
  view: wgpu::TextureView,
  sampler: wgpu::Sampler,
}

impl Texture {
  pub fn new(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
    let name = file_name_from_path(path);

    let diffuse_bytes = read_file_to_byte_vec(path);
    let diffuse_image = image::load_from_memory(diffuse_bytes.as_slice()).unwrap();
    let diffuse_rgba: ImageBuffer<Rgba<u8>, Vec<u8>> = diffuse_image.to_rgba8();
    let dimensions = diffuse_image.dimensions();

    let texture_size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };

    // * Keep these comments in here, they're very helpful.

    // Initial creation of the texture.
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      // All textures are stored as 3D, we represent our 2D texture
      // by setting depth to 1.
      size: texture_size,
      mip_level_count: 1, // We'll talk about this a little later
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      // Most images are stored using sRGB, so we need to reflect that here.
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
      // COPY_DST means that we want to copy data to this texture
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: Some(&name),
      // This is the same as with the SurfaceConfig. It
      // specifies what texture formats can be used to
      // create TextureViews for this texture. The base
      // texture format (Rgba8UnormSrgb in this case) is
      // always supported. Note that using a different
      // texture format is not supported on the WebGL2
      // backend.
      view_formats: &[],
    });

    // And now we upload it into the queue for usage.
    queue.write_texture(
      // Tells wgpu where to copy the pixel data
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      // The actual pixel data
      &diffuse_rgba,
      // The layout of the texture
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      texture_size,
    );

    // We don't need to configure the texture view much, so let's
    // let wgpu define it.
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    let mut diffuse_bind_group_name = name.clone();
    diffuse_bind_group_name.push_str("_diffuse_bind_group");

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &Texture::get_wgpu_bind_group_layout(device),
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&sampler),
        },
      ],
      label: Some(&diffuse_bind_group_name),
    });

    Texture {
      name,
      dimensions: UVec2::new(dimensions.0, dimensions.1),

      diffuse_bind_group,

      texture,
      view,
      sampler,
    }
  }

  ///
  /// Get the Texture's name.
  ///
  pub fn get_name(&self) -> &String {
    &self.name
  }

  ///
  /// Get the wgpu diffuse bind group for rendering.
  ///
  pub fn get_wgpu_diffuse_bind_group(&self) -> &wgpu::BindGroup {
    &self.diffuse_bind_group
  }

  ///
  /// Static function to tell wgpu how to use the Texture.
  ///
  pub fn get_wgpu_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          // This should match the filterable field of the
          // corresponding Texture entry above.
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
          count: None,
        },
      ],
      label: Some("texture_bind_group_layout"),
    })
  }
}
