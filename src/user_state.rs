use crate::render::RenderState;

pub trait UserState {
    fn update(&mut self);
    fn draw(&self, painter: &RenderState);
}
