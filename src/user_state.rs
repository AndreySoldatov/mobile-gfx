use image::RgbaImage;

use crate::{error_manager::RuntimeError, render::RenderState};

pub trait UserState {
    fn create(_cc: &mut CreationContext) -> Self;
    fn update(&mut self) {}
    fn draw(&self, _painter: &mut RenderState);
    fn on_error(&mut self, error: RuntimeError) {
        log::error!("Runtime error: {}", error)
    }
}

pub struct CreationContext {
    pub(crate) scale: u32,
    pub(crate) sprites: Vec<RgbaImage>,
}

#[derive(Clone, Copy)]
pub struct SpriteKey(pub(crate) usize);

impl Default for CreationContext {
    fn default() -> Self {
        Self {
            scale: 4,
            sprites: vec![],
        }
    }
}

impl CreationContext {
    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale
    }

    pub fn load_image(&mut self, image: RgbaImage) -> SpriteKey {
        let index = self.sprites.len();
        self.sprites.push(image);
        SpriteKey(index)
    }
}
