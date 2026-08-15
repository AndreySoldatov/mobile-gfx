use glam::Vec2;
#[cfg(not(target_os = "android"))]
use mobile_gfx::app;
use mobile_gfx::{render, shapes::Color, user_state::UserState};
#[cfg(not(target_os = "android"))]
use winit::event_loop::EventLoop;

struct MyState {}
impl UserState for MyState {
    fn update(&mut self) {}
    fn draw(&self, painter: &render::RenderState) {
        let w = painter.width();
        let h = painter.height();
        painter.draw_triangle(
            Vec2::new(w / 2.0 - w / 4.0, h / 2.0 + h / 4.0),
            Vec2::new(w / 2.0, h / 2.0 - h / 4.0),
            Vec2::new(w / 2.0 + w / 4.0, h / 2.0 + h / 4.0),
            Color::WHITE,
        );
    }
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = app::App::new(Box::new(MyState {}));
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "android")]
fn main() {}
