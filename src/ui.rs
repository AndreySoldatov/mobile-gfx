use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use glam::Vec2;
use rustc_hash::FxHasher;

pub type WidgetId = u64;

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

    pub fn active_touch_from_widget(&self, id: &WidgetId) -> Option<TouchId> {
        self.active
            .iter()
            .find_map(|(t_id, w_id)| if w_id == id { Some(*t_id) } else { None })
    }

    pub fn active_widget_from_touch(&self, touch: &TouchId) -> Option<WidgetId> {
        self.active
            .iter()
            .find_map(|(t_id, w_id)| if t_id == touch { Some(*w_id) } else { None })
    }

    pub fn set_active(&mut self, t_id: TouchId, w_id: WidgetId) {
        self.active.insert((t_id, w_id));
    }

    pub fn remove_active(&mut self, t_id: TouchId, w_id: WidgetId) {
        self.active.remove(&(t_id, w_id));
    }
}

pub trait Widget {
    type State;
    fn draw(self, ui: &mut UiState, painter: &mut RenderState, input: &InputState) -> Self::State;
    fn id(&self) -> WidgetId;
}

#[derive(Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub down: bool,
    pub released: bool,
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

pub struct Button {
    pub pos: Vec2,
    pub content: ButtonContent,
    pub padding: f32,
    pub capturing: bool,
}

impl Button {
    pub fn new_text(pos: Vec2, content: &str) -> Self {
        Self {
            pos,
            content: ButtonContent::Text(content.into()),
            capturing: true,
            padding: 4.0,
        }
    }

    pub fn new_icon(pos: Vec2, content: SpriteKey) -> Self {
        Self {
            pos,
            content: ButtonContent::Icon(content),
            capturing: true,
            padding: 4.0,
        }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_capturing(mut self, capturing: bool) -> Self {
        self.capturing = capturing;
        self
    }
}

impl Widget for Button {
    type State = ButtonState;

    fn draw(self, ui: &mut UiState, painter: &mut RenderState, input: &InputState) -> Self::State {
        let bounds = match &self.content {
            ButtonContent::Text(text) => painter.text_rect(text, DrawTextParams::default()),
            ButtonContent::Icon(icon) => Vec2::new(icon.width() as f32, icon.height() as f32),
        } + Vec2::ONE * self.padding * 2.0;
        let rect = Rect {
            tl: self.pos,
            wh: bounds,
        };

        // input
        let id = self.id();
        let mut res_state = ButtonState {
            down: false,
            pressed: false,
            released: false,
        };

        if let Some(touch) = ui.active_touch_from_widget(&id) {
            if let Some(touch_pos) = input.get_touch(touch) {
                res_state.down = true;
                if !self.capturing {
                    if !contains(rect, touch_pos) {
                        res_state.released = true;
                        res_state.down = false;
                        ui.remove_active(touch, id);
                    }
                }
            } else {
                res_state.released = true;
                ui.remove_active(touch, id);
            }
        } else {
            if let Some(touch) = input.touch_map().iter().find_map(|(t_id, touch_pos)| {
                if contains(rect, *touch_pos) {
                    Some(*t_id)
                } else {
                    None
                }
            }) && ui.active_widget_from_touch(&touch).is_none()
            {
                ui.set_active(touch, id);
                res_state.pressed = true;
                res_state.down = true;
            }
        }

        // draw
        painter.draw_rect_ex(
            self.pos,
            bounds,
            DrawRectParams {
                shape_params: DrawShapeParams {
                    fill: Some(if res_state.down {
                        ui.theme.pressed
                    } else {
                        ui.theme.background
                    }),
                    stroke: Some(Stroke {
                        color: ui.theme.foreground,
                        thickness: 1.0,
                    }),
                },
                ..Default::default()
            },
        );

        match &self.content {
            ButtonContent::Text(text) => painter.draw_text(
                text,
                self.pos + Vec2::ONE * self.padding,
                ui.theme.foreground,
            ),
            ButtonContent::Icon(icon) => {
                painter.draw_sprite(self.pos + Vec2::ONE * self.padding, *icon);
            }
        };

        res_state
    }

    fn id(&self) -> WidgetId {
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

pub struct Slider<'a> {
    pub pos: Vec2,
    pub current: &'a mut f32,
    pub min: f32,
    pub max: f32,
    pub size: Vec2,
}

impl<'a> Widget for Slider<'a> {
    type State = ();

    fn draw(self, ui: &mut UiState, painter: &mut RenderState, input: &InputState) -> Self::State {
        let w_id = self.id();
        let rect = Rect {
            tl: self.pos,
            wh: self.size,
        };

        // Update
        if let Some(t_id) = ui.active_touch_from_widget(&w_id) {
            if let Some(touch_pos) = input.get_touch(t_id) {
                let ratio = (touch_pos.x - self.pos.x).clamp(0.0, self.size.x) / self.size.x;
                *self.current = self.min + ratio * (self.max - self.min);
            } else {
                ui.remove_active(t_id, w_id);
            }
        } else {
            if let Some((t_id, t_pos)) = input.touch_map().iter().find_map(|(id, pos)| {
                if contains(rect, *pos) {
                    Some((*id, *pos))
                } else {
                    None
                }
            }) && ui.active_widget_from_touch(&t_id).is_none()
            {
                ui.set_active(t_id, w_id);
                let ratio = (t_pos.x - self.pos.x).clamp(0.0, self.size.x) / self.size.x;
                *self.current = self.min + ratio * (self.max - self.min);
            }
        }

        // Draw
        painter.draw_rect_ex(
            self.pos,
            self.size,
            DrawRectParams {
                shape_params: DrawShapeParams::new(
                    ui.theme.background,
                    Stroke {
                        thickness: 1.0,
                        color: ui.theme.foreground,
                    },
                ),
                ..Default::default()
            },
        );
        let ratio = (*self.current - self.min) / (self.max - self.min);
        painter.draw_rect(
            self.pos,
            Vec2::new(ratio * self.size.x, self.size.y),
            ui.theme.foreground,
        );
    }

    fn id(&self) -> WidgetId {
        let mut hasher = FxHasher::default();
        self.pos.x.to_bits().hash(&mut hasher);
        self.pos.y.to_bits().hash(&mut hasher);
        self.size.x.to_bits().hash(&mut hasher);
        self.size.y.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}
