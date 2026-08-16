use std::sync::Arc;

use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent,
    event_loop::ControlFlow, window::Window,
};

use crate::{
    render::RenderState,
    user_state::{CreationContext, UserState},
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, wgpu_init},
};

pub struct Context {
    pub window: Arc<Window>,
    pub surface: Option<WgpuSurface>,
    pub wgpu_state: WgpuState,
    pub render_state: RenderState,
}

impl Context {
    fn render(&mut self) {
        let Some(surface) = &self.surface else {
            self.render_state.clear_buffers();
            return;
        };

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
                self.issue_new_surface();
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

        self.render_state.render(
            &surface.config,
            &mut encoder,
            &surface_view,
            &self.wgpu_state,
        );

        self.wgpu_state.queue.submit(Some(encoder.finish()));
        self.wgpu_state.queue.present(output);
    }

    pub fn issue_new_surface(&mut self) {
        self.surface = None;
        self.surface = Some(create_wgpu_surface(&self.wgpu_state, self.window.clone()))
    }

    pub fn resize(&mut self, s: PhysicalSize<u32>, scale: u32) {
        let Some(surface) = &mut self.surface else {
            return;
        };
        surface.config.width = s.width;
        surface.config.height = s.height;
        surface
            .surface
            .configure(&self.wgpu_state.device, &surface.config);
        self.render_state.resize(
            (s.width / scale, s.height / scale),
            &self.wgpu_state,
            self.surface.as_ref().unwrap().config.format,
        );
    }
}

pub struct App {
    pub state: Option<Context>,
    user_state: Box<dyn UserState>,
    cc: CreationContext,
}

impl App {
    pub fn new(user_state: Box<dyn UserState>) -> Self {
        let cc = user_state.on_create();
        assert!(cc.scale > 0);
        Self {
            state: None,
            user_state,
            cc,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &mut self.state {
            Some(state) => {
                if state.surface.is_none() {
                    state.issue_new_surface();
                    let surf = state.surface.as_ref().unwrap();
                    state.render_state.resize(
                        (
                            surf.config.width / self.cc.scale,
                            surf.config.height / self.cc.scale,
                        ),
                        &state.wgpu_state,
                        state.surface.as_ref().unwrap().config.format,
                    );
                    state.window.request_redraw();
                }
            }
            None => {
                let cc = self.user_state.on_create();
                event_loop.set_control_flow(ControlFlow::Poll);

                let window = Arc::new(
                    #[cfg(target_os = "android")]
                    event_loop
                        .create_window(Window::default_attributes())
                        .unwrap(),
                    #[cfg(not(target_os = "android"))]
                    event_loop
                        .create_window(
                            Window::default_attributes()
                                .with_inner_size(winit::dpi::PhysicalSize::new(360, 800)), // .with_resizable(false),
                        )
                        .unwrap(),
                );

                let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();
                let render_state = RenderState::new(
                    &wgpu_state,
                    &surface.config,
                    (
                        surface.config.width / cc.scale,
                        surface.config.height / cc.scale,
                    ),
                );
                let inner_state = Context {
                    wgpu_state,
                    surface: Some(surface),
                    window,
                    render_state,
                };
                self.state = Some(inner_state);
            }
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.state.as_mut().unwrap().surface = None
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();
                if state.surface.is_some() {
                    self.user_state.update();
                    self.user_state.draw(&mut state.render_state);

                    state.render();
                }
                state.window.request_redraw();
            }
            WindowEvent::Destroyed | WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(s) => {
                if s.width == 0 || s.height == 0 {
                    return;
                }
                let state = self.state.as_mut().unwrap();
                state.resize(s, self.cc.scale);
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}
