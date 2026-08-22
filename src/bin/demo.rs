use glam::Vec2;
use mobile_gfx::{
    SpriteKey, UserState,
    app::App,
    color::Color,
    shapes::{Alignment, DrawSpriteParams, Size::Scale},
};
use winit::event_loop::EventLoop;

struct MyState {
    test_image: SpriteKey,
    a: f32,
    last_frame: std::time::Instant,
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
        }
    }

    fn update(
        &mut self,
        _input: &mobile_gfx::input::InputState,
        _ctx: &mobile_gfx::RuntimeContext,
    ) {
        self.a += (std::time::Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = std::time::Instant::now();
    }

    fn draw(
        &self,
        painter: &mut mobile_gfx::render::RenderState,
        ctx: &mobile_gfx::RuntimeContext,
    ) {
        painter.draw_line(
            Vec2::new(50.0, 0.0),
            Vec2::new(50.0, ctx.height),
            1.0,
            Color::WHITE,
        );
        painter.draw_line(
            Vec2::new(0.0, 50.0),
            Vec2::new(ctx.width, 50.0),
            1.0,
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
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}
