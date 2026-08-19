use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    error_manager::pop_error,
    render::RenderState,
    user_state::{CreationContext, UserState},
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, wgpu_init},
};

pub(crate) struct Runtime<U: UserState> {
    pub(crate) window: Arc<Window>,
    pub(crate) surface: Option<WgpuSurface>,
    pub(crate) wgpu_state: WgpuState,
    pub(crate) render_state: RenderState,
    pub(crate) user_state: U,
    pub(crate) scale: u32,
}

impl<U: UserState> Runtime<U> {
    pub(crate) fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let window = Arc::new(
            #[cfg(target_os = "android")]
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
            #[cfg(not(target_os = "android"))]
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(winit::dpi::PhysicalSize::new(360, 800)),
                )
                .unwrap(),
        );

        let mut cc = CreationContext::default();
        let user_state = U::create(&mut cc);

        let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();

        let render_state = RenderState::new(
            &wgpu_state,
            &surface.config,
            (
                surface.config.width / cc.scale,
                surface.config.height / cc.scale,
            ),
            cc.sprites,
        );
        Self {
            wgpu_state,
            surface: Some(surface),
            window,
            render_state,

            user_state,
            scale: cc.scale,
        }
    }

    pub(crate) fn create_surface(&mut self) {
        self.issue_new_surface();
        let surf = self.surface.as_ref().unwrap();
        self.render_state.resize(
            (
                surf.config.width / self.scale,
                surf.config.height / self.scale,
            ),
            &self.wgpu_state,
            self.surface.as_ref().unwrap().config.format,
        );
        self.window.request_redraw();
    }

    pub(crate) fn update(&mut self) {
        let Some(surface) = &self.surface else {
            self.render_state.clear_buffers();
            return;
        };

        self.user_state.update();
        self.user_state.draw(&mut self.render_state);

        while let Some(error) = pop_error() {
            self.user_state.on_error(error);
        }

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

        self.window.request_redraw();
    }

    pub(crate) fn issue_new_surface(&mut self) {
        self.surface = None;
        self.surface = Some(create_wgpu_surface(&self.wgpu_state, self.window.clone()))
    }

    pub(crate) fn resize(&mut self, s: PhysicalSize<u32>) {
        let Some(surface) = &mut self.surface else {
            return;
        };
        surface.config.width = s.width;
        surface.config.height = s.height;
        surface
            .surface
            .configure(&self.wgpu_state.device, &surface.config);
        self.render_state.resize(
            (s.width / self.scale, s.height / self.scale),
            &self.wgpu_state,
            self.surface.as_ref().unwrap().config.format,
        );

        self.window.request_redraw();
    }
}
