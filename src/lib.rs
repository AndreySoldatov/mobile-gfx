use glam::Vec2;
use image::RgbaImage;
use slotmap::SlotMap;

use crate::{atlas::AtlasKey, error_manager::RuntimeError, input::InputState, render::RenderState};

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
pub mod input;
pub mod render;
pub mod shapes;
pub mod text;

pub trait UserState {
    fn create(_cc: &mut CreationContext) -> Self;
    #[allow(unused)]
    fn update(&mut self, input: &InputState, ctx: &RuntimeContext) {}
    fn draw(&self, painter: &mut RenderState, ctx: &RuntimeContext);
    fn on_error(&mut self, error: RuntimeError) {
        log::error!("Runtime error: {}", error)
    }
}

pub struct RuntimeContext {
    pub width: f32,
    pub height: f32,
}

pub struct CreationContext {
    pub(crate) scale: u32,
    pub(crate) atlas_staging: SlotMap<AtlasKey, RgbaImage>,
    pub(crate) physical_size: Vec2,
}

#[derive(Clone, Copy)]
pub struct SpriteKey(pub(crate) AtlasKey);

impl Default for CreationContext {
    fn default() -> Self {
        Self {
            scale: 4,
            atlas_staging: SlotMap::with_key(),
            physical_size: Vec2::ZERO,
        }
    }
}

impl CreationContext {
    pub fn set_scale(&mut self, scale: u32) -> Vec2 {
        self.scale = scale;
        self.physical_size / (scale as f32)
    }

    pub fn load_image(&mut self, image: RgbaImage) -> SpriteKey {
        SpriteKey(self.atlas_staging.insert(image))
    }

    pub fn physical_size(&self) -> Vec2 {
        self.physical_size
    }
}
