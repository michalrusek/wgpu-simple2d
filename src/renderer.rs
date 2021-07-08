use crate::texture::Texture;
use std::{mem};
use futures::executor::{LocalPool, LocalSpawner};
use futures::task::SpawnExt;
use wgpu::{DepthBiasState, MultisampleState, PrimitiveState, util::{DeviceExt, StagingBelt}};
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section, Text, ab_glyph};

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
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    staging_belt_local_pool: LocalPool,
    staging_belt_local_spawner: LocalSpawner,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window, desired_res: winit::dpi::PhysicalSize<u32>) -> Self {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }).await.unwrap();
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        }, None).await.unwrap();

        // Setup font rendering
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../res/font/PressStart2P-Regular.ttf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, render_format);

        // The font library requires a staging belt that has to be synced manually unfortunately
        // TODO: Check if you could load the font to gpu and just draw instanced on chars?
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let staging_belt_local_pool = LocalPool::new();
        let staging_belt_local_spawner = staging_belt_local_pool.spawner();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: render_format,
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
                        ty: wgpu::BindingType::Texture {
                            multisampled: false, 
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {filterable: false,},
                        },
                        count: None,   
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
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

            let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
            let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));
            
            // TODO: maybe extracting it would help creating multiple pipelines in case lights are added or something?
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<crate::texture::Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    offset: 0,
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x2,
                                }
                            ],
                        }
                    ],
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    bias: DepthBiasState::default(),
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                }),
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[
                        wgpu::ColorTargetState {
                            format: sc_desc.format,
                            write_mask: wgpu::ColorWrite::ALL,
                            blend: Some(wgpu::BlendState {
                                alpha: wgpu::BlendComponent {
                                    src_factor: wgpu::BlendFactor::One,
                                    dst_factor: wgpu::BlendFactor::One,
                                    operation: wgpu::BlendOperation::Min,
                                },
                                color: wgpu::BlendComponent {
                                    src_factor: wgpu::BlendFactor::SrcAlpha,
                                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                    operation: wgpu::BlendOperation::Add,
                                },
                            })
                        }
                    ]
                }),
                multisample: MultisampleState {
                    alpha_to_coverage_enabled: false,
                    count: 1,
                    mask: !0,
                },
                primitive: PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    clamp_depth: false,
                    conservative: false,
                    cull_mode: Some(wgpu::Face::Back),
                    front_face: wgpu::FrontFace::Ccw,
                    strip_index_format: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                },
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
            desired_res,
            glyph_brush,
            staging_belt,
            staging_belt_local_pool,
            staging_belt_local_spawner,
        }
    }
    pub fn render(&mut self, renderables: &Vec<Renderable>, renderable_texts: &Vec<RenderableText>) {
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
                    // TODO: HOW TO RENDER RESOLUTIONS OTHER THAN 16:9??
                    // assume target resolution of 1280x720 keeping the horizontal ratio steady
                    let horiz_ratio = 1.; //self.size.width as f32 / self.desired_res.width as f32;
                    let vert_ration = 1.; //self.size.height as f32 / self.desired_res.height as f32;
                    
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
                    
                    let (tex_x1, tex_y1, tex_x2, tex_y2) = {
                        if !renderable.horiz_mirror {
                            (0., 0., 1., 1.)
                        } else {
                            (1., 0., 0., 1.)
                        }
                    };
                    buffers.push(
                        
                        self.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(&[
                                    crate::texture::Vertex{position: [p1x, p1y, 0.0], tex_coords: [tex_x1, tex_y1]},
                                    crate::texture::Vertex{position: [p1x, p2y, 0.0], tex_coords: [tex_x1, tex_y2]},
                                    crate::texture::Vertex{position: [p2x, p2y, 0.0], tex_coords: [tex_x2, tex_y2]},
                
                                    crate::texture::Vertex{position: [p1x, p1y, 0.0], tex_coords: [tex_x1, tex_y1]},
                                    crate::texture::Vertex{position: [p2x, p2y, 0.0], tex_coords: [tex_x2, tex_y2]},
                                    crate::texture::Vertex{position: [p2x, p1y, 0.0], tex_coords: [tex_x2, tex_y1]},
                
                                ]),
                                usage: wgpu::BufferUsage::VERTEX,
                            }
                        )
                    );
                }
            }
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            }),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // Render renderables
            {
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
        }
        // Render text
        {
            for renderable_text in renderable_texts {
                self.glyph_brush.queue(Section {
                    screen_position: (renderable_text.x * self.size.width as f32, renderable_text.y * self.size.height as f32),
                    text: vec![
                        Text::new(&renderable_text.text)
                            .with_color(renderable_text.color)
                            .with_scale(renderable_text.size)
                    ],
                    ..Section::default()
                });
            }
            

            self.glyph_brush.draw_queued(
                &self.device, 
                &mut self.staging_belt, 
                &mut encoder, 
                &frame.view, 
                self.size.width, 
                self.size.height,
            ).expect("Drawing glyphs queued");
        }
        self.staging_belt.finish();

        self.queue.submit(std::iter::once(encoder.finish()));

        self.staging_belt_local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Staging belt recall;");

        self.staging_belt_local_pool.run_until_stalled();
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
    pub horiz_mirror: bool,
}

pub struct RenderableText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 4]
}