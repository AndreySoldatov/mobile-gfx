use wgpu::{include_wgsl, util::DeviceExt};

use crate::{blit::Blit, wgpu_state::WgpuState};

pub struct RenderState {
    pub pipeline: wgpu::RenderPipeline,
    pub blit: Blit,
    pub vertex_uniform_buffer: wgpu::Buffer,
    pub vertex_uniform_bg: wgpu::BindGroup,
    pub pixel_size: (u32, u32),

    pub vertices: Vec<Vertex>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
    col: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexUniform {
    screen_size: [f32; 2],
}

impl RenderState {
    pub fn new(wgpu_state: &WgpuState, config: &wgpu::SurfaceConfiguration) -> Self {
        let pixel_size = (180, 400);

        // TODO: change uniform to embedded constant
        let shader = wgpu_state
            .device
            .create_shader_module(include_wgsl!("shaders/uber.wgsl"));

        let vert_uni = VertexUniform {
            screen_size: [pixel_size.0 as f32, pixel_size.1 as f32],
        };

        let vertex_uniform_buffer =
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex uniform buffer"),
                    contents: bytemuck::cast_slice(&[vert_uni]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let ver_uni_bgl =
            wgpu_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("vertex uniform bgl"),
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
                });

        let ver_uni_bg = wgpu_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("vertex uniform bg"),
                layout: &ver_uni_bgl,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                }],
            });

        let render_pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("RPL"),
                    bind_group_layouts: &[Some(&ver_uni_bgl)],
                    immediate_size: 0,
                });

        let pipeline = wgpu_state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("RP"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(Vertex::desc())],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format.clone(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None,
                cache: None,
            });

        let blit = Blit::new(&wgpu_state.device, pixel_size, config.format.clone());

        Self {
            pipeline,
            blit,
            pixel_size,
            vertex_uniform_bg: ver_uni_bg,
            vertex_uniform_buffer,

            vertices: vec![],
        }
    }

    pub fn render(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        encoder: &mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,
        device: &wgpu::Device,
    ) {
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blit.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            if !self.vertices.is_empty() {
                let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex buffer"),
                    contents: &bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &self.vertex_uniform_bg, &[]);
                render_pass.set_vertex_buffer(0, vb.slice(..));
                render_pass.draw(0..(self.vertices.len() as u32), 0..1);
            }
        }

        {
            let blit = &self.blit;

            // Blit pass
            let mut blit_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blit Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            let (x, y, w, h) = blit.integer_fit(config.width, config.height);
            blit_pass.set_viewport(x, y, w, h, 0.0, 1.0);
            blit_pass.set_pipeline(&blit.pipeline);
            blit_pass.set_bind_group(0, &blit.bg, &[]);
            blit_pass.draw(0..3, 0..1);
        }

        self.vertices.clear();
    }

    pub fn draw_triangle(&mut self, p1: glam::Vec2, p2: glam::Vec2, p3: glam::Vec2, color: Color) {
        self.vertices.push(Vertex {
            pos: [p1.x, p1.y],
            uv: [0.0, 0.0],
            col: [color.r, color.g, color.b],
        });
        self.vertices.push(Vertex {
            pos: [p2.x, p2.y],
            uv: [0.0, 0.0],
            col: [color.r, color.g, color.b],
        });
        self.vertices.push(Vertex {
            pos: [p3.x, p3.y],
            uv: [0.0, 0.0],
            col: [color.r, color.g, color.b],
        });
    }
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
