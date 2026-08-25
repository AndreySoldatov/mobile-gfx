use glam::Vec2;
use image::{Rgba, RgbaImage};
use slotmap::SlotMap;
use wgpu::include_wgsl;

use crate::{
    atlas::{Atlas, AtlasKey},
    blit::Blit,
    buffer::Buffer,
    color::{Color, srgb_to_linear},
    font::Font,
    wgpu_state::WgpuState,
};

pub struct RenderState {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) blit: Blit,
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertex_buffer: Buffer<Vertex>,
    pub(crate) index_buffer: Buffer<u32>,
    pub(crate) clear_color: wgpu::Color,
    pub(crate) atlas: Atlas,
    pub(crate) white_pixel: AtlasKey,
    pub(crate) system_font: Font,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [f32; 3],
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

pub(crate) fn white_pixel() -> RgbaImage {
    let mut pixel = RgbaImage::new(1, 1);
    pixel.put_pixel(0, 0, Rgba([255, 255, 255, 255]));
    pixel
}

impl RenderState {
    pub(crate) fn new(
        wgpu_state: &WgpuState,
        config: &wgpu::SurfaceConfiguration,
        pixel_size: Vec2,
        atlas_staging: &mut SlotMap<AtlasKey, RgbaImage>,
    ) -> Self {
        let white_pixel = atlas_staging.insert(white_pixel());
        let system_font =
            Font::load_from_bdf(atlas_staging, include_str!("assets/fonts/cherry-10-r.bdf"))
                .unwrap();
        let atlas = Atlas::new(&wgpu_state, &atlas_staging);

        let shader = wgpu_state
            .device
            .create_shader_module(include_wgsl!("assets/shaders/uber.wgsl"));

        let render_pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("RPL"),
                    bind_group_layouts: &[Some(&atlas.bgl)],
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
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &[
                            ("screen_width", pixel_size.x as f64),
                            ("screen_height", pixel_size.y as f64),
                        ],
                        ..Default::default()
                    },
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format.clone(),
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions {
                        constants: &[
                            ("screen_width", pixel_size.x as f64),
                            ("screen_height", pixel_size.y as f64),
                        ],
                        ..Default::default()
                    },
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
            vertices: vec![],
            indices: vec![],
            vertex_buffer: Buffer::new(
                wgpu::BufferUsages::VERTEX,
                &wgpu_state.device,
                "Vertex Buffer",
            ),
            index_buffer: Buffer::new(
                wgpu::BufferUsages::INDEX,
                &wgpu_state.device,
                "Index Buffer",
            ),
            clear_color: wgpu::Color::BLACK,
            atlas,
            white_pixel,
            system_font,
        }
    }

    pub(crate) fn clear_buffers(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub(crate) fn render(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        encoder: &mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,
        state: &WgpuState,
    ) {
        self.vertex_buffer
            .write(&self.vertices, &state.queue, &state.device);
        self.index_buffer
            .write(&self.indices, &state.queue, &state.device);
        self.clear_buffers();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blit.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            {
                if self.vertex_buffer.length() > 0 {
                    render_pass.set_pipeline(&self.pipeline);
                    render_pass.set_bind_group(0, &self.atlas.bg, &[]);
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice());
                    render_pass
                        .set_index_buffer(self.index_buffer.slice(), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..(self.index_buffer.length() as u32), 0, 0..1);
                }
            }
        }

        self.blit.render(encoder, surface_view, config);
    }

    #[allow(dead_code)]
    pub(crate) fn push_vertex(&mut self, vert: Vertex) {
        self.indices.push(self.vertices.len() as u32);
        self.vertices.push(vert);
    }

    #[allow(dead_code)]
    pub(crate) fn append_vertices(&mut self, verts: &[Vertex], indices: &[u32]) {
        assert!(indices.len() > 0);
        self.indices
            .extend(indices.iter().map(|i| i + self.vertices.len() as u32));
        self.vertices.extend_from_slice(verts);
    }

    pub fn clear_color(&mut self, color: Color) {
        let color = srgb_to_linear(color);

        self.clear_color = wgpu::Color {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: 1.0,
        }
    }
}
