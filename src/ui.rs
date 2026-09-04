use std::hash::{Hash, Hasher};

use glam::Vec2;
use rustc_hash::FxHasher;

type WidgetId = u64;

use crate::{
    SpriteKey,
    color::Color,
    input::InputState,
    render::RenderState,
    shapes::{DrawRectParams, DrawShapeParams, Stroke},
    text::DrawTextParams,
};

pub struct UiTheme {
    pub background: Color,
    pub pressed: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
}

pub struct UiState {
    pub theme: UiTheme,
    active: Vec<WidgetId>,
}

pub enum ButtonContent {
    Text(String),
    Icon(SpriteKey),
}

impl Default for ButtonContent {
    fn default() -> Self {
        Self::Text("Button".into())
    }
}

pub struct ButtonParams {
    pub content: ButtonContent,
    pub padding: f32,
    pub capturing: bool,
}

impl ButtonParams {
    fn hash(&self) -> WidgetId {
        let mut hasher = FxHasher::default();
        match &self.content {
            ButtonContent::Icon(icon) => {
                icon.atlas_key.hash(&mut hasher);
            }
            ButtonContent::Text(text) => {
                text.hash(&mut hasher);
            }
        }
        self.capturing.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for ButtonParams {
    fn default() -> Self {
        Self {
            content: Default::default(),
            padding: 4.0,
            capturing: true,
        }
    }
}

pub struct ButtonState {
    pub pressed: bool,
    pub down: bool,
    pub released: bool,
}

impl UiState {
    pub fn new(theme: UiTheme) -> Self {
        Self {
            theme,
            active: vec![],
        }
    }

    fn active(&self, id: &WidgetId) -> bool {
        self.active.contains(id)
    }

    pub fn button(
        &mut self,
        painter: &mut RenderState,
        input: &InputState,
        pos: Vec2,
        params: &ButtonParams,
    ) -> ButtonState {
        // input
        let id = params.hash();

        if self.active(&id) {}

        // draw
        let bound = match &params.content {
            ButtonContent::Text(text) => painter.text_rect(text, DrawTextParams::default()),
            ButtonContent::Icon(icon) => {
                let ar = painter.atlas.entry(icon.atlas_key);
                Vec2::new(ar.px.w as f32, ar.px.h as f32)
            }
        } + Vec2::ONE * params.padding * 2.0;

        painter.draw_rect_ex(
            pos,
            bound,
            DrawRectParams {
                shape_params: DrawShapeParams {
                    fill: Some(self.theme.background),
                    stroke: Some(Stroke {
                        color: self.theme.foreground,
                        thickness: 1.0,
                    }),
                },
                ..Default::default()
            },
        );

        match &params.content {
            ButtonContent::Text(text) => painter.draw_text(
                text,
                pos + Vec2::ONE * params.padding,
                self.theme.foreground,
            ),
            ButtonContent::Icon(icon) => {
                painter.draw_sprite(pos + Vec2::ONE * params.padding, *icon);
            }
        };

        todo!()
    }
}
