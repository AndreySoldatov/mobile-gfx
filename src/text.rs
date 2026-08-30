use glam::Vec2;

use crate::{
    atlas::AtlasRegion,
    color::Color,
    render::{RenderState, Vertex},
};

impl RenderState {
    pub(crate) fn draw_atlas_entry(&mut self, pos: Vec2, atlas_entry: AtlasRegion, c: Color) {
        let sprite_dims = Vec2::new(atlas_entry.px.w as f32, atlas_entry.px.h as f32);
        let p0 = pos;
        let p1 = pos + sprite_dims * Vec2::X;
        let p2 = pos + sprite_dims * Vec2::Y;
        let p3 = pos + sprite_dims;

        let uvx = Vec2::new(atlas_entry.uv.1.x, atlas_entry.uv.0.y);
        let uvy = Vec2::new(atlas_entry.uv.0.x, atlas_entry.uv.1.y);
        self.append_vertices(
            &[
                Vertex::new(p0, c, atlas_entry.uv.0),
                Vertex::new(p2, c, uvy),
                Vertex::new(p3, c, atlas_entry.uv.1),
                Vertex::new(p1, c, uvx),
            ],
            &[0, 1, 2, 2, 3, 0],
        );
    }

    pub fn draw_char(&mut self, c: char, pos: Vec2, color: Color) {
        let glyph_entry = self.atlas.entry(self.system_font.get(c));
        self.draw_atlas_entry(pos, glyph_entry, color);
    }

    pub fn draw_text_ex(&mut self, text: &str, pos: Vec2, color: Color, params: DrawTextParams) {
        if text.is_empty() {
            return;
        }
        let line_height = self
            .atlas
            .entry(self.system_font.get(text.chars().next().unwrap()))
            .px
            .h as f32
            + params.line_height as f32;

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
                    cursor.x += glyph_entry.px.w as f32 - 1.0 + (params.char_spacing as f32);
                }
            }
        }
    }

    pub fn draw_text(&mut self, text: &str, pos: Vec2, color: Color) {
        self.draw_text_ex(text, pos, color, DrawTextParams::default());
    }

    pub fn text_rect(&self, text: &str, params: DrawTextParams) -> Vec2 {
        let line_height = self
            .atlas
            .entry(self.system_font.get(text.chars().next().unwrap()))
            .px
            .h as f32
            + params.line_height as f32;

        text.lines().fold(Vec2::ZERO, |acc, l| {
            let max_width = l
                .chars()
                .map(|c| self.atlas.entry(self.system_font.get(c)).px.w as f32)
                .reduce(|acc, width| acc + width - 1.0)
                .unwrap_or(0.0)
                .max(acc.x);

            Vec2::new(max_width, acc.y + line_height)
        })
    }
}

pub struct DrawTextParams {
    pub line_height: u32,
    pub char_spacing: u32,
}

impl Default for DrawTextParams {
    fn default() -> Self {
        Self {
            line_height: 0,
            char_spacing: 0,
        }
    }
}
