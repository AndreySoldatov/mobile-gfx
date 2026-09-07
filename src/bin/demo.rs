use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
};

use glam::Vec2;
use mobile_gfx::{
    RuntimeContext, SpriteKey, UserState,
    app::App,
    color::Color,
    shapes::{Alignment, DrawRectParams, DrawShapeParams, DrawSpriteParams, Stroke},
    ui::{Button, ButtonState, Slider, UiState, UiTheme, Widget},
};
use rustc_hash::FxHasher;
use winit::event_loop::EventLoop;

const WINDOW_SIZE: usize = 16;

struct ThumbStick {
    sprite: SpriteKey,
    outer_radius: f32,
    center: Vec2,
}

impl Widget for ThumbStick {
    type State = Vec2;

    fn draw(
        self,
        ui: &mut UiState,
        painter: &mut mobile_gfx::render::RenderState,
        input: &mobile_gfx::input::InputState,
    ) -> Self::State {
        let w_id = self.id();

        // Input
        let mut result = Vec2::ZERO;
        if let Some(t_id) = ui.active_touch_from_widget(&w_id) {
            if let Some(t_pos) = input.get_touch(t_id) {
                let delta = t_pos - self.center;
                let length = delta.length().min(self.outer_radius);
                let norm = delta.normalize() * length;
                result = norm / self.outer_radius;
            } else {
                ui.remove_active(t_id, w_id);
            }
        } else {
            if let Some((t_id, t_pos)) = input.touch_map().iter().find_map(|(t_id, t_pos)| {
                if (t_pos - self.center).length() <= self.outer_radius {
                    Some((*t_id, t_pos))
                } else {
                    None
                }
            }) && ui.active_widget_from_touch(&t_id).is_none()
            {
                let delta = t_pos - self.center;
                let length = delta.length().min(self.outer_radius);
                let norm = delta.normalize() * length;
                result = norm / self.outer_radius;
                ui.set_active(t_id, w_id);
            }
        }

        // Draw
        painter.draw_circle_ex(
            self.center,
            self.outer_radius,
            DrawShapeParams::new(
                ui.theme.background,
                Stroke {
                    thickness: 1.0,
                    color: ui.theme.foreground,
                },
            ),
        );
        let inner_center = result * (self.outer_radius - self.sprite.width() as f32 * 0.4);

        let angle = result.to_angle();

        painter.draw_sprite_ex(
            inner_center + self.center,
            self.sprite,
            DrawSpriteParams {
                alignment: Alignment::CENTER,
                angle: angle,
                ..Default::default()
            },
        );

        result
    }

    fn id(&self) -> mobile_gfx::ui::WidgetId {
        let mut hasher = FxHasher::default();
        self.sprite.width().hash(&mut hasher);
        self.outer_radius.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

struct MyState {
    status_window: VecDeque<ButtonState>,
    a: f32,
    last_frame: std::time::Instant,
    capture: bool,
    speed: f32,
    ui: UiState,
    stick: SpriteKey,
}

impl UserState for MyState {
    fn create(cc: &mut mobile_gfx::CreationContext) -> Self {
        let sprite = cc.load_image(
            image::load_from_memory(include_bytes!("../../thumbstick.png"))
                .unwrap()
                .to_rgba8(),
        );

        Self {
            status_window: VecDeque::new(),
            a: 0.0,
            last_frame: std::time::Instant::now(),
            capture: true,
            speed: 1.0,
            ui: UiState::new(UiTheme {
                background: Color::BLACK,
                foreground: Color::WHITE,
                pressed: Color::DARK_GRAY,
                primary: Color::RED,
                secondary: Color::BLUE,
            }),
            stick: sprite,
        }
    }

    fn frame(&mut self, ctx: RuntimeContext) {
        let RuntimeContext {
            input,
            painter,
            frame,
        } = ctx;
        let (width, _height) = (frame.width(), frame.height());

        let dt = self.last_frame.elapsed();
        self.last_frame = std::time::Instant::now();
        self.a += dt.as_secs_f32() * self.speed;

        if Button::new_text(Vec2::new(100.0, 10.0), "capture")
            .draw(&mut self.ui, painter, input)
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
                .draw(&mut self.ui, painter, input),
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
            max: 4.0,
            pos: Vec2::new(10.0, 10.0),
            size: Vec2::new(60.0, 6.0),
        }
        .draw(&mut self.ui, painter, input);

        painter.draw_text(
            &format!("Speed: {:.2?}", self.speed),
            Vec2::new(10.0, 20.0),
            Color::WHITE,
        );
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}
