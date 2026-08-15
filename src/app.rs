use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
    render::RenderState,
    user_state::UserState,
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, issue_surface_texture, wgpu_init},
};

pub struct Context {
    pub window: Arc<Window>,
    pub surface: Option<WgpuSurface>,
    pub wgpu_state: WgpuState,
    pub render_state: RenderState,
}

impl Context {
    fn render(&mut self) {
        self.window.request_redraw();

        let Some(surface) = &self.surface else {
            return;
        };
        let Some(output) = issue_surface_texture(surface, &self.wgpu_state.device) else {
            return;
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
    pub state: Option<Context>,
    user_state: Box<dyn UserState>,
}

impl App {
    pub fn new(user_state: Box<dyn UserState>) -> Self {
        Self {
            state: None,
            user_state,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
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
                                .with_inner_size(winit::dpi::PhysicalSize::new(360, 800))
                                .with_resizable(false),
                        )
                        .unwrap(),
                );

                let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();
                let render_state = RenderState::new(&wgpu_state, &surface.config);
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
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();
                if state.surface.is_some() {
                    self.user_state.update();
                    self.user_state.draw(&state.render_state);

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
