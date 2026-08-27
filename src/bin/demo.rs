use std::collections::HashMap;

use glam::Vec2;
use mobile_gfx::{
    RuntimeContext, SpriteKey, UserState,
    app::App,
    color::Color,
    shapes::{Alignment, DrawRectParams, DrawShapeParams, DrawSpriteParams, Size::Scale, Stroke},
};
use winit::event_loop::EventLoop;

struct MyState {
    test_image: SpriteKey,
    a: f32,
    last_frame: std::time::Instant,
    touches: HashMap<u32, Vec2>,
}

impl UserState for MyState {
    fn create(cc: &mut mobile_gfx::CreationContext) -> Self {
        let test_image = cc.load_image(
            image::load_from_memory(include_bytes!("../../fruit_apple.png"))
                .unwrap()
                .to_rgba8(),
        );

        Self {
            test_image,
            a: 0.0,
            last_frame: std::time::Instant::now(),
            touches: HashMap::new(),
        }
    }

    fn frame(&mut self, ctx: RuntimeContext) {
        let painter = ctx.painter;
        let input = ctx.input;
        let (width, height) = (ctx.frame.width(), ctx.frame.height());

        self.a += (std::time::Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = std::time::Instant::now();

        self.touches = input.touch_map().clone();

        painter.clear_color(Color::DARK_GRAY);

        painter.draw_line(
            Vec2::new(50.0, 0.0),
            Vec2::new(50.0, height),
            Stroke {
                color: Color::WHITE,
                thickness: 1.0,
            },
        );
        painter.draw_line(
            Vec2::new(0.0, 50.0),
            Vec2::new(width, 50.0),
            Stroke {
                color: Color::WHITE,
                thickness: 1.0,
            },
        );
        painter.draw_text("180px", Vec2::new(width - 26.0, 40.0), Color::WHITE);
        painter.draw_text(
            &format!("{}px", height),
            Vec2::new(52.0, height - 12.0),
            Color::WHITE,
        );

        painter.draw_sprite_ex(
            Vec2::new(50.0, 50.0),
            self.test_image,
            DrawSpriteParams {
                alignment: Alignment::CENTER,
                angle: self.a * 2.0,
                size: Scale((self.a.sin() + 2.0) * 2.0),
                ..Default::default()
            },
        );

        painter.draw_rect_ex(
            Vec2::new(100.0, 150.0),
            Vec2::new(40.0, 20.0),
            DrawRectParams {
                shape_params: DrawShapeParams {
                    stroke: Some(Stroke {
                        thickness: 4.0,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                angle: self.a,
                alignment: Alignment::CENTER,
                ..Default::default()
            },
        );

        painter.draw_circle_lines(
            Vec2::new(50.0, 300.0),
            (self.a.sin() + 2.0) * 20.0,
            Stroke {
                thickness: (self.a.sin() + 2.0) * 5.0,
                color: Color::ORANGE,
            },
        );

        for (id, pos) in &self.touches {
            painter.draw_text(
                &format!("id: {}", id),
                pos - Vec2::ONE * 10.0 - Vec2::Y * 12.0,
                Color::WHITE,
            );
            painter.draw_rect(pos - Vec2::ONE * 10.0, Vec2::ONE * 20.0, Color::YELLOW);
        }

        for i in 0..10 {
            painter.draw_rect_ex(
                Vec2::new(i as f32 * 18.0 + 2.0, 350.0),
                Vec2::new(15.0, 24.0),
                DrawRectParams {
                    shape_params: DrawShapeParams::new(
                        Color::WHITE,
                        Stroke {
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                },
            );
            painter.draw_text(
                &format!("{}", ('a' as u8 + i) as char),
                Vec2::new(i as f32 * 18.0 + 6.0, 355.0),
                Color::BLACK,
            );
        }

        painter.draw_poly_ex(
            &[
                Vec2::new(100.0, 100.0),
                Vec2::new(140.0, 160.0),
                Vec2::new(140.0, 250.0),
                Vec2::new(90.0, 170.0),
            ],
            DrawShapeParams::new(
                Color::WHITE,
                Stroke {
                    thickness: 4.0,
                    color: Color::BLACK,
                },
            ),
        );
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}
