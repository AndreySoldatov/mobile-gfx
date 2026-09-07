use std::collections::VecDeque;

use glam::Vec2;
use mobile_gfx::{
    RuntimeContext, UserState,
    app::App,
    color::Color,
    ui::{Button, ButtonState, Slider, Widget},
};
use winit::event_loop::EventLoop;

const WINDOW_SIZE: usize = 16;

struct MyState {
    status_window: VecDeque<ButtonState>,
    a: f32,
    last_frame: std::time::Instant,
    capture: bool,
    speed: f32,
}

impl UserState for MyState {
    fn create(_cc: &mut mobile_gfx::CreationContext) -> Self {
        Self {
            status_window: VecDeque::new(),
            a: 0.0,
            last_frame: std::time::Instant::now(),
            capture: true,
            speed: 1.0,
        }
    }

    fn frame(&mut self, ctx: RuntimeContext) {
        let RuntimeContext {
            input,
            painter,
            ui,
            frame,
        } = ctx;
        let (width, _height) = (frame.width(), frame.height());

        let dt = self.last_frame.elapsed();
        self.last_frame = std::time::Instant::now();
        self.a += dt.as_secs_f32() * self.speed;

        if Button::new_text(Vec2::new(100.0, 10.0), "capture")
            .draw(ui, painter, input)
            .pressed
        {
            self.capture = !self.capture;
        }

        let button_pos = Vec2::new(10.0, 34.0)
            + Vec2::new(
                (((3.0 * self.a * 0.2).cos() + 1.0) * 0.5) * (width - 60.0),
                (((4.0 * self.a * 0.2).sin() + 1.0) * 0.5) * (60.0),
            );

        self.status_window.push_back(
            Button::new_text(button_pos, "button")
                .with_capturing(self.capture)
                .draw(ui, painter, input),
        );
        if self.status_window.len() > WINDOW_SIZE {
            self.status_window.pop_front();
        }

        painter.draw_text(
            &format!("{:#?}", self.status_window[self.status_window.len() - 1]),
            Vec2::new(10.0, 120.0),
            Color::WHITE,
        );

        painter.draw_text("down", Vec2::new(10.0, 180.0), Color::WHITE);
        painter.draw_text("prsd", Vec2::new(40.0, 180.0), Color::WHITE);
        painter.draw_text("rlsd", Vec2::new(70.0, 180.0), Color::WHITE);

        for i in 0..WINDOW_SIZE {
            let status = self.status_window.get(i).unwrap_or(&ButtonState {
                pressed: false,
                down: false,
                released: false,
            });

            if status.down {
                painter.draw_rect(
                    Vec2::new(10.0, 200.0 + i as f32 * 12.0),
                    Vec2::ONE * 10.0,
                    Color::GREEN,
                );
            }

            if status.pressed {
                painter.draw_rect(
                    Vec2::new(40.0, 200.0 + i as f32 * 12.0),
                    Vec2::ONE * 10.0,
                    Color::GREEN,
                );
            }

            if status.released {
                painter.draw_rect(
                    Vec2::new(70.0, 200.0 + i as f32 * 12.0),
                    Vec2::ONE * 10.0,
                    Color::GREEN,
                );
            }
        }

        Slider {
            current: &mut self.speed,
            min: 0.0,
            max: 2.0,
            pos: Vec2::new(10.0, 10.0),
            size: Vec2::new(60.0, 6.0),
        }
        .draw(ui, painter, input);

        painter.draw_text(
            &format!("Speed: {:.2?}", self.speed),
            Vec2::new(10.0, 20.0),
            Color::WHITE,
        );

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}
