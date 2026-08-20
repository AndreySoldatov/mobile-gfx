use image::RgbaImage;
use slotmap::SlotMap;

use crate::{atlas::AtlasKey, error_manager::RuntimeError, render::RenderState};

mod atlas;
mod blit;
mod buffer;
mod error_manager;
mod font;
mod owned_window_handle;
mod runtime;
mod wgpu_state;

pub mod app;
pub mod color;
pub mod render;
pub mod shapes;
pub mod text;

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
    pub(crate) atlas_staging: SlotMap<AtlasKey, RgbaImage>,
}

#[derive(Clone, Copy)]
pub struct SpriteKey(pub(crate) AtlasKey);

impl Default for CreationContext {
    fn default() -> Self {
        Self {
            scale: 4,
            atlas_staging: SlotMap::with_key(),
        }
    }
}

impl CreationContext {
    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale
    }

    pub fn load_image(&mut self, image: RgbaImage) -> SpriteKey {
        SpriteKey(self.atlas_staging.insert(image))
    }
}
