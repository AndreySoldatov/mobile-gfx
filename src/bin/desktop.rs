use std::collections::VecDeque;

use glam::{Mat2, Vec2};
#[cfg(not(target_os = "android"))]
use mobile_gfx::app;
use mobile_gfx::{
    color::Color,
    render,
    user_state::{SpriteKey, UserState},
};
#[cfg(not(target_os = "android"))]
use winit::event_loop::EventLoop;

struct MyState {
    clock: std::time::Instant,
    dt: f32,
    time: f32,
    fps_window: VecDeque<f32>,
    cat: SpriteKey,
}

impl UserState for MyState {
    fn create(cc: &mut mobile_gfx::user_state::CreationContext) -> Self {
        cc.set_scale(2);
        let image = image::load_from_memory(include_bytes!("assets/cat-dithered.png"))
            .unwrap()
            .to_rgba8();
        let image = cc.load_image(image);
        Self {
            clock: std::time::Instant::now(),
            dt: 0.0,
            time: 0.0,
            fps_window: VecDeque::new(),
            cat: image,
        }
    }
    fn update(&mut self) {
        self.dt = self.clock.elapsed().as_secs_f32();
        self.time += self.dt;
        self.clock = std::time::Instant::now();

        self.fps_window.push_back(self.dt);
        if self.fps_window.len() > 40 {
            self.fps_window.pop_front();
        }
        let fps = 1.0
            / (self.fps_window.iter().fold(0.0, |acc, v| acc + v) / (self.fps_window.len() as f32));
        log::info!("FPS: {}", fps);
    }
    fn draw(&self, painter: &mut render::RenderState) {
        painter.clear_color(Color::BLACK);
        let w = painter.width();
        let h = painter.height();
        let rmat = Mat2::from_angle(self.time);

        painter.draw_triangle(
            rmat * Vec2::new(-10.0, 10.0) + Vec2::new(w * 0.75, h * 0.75),
            rmat * Vec2::new(0.0, -10.0) + Vec2::new(w * 0.75, h * 0.75),
            rmat * Vec2::new(10.0, 10.0) + Vec2::new(w * 0.75, h * 0.75),
            Color::GRAY,
        );

        painter.draw_line(
            rmat * Vec2::new(0.0, -40.0) + Vec2::new(w / 2.0, h / 2.0),
            rmat * Vec2::new(0.0, 40.0) + Vec2::new(w / 2.0, h / 2.0),
            1.0,
            Color::RED,
        );

        painter.draw_line(Vec2::new(0.0, 0.0), Vec2::new(w, h), 1.0, Color::WHITE);
        painter.draw_line(Vec2::new(40.0, 0.0), Vec2::new(40.0, h), 1.0, Color::WHITE);
        painter.draw_line(Vec2::new(0.0, 40.0), Vec2::new(w, 40.0), 1.0, Color::WHITE);

        painter.draw_sprite(
            Vec2::new(
                (self.time.sin() + 1.0) * 20.0 + 20.0,
                (self.time.cos() + 1.0) * 20.0 + 20.0,
            ),
            self.cat,
            Color::WHITE,
        );
    }
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = app::App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "android")]
fn main() {}
