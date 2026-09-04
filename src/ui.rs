use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use glam::Vec2;
use rustc_hash::FxHasher;

type WidgetId = u64;

use crate::{
    SpriteKey,
    color::Color,
    input::{InputState, TouchId},
    render::RenderState,
    shapes::{DrawRectParams, DrawShapeParams, Stroke},
    text::DrawTextParams,
    utils::{Rect, contains},
};

pub struct UiTheme {
    pub background: Color,
    pub pressed: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
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

#[derive(Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub down: bool,
    pub released: bool,
}

pub struct UiState {
    pub theme: UiTheme,
    active: HashSet<(TouchId, WidgetId)>,
}

impl UiState {
    pub fn new(theme: UiTheme) -> Self {
        Self {
            theme,
            active: HashSet::new(),
        }
    }

    fn active(&self, id: &WidgetId) -> Option<TouchId> {
        self.active
            .iter()
            .find_map(|(t_id, w_id)| if w_id == id { Some(*t_id) } else { None })
    }

    pub fn button(
        &mut self,
        painter: &mut RenderState,
        input: &InputState,
        pos: Vec2,
        params: &ButtonParams,
    ) -> ButtonState {
        let bounds = match &params.content {
            ButtonContent::Text(text) => painter.text_rect(text, DrawTextParams::default()),
            ButtonContent::Icon(icon) => {
                let ar = painter.atlas.entry(icon.atlas_key);
                Vec2::new(ar.px.w as f32, ar.px.h as f32)
            }
        } + Vec2::ONE * params.padding * 2.0;
        let rect = Rect {
            tl: pos,
            wh: bounds,
        };

        // input
        let id = params.hash();
        let mut res_state = ButtonState {
            down: false,
            pressed: false,
            released: false,
        };

        if let Some(touch) = self.active(&id) {
            if let Some(touch_pos) = input.touch_map().iter().find_map(|(t_id, touch_pos)| {
                if *t_id == touch {
                    Some(*touch_pos)
                } else {
                    None
                }
            }) {
                res_state.down = true;
                if !params.capturing {
                    if !contains(rect, touch_pos) {
                        res_state.released = true;
                        res_state.down = false;
                        self.active.remove(&(touch, id));
                    }
                }
            } else {
                res_state.released = true;
                self.active.remove(&(touch, id));
            }
        } else {
            if let Some(touch) = input.touch_map().iter().find_map(|(t_id, touch_pos)| {
                if contains(rect, *touch_pos) {
                    Some(*t_id)
                } else {
                    None
                }
            }) {
                self.active.insert((touch, id));
                res_state.pressed = true;
                res_state.down = true;
            }
        }

        // draw
        painter.draw_rect_ex(
            pos,
            bounds,
            DrawRectParams {
                shape_params: DrawShapeParams {
                    fill: Some(if res_state.down {
                        self.theme.pressed
                    } else {
                        self.theme.background
                    }),
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

        res_state
    }
}
