use std::sync::Arc;

use glam::Vec2;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    CreationContext, FrameContext, RuntimeContext, UserState,
    color::Color,
    input::InputState,
    render::RenderState,
    ui::{UiState, UiTheme},
    wgpu_state::{WgpuState, WgpuSurface, create_wgpu_surface, wgpu_init},
};

pub(crate) struct Runtime<U: UserState> {
    pub(crate) window: Arc<Window>,
    pub(crate) surface: Option<WgpuSurface>,
    pub(crate) wgpu_state: WgpuState,
    pub(crate) render_state: RenderState,
    pub(crate) user_state: U,
    pub(crate) ctx: FrameContext,
    pub(crate) input: InputState,
    pub(crate) ui: UiState,
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

        let (pw, ph) = (window.inner_size().width, window.inner_size().height);

        let factor = if pw > ph { ph / 180 } else { pw / 180 };
        let pixel_size = Vec2::new((pw / factor) as f32, (ph / factor) as f32);

        let mut cc = CreationContext {
            frame: FrameContext {
                frame_size: pixel_size,
            },
            ..Default::default()
        };
        let user_state = U::create(&mut cc);

        let (wgpu_state, surface) = wgpu_init(window.clone()).unwrap();

        let render_state = RenderState::new(
            &wgpu_state,
            &surface.config,
            pixel_size,
            &mut cc.atlas_staging,
        );
        let input = InputState::new(Vec2::new(pw as f32, ph as f32), pixel_size);
        Self {
            wgpu_state,
            surface: Some(surface),
            window,
            render_state,
            user_state,
            ctx: FrameContext {
                frame_size: pixel_size,
            },
            input,
            ui: UiState::new(UiTheme {
                background: Color::BLACK,
                foreground: Color::WHITE,
                pressed: Color::DARK_GRAY,
                primary: Color::RED,
                secondary: Color::BLUE,
            }),
        }
    }

    pub(crate) fn update(&mut self) {
        let Some(surface) = &self.surface else {
            self.render_state.clear_buffers();
            return;
        };

        let fctx = RuntimeContext {
            painter: &mut self.render_state,
            input: &self.input,
            frame: &self.ctx,
            ui: &mut self.ui,
        };
        self.user_state.frame(fctx);

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

        #[cfg(feature = "wgpu-counters")]
        {
            self.wgpu_state
                .device
                .poll(wgpu::PollType::wait_indefinitely())
                .unwrap();
            log::info!(
                "Internal counters: {:#?}",
                self.wgpu_state.device.get_internal_counters()
            );
            if let Some(alloc_stats) = self.wgpu_state.device.generate_allocator_report() {
                log::info!("Allocator report: {:#?}", alloc_stats);
            }
        }
    }

    pub(crate) fn issue_new_surface(&mut self) {
        self.surface = None;
        self.surface = Some(create_wgpu_surface(&self.wgpu_state, self.window.clone()))
    }

    pub(crate) fn create_surface(&mut self) {
        self.issue_new_surface();
        self.window.request_redraw();
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

        self.input.resize(Vec2::new(
            surface.config.width as f32,
            surface.config.height as f32,
        ));
        self.window.request_redraw();
    }
}
