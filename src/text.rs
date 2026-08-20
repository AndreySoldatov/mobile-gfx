use glam::Vec2;

use crate::{color::Color, render::RenderState};

impl RenderState {
    pub fn draw_char(&mut self, c: char, pos: Vec2, color: Color) {
        let glyph_entry = self.atlas.entry(self.system_font.get(c));
        self.draw_atlas_entry(pos, glyph_entry, color);
    }

    pub fn draw_text(&mut self, text: &str, pos: Vec2, color: Color) {
        if text.is_empty() {
            return;
        }
        let line_height = self
            .atlas
            .entry(self.system_font.get(text.chars().next().unwrap()))
            .px
            .h as f32;

        let mut cursor = pos;
        for c in text.chars() {
            match c {
                '\n' => {
                    cursor.x = pos.x;
                    cursor.y += line_height;
                }
                _ => {
                    let glyph_entry = self.atlas.entry(self.system_font.get(c));
                    self.draw_atlas_entry(cursor, glyph_entry, color);
                    cursor.x += glyph_entry.px.w as f32 - 1.0;
                }
            }
        }
    }
}
