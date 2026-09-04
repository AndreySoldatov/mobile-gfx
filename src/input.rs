use std::collections::HashMap;

use glam::Vec2;
use winit::event::{ElementState, Touch};

use crate::utils::{contains, factor, integer_fit};

pub type TouchId = u32;

const MOUSE_ID: u32 = 1000;

pub struct InputState {
    table: HashMap<TouchId, Vec2>,
    physical_size: Vec2,
    logical_size: Vec2,
    mouse_state: ElementState,
}

impl InputState {
    pub(crate) fn new(p_size: Vec2, l_size: Vec2) -> Self {
        Self {
            table: HashMap::new(),
            physical_size: p_size,
            logical_size: l_size,
            mouse_state: ElementState::Released,
        }
    }

    fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        let rect = integer_fit(self.physical_size, self.logical_size);
        if contains(rect, pos) {
            Some((pos - rect.tl) / factor(self.physical_size, self.logical_size))
        } else {
            None
        }
    }

    pub(crate) fn touch_event(&mut self, touch: Touch) {
        match touch.phase {
            winit::event::TouchPhase::Started | winit::event::TouchPhase::Moved => {
                if let Some(np) =
                    self.normalize_pos(Vec2::new(touch.location.x as f32, touch.location.y as f32))
                {
                    self.table.insert(touch.id as u32, np);
                } else {
                    self.table.remove(&(touch.id as u32));
                }
            }
            winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                self.table.remove(&(touch.id as u32));
            }
        }
    }

    pub(crate) fn mouse_event(&mut self, cursor_pos: Vec2, state: ElementState) {
        self.mouse_state = state;
        match state {
            ElementState::Pressed => {
                if let Some(np) = self.normalize_pos(cursor_pos) {
                    self.table.insert(MOUSE_ID, np);
                }
            }
            ElementState::Released => {
                self.table.remove(&MOUSE_ID);
            }
        }
    }

    pub(crate) fn mouse_moved(&mut self, cursor_pos: Vec2) {
        if let Some(np) = self.normalize_pos(cursor_pos)
            && self.mouse_state == ElementState::Pressed
        {
            self.table.insert(MOUSE_ID, np);
        } else {
            self.table.remove(&MOUSE_ID);
        }
    }

    pub fn touch_map(&self) -> &HashMap<TouchId, Vec2> {
        &self.table
    }

    pub(crate) fn resize(&mut self, new_p_size: Vec2) {
        self.physical_size = new_p_size;
    }
}
