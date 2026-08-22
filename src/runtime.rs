use std::sync::Arc;

use glam::Vec2;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    CreationContext, RuntimeContext, UserState,
    error_manager::pop_error,
    input::InputState,
    render::RenderState,
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, wgpu_init},
};

pub(crate) struct Runtime<U: UserState> {
    pub(crate) window: Arc<Window>,
    pub(crate) surface: Option<WgpuSurface>,
    pub(crate) wgpu_state: WgpuState,
    pub(crate) render_state: RenderState,
    pub(crate) user_state: U,
    pub(crate) ctx: RuntimeContext,
    pub(crate) scale: u32,
    pub(crate) input: InputState,
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
        cc.physical_size = Vec2::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        let user_state = U::create(&mut cc);

        let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();

        let pixel_size = (
            surface.config.width / cc.scale,
            surface.config.height / cc.scale,
        );
        let render_state = RenderState::new(
            &wgpu_state,
            &surface.config,
            pixel_size,
            &mut cc.atlas_staging,
        );
        let input = InputState::new(
            Vec2::new(surface.config.width as f32, surface.config.height as f32),
            Vec2::new(pixel_size.0 as f32, pixel_size.1 as f32),
        );
        Self {
            wgpu_state,
            surface: Some(surface),
            window,
            render_state,
            user_state,
            scale: cc.scale,
            ctx: RuntimeContext {
                width: pixel_size.0 as f32,
                height: pixel_size.1 as f32,
            },
            input,
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

        self.user_state.update(&self.input, &self.ctx);
        self.user_state.draw(&mut self.render_state, &self.ctx);

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
        let new_pixel_size = (s.width / self.scale, s.height / self.scale);

        self.input.resize(
            Vec2::new(surface.config.width as f32, surface.config.height as f32),
            Vec2::new(self.ctx.width, self.ctx.height),
        );
        self.render_state.resize(
            new_pixel_size,
            &self.wgpu_state,
            self.surface.as_ref().unwrap().config.format,
        );
        self.ctx.width = new_pixel_size.0 as f32;
        self.ctx.height = new_pixel_size.1 as f32;

        self.window.request_redraw();
    }
}
