#[cfg(not(target_os = "android"))]
use mobile_gfx::app;
#[cfg(not(target_os = "android"))]
use winit::event_loop::EventLoop;

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = app::App::new();
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "android")]
fn main() {}
