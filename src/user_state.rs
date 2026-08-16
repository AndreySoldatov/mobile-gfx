use crate::render::RenderState;

pub trait UserState {
    fn on_create(&self) -> CreationContext {
        CreationContext::default()
    }
    fn update(&mut self);
    fn draw(&self, painter: &mut RenderState);
}

pub struct CreationContext {
    pub scale: u32,
}

impl Default for CreationContext {
    fn default() -> Self {
        Self { scale: 4 }
    }
}
