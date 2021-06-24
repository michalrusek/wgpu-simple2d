use crate::texture::Texture;
use std::{mem};
use wgpu::util::DeviceExt;

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    depth_texture: Texture,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    textures: Vec<Texture>,
    desired_res: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window, desired_res: winit::dpi::PhysicalSize<u32>) -> Self {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        }).await.unwrap();
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        }, None).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                        count: None,   
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                        },
                        count: None,
                    },
                ],
                label: None
            }
        );

        let depth_texture = Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let render_pipeline = {
            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

            let vs_module = device.create_shader_module(wgpu::include_spirv!("shader.vert.spv"));
            let fs_module = device.create_shader_module(wgpu::include_spirv!("shader.frag.spv"));
            
            // TODO: maybe extracting it would help creating multiple pipelines in case lights are added or something?
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(
                    wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                        clamp_depth: false,
                    }
                ),
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format: sc_desc.format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }
                ],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilStateDescriptor::default(),
                }),
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint32,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: std::mem::size_of::<crate::texture::Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttributeDescriptor {
                                    offset: 0,
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float3,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float2,
                                }
                            ],
                        }
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            })
        };

        Self { 
            size, 
            surface, 
            device, queue, 
            sc_desc, 
            swap_chain, 
            depth_texture, 
            render_pipeline, 
            texture_bind_group_layout, 
            textures: vec![],
            desired_res
        }
    }
    pub fn render(&mut self, renderables: &Vec<Renderable>) {
        // SEND BUFFERS AND SHIT TO GPU AND RENDER
        let frame = self.swap_chain.get_current_frame()
            .expect("Didnt get frame")
            .output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut buffers: Vec<wgpu::Buffer> = vec![];
            for renderable in renderables {
                if renderable.texture_id as usize > (self.textures.len() - 1) {
                    println!("Wrong texture id {:?}. Can't create buffer", renderable.texture_id);
                } else {
                    // assume target resolution of 1280x720 keeping the horizontal ratio steady
                    let horiz_ratio = self.size.width as f32 / self.desired_res.width as f32;
                    let vert_ration = self.size.height as f32 / self.desired_res.height as f32;
                    
                    // calculate points based on the ratio and stuff
                    let p1x = renderable.p1[0] * vert_ration * 2.0 - 1.0;
                    let p1y = -(renderable.p1[1] * horiz_ratio * 2.0 - 1.0);
                    let (p2x, p2y) = {
                        if renderable.use_texture_size {
                            let texture = self.textures.get(renderable.texture_id as usize).unwrap();
                            let x = (renderable.p1[0] + texture.width as f32 / self.desired_res.width as f32) * vert_ration * 2.0 - 1.;
                            let y = -((renderable.p1[1] + texture.height as f32 / self.desired_res.height as f32) * horiz_ratio * 2.0 - 1.);
                            (x, y)
                        } else {
                            (renderable.p2[0] * vert_ration * 2.0 - 1.0, -(renderable.p2[1] * horiz_ratio * 2.0 - 1.0))
                        }
                    };
                    
                    buffers.push(
                        self.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(&[
                                    crate::texture::Vertex{position: [p1x, p1y, 0.0], tex_coords: [0.0, 0.0]},
                                    crate::texture::Vertex{position: [p1x, p2y, 0.0], tex_coords: [0.0, 1.0]},
                                    crate::texture::Vertex{position: [p2x, p2y, 0.0], tex_coords: [1.0, 1.0]},
                
                                    crate::texture::Vertex{position: [p1x, p1y, 0.0], tex_coords: [0.0, 0.0]},
                                    crate::texture::Vertex{position: [p2x, p2y, 0.0], tex_coords: [1.0, 1.0]},
                                    crate::texture::Vertex{position: [p2x, p1y, 0.0], tex_coords: [1.0, 0.0]},
                
                                ]),
                                usage: wgpu::BufferUsage::VERTEX,
                            }
                        )
                    );
                }
            }
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.4,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for (i, renderable) in renderables.iter().enumerate() {
                if renderable.texture_id as usize > (self.textures.len() - 1) {
                    println!("Wrong texture id {:?}. Can't render", renderable.texture_id);
                } else {
                    let texture = self.textures.get(renderable.texture_id as usize).unwrap();
                    let buffer = buffers.get(i).unwrap();
                    let bind_group = texture.bind_group.as_ref().unwrap();
                    render_pass.set_vertex_buffer(0, buffer.slice(..));
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.draw(0..6, 0..1);
                }
            }            
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.depth_texture = Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
    pub fn register_texture(&mut self, texture_path: &str) -> usize {
        if let Ok(texture) = Texture::load(&self.device, &self.queue, texture_path, &self.texture_bind_group_layout) {
            self.textures.push(texture);
            self.textures.len() - 1
        } else {
            panic!("Couldn't register texture: ".to_string() + &texture_path.to_string());
        }        
    }
}

pub struct Renderable {
    pub texture_id: usize,
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub use_texture_size: bool,
}