use glam::Vec2;
use image::RgbaImage;
use slotmap::SlotMap;

use crate::{atlas::AtlasKey, input::InputState, render::RenderState};

mod atlas;
mod blit;
mod buffer;
mod font;
mod owned_window_handle;
mod runtime;
mod utils;
mod wgpu_state;

pub mod app;
pub mod color;
pub mod input;
pub mod render;
pub mod shapes;
pub mod text;

pub trait UserState {
    fn create(_cc: &mut CreationContext) -> Self;
    fn frame(&mut self, ctx: RuntimeContext);
}

pub struct RuntimeContext<'a> {
    pub input: &'a InputState,
    pub painter: &'a mut RenderState,
    pub frame: &'a FrameContext,
}

#[derive(Default)]
pub struct FrameContext {
    frame_size: Vec2,
}

impl FrameContext {
    pub fn width(&self) -> f32 {
        self.frame_size.x
    }

    pub fn height(&self) -> f32 {
        self.frame_size.y
    }
}

#[derive(Clone, Copy)]
pub struct SpriteKey {
    pub(crate) atlas_key: AtlasKey,
    width: u32,
    height: u32,
}

impl SpriteKey {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct CreationContext {
    pub(crate) atlas_staging: SlotMap<AtlasKey, RgbaImage>,
    pub frame: FrameContext,
}

impl Default for CreationContext {
    fn default() -> Self {
        Self {
            atlas_staging: SlotMap::with_key(),
            frame: FrameContext::default(),
        }
    }
}

impl CreationContext {
    pub fn load_image(&mut self, image: RgbaImage) -> SpriteKey {
        SpriteKey {
            width: image.width(),
            height: image.height(),
            atlas_key: self.atlas_staging.insert(image),
        }
    }
}
