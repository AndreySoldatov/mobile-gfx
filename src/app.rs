use std::sync::Arc;

use glam::Vec2;
use log::info;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
    dstate::DynamicState,
    render::{self, RenderState},
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, wgpu_init},
};

pub struct InnerState {
    pub window: Arc<Window>,
    pub surface: Option<WgpuSurface>,
    pub wgpu_state: WgpuState,
    pub render_state: RenderState,

    pub dstate: DynamicState,
}

impl InnerState {
    fn render(&mut self) {
        let Some(surface) = &self.surface else {
            return;
        };

        self.window.request_redraw();

        let output = match surface.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Suboptimal(_) => {
                surface
                    .surface
                    .configure(&self.wgpu_state.device, &surface.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return;
            }
        };

        let surface_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.wgpu_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("render encoder"),
                });

        self.dstate.update_point(Vec2::new(
            self.render_state.pixel_size.0 as f32,
            self.render_state.pixel_size.1 as f32,
        ));

        self.render_state.draw_triangle(
            Vec2::new(10.0, 10.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(10.0, 200.0),
            render::Color {
                r: 1.0,
                g: 0.2,
                b: 0.3,
            },
        );
        self.render_state.draw_triangle(
            self.dstate.p0,
            self.dstate.p1,
            self.dstate.p2,
            render::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        );
        self.render_state.draw_triangle(
            Vec2::new(100.0, 100.0),
            Vec2::new(150.0, 50.0),
            Vec2::new(100.0, 200.0),
            render::Color {
                r: 0.1,
                g: 0.2,
                b: 1.0,
            },
        );

        self.render_state.render(
            &surface.config,
            &mut encoder,
            &surface_view,
            &self.wgpu_state.device,
        );

        self.wgpu_state.queue.submit(Some(encoder.finish()));
        self.wgpu_state.queue.present(output);
    }

    pub fn issue_new_surface(&mut self) {
        self.surface = None;
        self.surface = Some(create_wgpu_surface(&self.wgpu_state, self.window.clone()))
    }
}

pub struct App {
    pub state: Option<InnerState>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        info!("Resumed!");
        match &mut self.state {
            Some(s) => {
                if s.surface.is_none() {
                    s.issue_new_surface();
                }
            }
            None => {
                let window = Arc::new(
                    #[cfg(target_os = "android")]
                    event_loop
                        .create_window(Window::default_attributes())
                        .unwrap(),
                    #[cfg(not(target_os = "android"))]
                    event_loop
                        .create_window(
                            Window::default_attributes()
                                .with_inner_size(winit::dpi::PhysicalSize::new(540, 1200))
                                .with_resizable(false),
                        )
                        .unwrap(),
                );

                let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();
                let render_state = RenderState::new(&wgpu_state, &surface.config);
                let inner_state = InnerState {
                    wgpu_state,
                    surface: Some(surface),
                    window,
                    render_state,
                    dstate: DynamicState::new(),
                };
                self.state = Some(inner_state);
            }
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("Suspended!");
        self.state.as_mut().unwrap().surface = None
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();
                if state.surface.is_some() {
                    state.render();
                }
            }
            WindowEvent::Destroyed => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
