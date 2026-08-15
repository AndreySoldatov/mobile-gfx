use std::collections::VecDeque;

#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use glam::{Mat2, Vec2};
#[cfg(target_os = "android")]
use log::LevelFilter;
#[cfg(target_os = "android")]
use mobile_gfx::app;
use mobile_gfx::{render, shapes::Color, user_state::UserState};
#[cfg(target_os = "android")]
use winit::{
    event_loop::EventLoop,
    platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp},
};

struct MyState {
    clock: std::time::Instant,
    dt: f32,
    time: f32,
    fps_window: VecDeque<f32>,
}

impl MyState {
    fn new() -> Self {
        Self {
            clock: std::time::Instant::now(),
            dt: 0.0,
            time: 0.0,
            fps_window: VecDeque::new(),
        }
    }
}

impl UserState for MyState {
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
    fn draw(&self, painter: &render::RenderState) {
        let w = painter.width();
        let h = painter.height();
        let rmat = Mat2::from_angle(self.time);
        painter.draw_triangle(
            rmat * Vec2::new(-40.0, 40.0) + Vec2::new(w / 2.0, h / 2.0),
            rmat * Vec2::new(0.0, -40.0) + Vec2::new(w / 2.0, h / 2.0),
            rmat * Vec2::new(40.0, 40.0) + Vec2::new(w / 2.0, h / 2.0),
            Color::WHITE,
        );
    }
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("mytag")
            .with_filter(FilterBuilder::new().parse("android_demo=info").build()),
    );

    let event_loop = EventLoop::with_user_event()
        .with_android_app(app)
        .build()
        .unwrap();

    let mut app = app::App::new(Box::new(MyState::new()));
    event_loop.run_app(&mut app).unwrap();
}
