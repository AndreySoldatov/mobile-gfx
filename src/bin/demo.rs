use std::collections::VecDeque;

use glam::Vec2;
use mobile_gfx::{
    RuntimeContext, SpriteKey, UserState,
    app::App,
    color::Color,
    shapes::{Alignment, DrawSpriteParams},
    ui::ButtonParams,
};
use winit::event_loop::EventLoop;

struct MyState {}

impl UserState for MyState {
    fn create(cc: &mut mobile_gfx::CreationContext) -> Self {
        Self {}
    }

    fn frame(&mut self, ctx: RuntimeContext) {
        let RuntimeContext {
            input,
            painter,
            ui,
            frame,
        } = ctx;
        let (width, height) = (ctx.frame.width(), ctx.frame.height());

        ui.button(
            painter,
            input,
            Vec2::new(10.0, 10.0),
            &ButtonParams::default(),
        );
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::<MyState>::new();
    event_loop.run_app(&mut app).unwrap();
}
