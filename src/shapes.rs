use crate::render::{RenderState, Vertex};
use glam::Vec2;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
}

impl RenderState {
    pub fn draw_triangle(&self, p0: Vec2, p1: Vec2, p2: Vec2, c: Color) {
        self.append_vertices(&[
            Vertex {
                pos: [p0.x, p0.y],
                col: [c.r, c.g, c.b],
                uv: [0.0, 0.0],
            },
            Vertex {
                pos: [p1.x, p1.y],
                col: [c.r, c.g, c.b],
                uv: [0.0, 0.0],
            },
            Vertex {
                pos: [p2.x, p2.y],
                col: [c.r, c.g, c.b],
                uv: [0.0, 0.0],
            },
        ]);
    }
}
