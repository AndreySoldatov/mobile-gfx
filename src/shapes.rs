use crate::{
    color::Color,
    render::{RenderState, Vertex},
    user_state::SpriteKey,
};
use glam::Vec2;

impl RenderState {
    pub fn white_pixel(&self) -> [f32; 2] {
        self.atlas.entries[&self.system_atlas_regions.white_pixel].uv[..2]
            .try_into()
            .unwrap()
    }

    pub fn draw_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, c: Color) {
        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
            ],
            &[0, 1, 2],
        );
    }

    pub fn draw_quad(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, c: Color) {
        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
                Vertex {
                    pos: [p3.x, p3.y],
                    col: [c.r, c.g, c.b],
                    uv: self.white_pixel(),
                },
            ],
            &[0, 1, 2, 2, 3, 0],
        );
    }

    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, t: f32, c: Color) {
        let n = Vec2::new(-(p1.y - p0.y), p1.x - p0.x).normalize_or_zero() * t * 0.5;
        let pq0 = p0 - n;
        let pq1 = p0 + n;
        let pq2 = p1 - n;
        let pq3 = p1 + n;
        self.draw_quad(pq0, pq2, pq3, pq1, c);
    }

    pub fn draw_rect(&mut self, pos: Vec2, dims: Vec2, c: Color) {
        let p0 = pos;
        let p1 = pos + dims * Vec2::X;
        let p2 = pos + dims * Vec2::Y;
        let p3 = pos + dims;
        self.draw_quad(p0, p1, p3, p2, c);
    }

    pub fn draw_sprite(&mut self, pos: Vec2, image: SpriteKey, c: Color) {
        let atlas_entry = &self.atlas.entries[&image.0];
        let sprite_dims = Vec2::new(atlas_entry.px.w as f32, atlas_entry.px.h as f32);
        let p0 = pos;
        let p1 = pos + sprite_dims * Vec2::X;
        let p2 = pos + sprite_dims * Vec2::Y;
        let p3 = pos + sprite_dims;
        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [c.r, c.g, c.b],
                    uv: [atlas_entry.uv[0], atlas_entry.uv[1]],
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [c.r, c.g, c.b],
                    uv: [atlas_entry.uv[0], atlas_entry.uv[3]],
                },
                Vertex {
                    pos: [p3.x, p3.y],
                    col: [c.r, c.g, c.b],
                    uv: [atlas_entry.uv[2], atlas_entry.uv[3]],
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [c.r, c.g, c.b],
                    uv: [atlas_entry.uv[2], atlas_entry.uv[1]],
                },
            ],
            &[0, 1, 2, 2, 3, 0],
        );
    }
}
