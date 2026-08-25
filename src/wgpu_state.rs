use std::sync::Arc;
use winit::window::Window;

use crate::owned_window_handle::OwnedWindowHandle;

pub struct WgpuState {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct WgpuSurface {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

pub fn wgpu_init(window: Arc<Window>) -> anyhow::Result<(WgpuState, WgpuSurface)> {
    let (instance, surface, adapter) =
        try_init_backend(wgpu::Backends::VULKAN, &window).or_else(|e| {
            log::warn!("Vulkan backend unavailable, trying GL fallback. Error: {e}");
            try_init_backend(wgpu::Backends::GL, &window)
        })?;

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        required_limits: adapter.limits(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
    }))?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
        color_space: wgpu::SurfaceColorSpace::Auto,
    };
    surface.configure(&device, &config);

    Ok((
        WgpuState {
            instance,
            adapter,
            device,
            queue,
        },
        WgpuSurface { surface, config },
    ))
}

fn try_init_backend(
    backends: wgpu::Backends,
    window: &Arc<Window>,
) -> anyhow::Result<(wgpu::Instance, wgpu::Surface<'static>, wgpu::Adapter)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    let owh = OwnedWindowHandle::new(window.clone()).unwrap();
    let surface = instance.create_surface(owh).unwrap();

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        #[cfg(target_os = "android")]
        power_preference: wgpu::PowerPreference::LowPower,
        #[cfg(not(target_os = "android"))]
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
        apply_limit_buckets: true,
    }))?;

    Ok((instance, surface, adapter))
}

pub fn create_wgpu_surface(wgpu_state: &WgpuState, window: Arc<Window>) -> WgpuSurface {
    let owh = OwnedWindowHandle::new(window.clone()).unwrap();
    let surface = wgpu_state.instance.create_surface(owh).unwrap();

    let surface_caps = surface.get_capabilities(&wgpu_state.adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
        color_space: wgpu::SurfaceColorSpace::Auto,
    };
    surface.configure(&wgpu_state.device, &config);

    WgpuSurface { surface, config }
}
