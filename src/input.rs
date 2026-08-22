use std::collections::HashMap;

use glam::Vec2;
use winit::event::{ElementState, Touch};

const MOUSE_ID: u32 = 1000;

pub struct InputState {
    table: HashMap<u32, Vec2>,
    physical_screen_size: Vec2,
    logical_screen_size: Vec2,
}

impl InputState {
    pub(crate) fn new(p_size: Vec2, l_size: Vec2) -> Self {
        Self {
            table: HashMap::new(),
            physical_screen_size: p_size,
            logical_screen_size: l_size,
        }
    }

    fn rescale_size(&self, s: Vec2) -> Vec2 {
        (s / self.physical_screen_size) * self.logical_screen_size
    }

    pub(crate) fn touch_event(&mut self, touch: Touch) {
        match touch.phase {
            winit::event::TouchPhase::Started | winit::event::TouchPhase::Moved => {
                self.table.insert(
                    touch.id as u32,
                    self.rescale_size(Vec2::new(touch.location.x as f32, touch.location.y as f32)),
                );
            }
            winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                self.table.remove(&(touch.id as u32));
            }
        }
    }

    pub(crate) fn mouse_event(&mut self, cursor_pos: Vec2, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.table.insert(MOUSE_ID, self.rescale_size(cursor_pos));
            }
            ElementState::Released => {
                self.table.remove(&MOUSE_ID);
            }
        }
    }

    pub(crate) fn mouse_moved(&mut self, cursor_pos: Vec2) {
        self.table.entry(MOUSE_ID).and_modify(|v| *v = cursor_pos);
    }

    pub fn touch_map(&self) -> &HashMap<u32, Vec2> {
        &self.table
    }

    pub(crate) fn resize(&mut self, new_p_size: Vec2, new_l_size: Vec2) {
        self.physical_screen_size = new_p_size;
        self.logical_screen_size = new_l_size;
    }
}
