use image::GenericImageView;
use anyhow::Result;
use std::path::Path;
use wgpu::util::DeviceExt;
use core::num::NonZeroU32;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: Option<wgpu::BindGroup>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { texture, view, sampler, bind_group: None, vertex_buffer: None, width: 0, height: 0 }
    }

    pub fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8], label: &str, layout: &wgpu::BindGroupLayout) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label), layout)
    }
    pub fn from_image(device: &wgpu::Device, queue: &wgpu::Queue, img: &image::DynamicImage, label: Option<&str>, layout: &wgpu::BindGroupLayout) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let text_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };
        let texture: wgpu::Texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: wgpu::Extent3d{
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                label: Some("diffuse_texture")
            }
        );
        queue.write_texture(wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            }, 
            &rgba, 
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(4 * dimensions.0).unwrap()),
                rows_per_image: Some(NonZeroU32::new(dimensions.1).unwrap())
            },
            text_size
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler)
                }
            ],
            label: None,
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[
                    Vertex{position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0]},
                    Vertex{position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0]},
                    Vertex{position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0]},

                    Vertex{position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0]},
                    Vertex{position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0]},
                    Vertex{position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0]},

                ]),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        Ok(Self {
            texture, 
            view: texture_view, 
            sampler: texture_sampler, 
            bind_group: Some(bind_group), 
            vertex_buffer: Some(vertex_buffer), 
            width: dimensions.0,
            height: dimensions.1, 
        })
    }

    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: P,
        layout: &wgpu::BindGroupLayout
    ) -> Result<Self> {
        let path_copy = path.as_ref().to_path_buf();
        let label = path_copy.to_str();

        let img = image::open(path)?;
        Self::from_image(device, queue, &img, label, layout)
    }

    // pub fn draw(&mut self, )
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2]
}
unsafe impl bytemuck::Pod for Vertex{}
unsafe impl bytemuck::Zeroable for Vertex{}