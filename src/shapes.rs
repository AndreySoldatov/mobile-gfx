use crate::{
    color::Color,
    render::{RenderState, Vertex},
};
use glam::Vec2;

impl RenderState {
    pub fn draw_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, c: Color) {
        self.append_vertices(
            &[
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
                Vertex {
                    pos: [p3.x, p3.y],
                    col: [c.r, c.g, c.b],
                    uv: [0.0, 0.0],
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
}
