use crate::{
    SpriteKey,
    atlas::AtlasRegion,
    color::Color,
    render::{RenderState, Vertex},
};
use glam::Vec2;

const EPS: f32 = 0.001;

impl RenderState {
    pub fn white_pixel(&self) -> [f32; 2] {
        self.atlas.entry(self.white_pixel).uv[..2]
            .try_into()
            .unwrap()
    }

    pub fn draw_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, c: Color) {
        let wp = self.white_pixel();
        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
            ],
            &[0, 1, 2],
        );
    }

    pub fn draw_quad(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, c: Color) {
        let wp = self.white_pixel();
        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
                },
                Vertex {
                    pos: [p3.x, p3.y],
                    col: [c.r, c.g, c.b],
                    uv: wp,
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

    pub(crate) fn draw_atlas_entry(&mut self, pos: Vec2, atlas_entry: AtlasRegion, c: Color) {
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

    pub fn draw_sprite(&mut self, pos: Vec2, sprite: SpriteKey) {
        let atlas_entry = self.atlas.entry(sprite.atlas_key);
        self.draw_atlas_entry(pos, atlas_entry, Color::WHITE);
    }

    pub fn draw_sprite_ex(&mut self, pos: Vec2, sprite: SpriteKey, params: DrawSpriteParams) {
        let dims = match params.size {
            Size::Dimentions(size) => size,
            Size::Scale(scale) => Vec2::new(
                sprite.width() as f32 * scale,
                sprite.height() as f32 * scale,
            ),
        };
        let mut p0 = Vec2::new(
            match params.alignment.hor {
                HorizontalAlignment::Left => 0.0,
                HorizontalAlignment::Center => -dims.x * 0.5,
                HorizontalAlignment::Right => -dims.x,
            },
            match params.alignment.ver {
                VerticalAlignment::Top => 0.0,
                VerticalAlignment::Center => -dims.y * 0.5,
                VerticalAlignment::Bottom => -dims.y,
            },
        );
        let mut p1 = p0 + Vec2::X * dims;
        let mut p2 = p0 + Vec2::Y * dims;
        let mut p3 = p0 + dims;

        if params.angle > EPS {
            p0 = p0.rotate_angle(params.angle);
            p1 = p1.rotate_angle(params.angle);
            p2 = p2.rotate_angle(params.angle);
            p3 = p3.rotate_angle(params.angle);
        }

        p0 += pos;
        p1 += pos;
        p2 += pos;
        p3 += pos;

        let atlas_entry = self.atlas.entry(sprite.atlas_key);

        self.append_vertices(
            &[
                Vertex {
                    pos: [p0.x, p0.y],
                    col: [params.tint.r, params.tint.g, params.tint.b],
                    uv: [atlas_entry.uv[0], atlas_entry.uv[1]],
                },
                Vertex {
                    pos: [p1.x, p1.y],
                    col: [params.tint.r, params.tint.g, params.tint.b],
                    uv: [atlas_entry.uv[2], atlas_entry.uv[1]],
                },
                Vertex {
                    pos: [p2.x, p2.y],
                    col: [params.tint.r, params.tint.g, params.tint.b],
                    uv: [atlas_entry.uv[0], atlas_entry.uv[3]],
                },
                Vertex {
                    pos: [p3.x, p3.y],
                    col: [params.tint.r, params.tint.g, params.tint.b],
                    uv: [atlas_entry.uv[2], atlas_entry.uv[3]],
                },
            ],
            &[0, 3, 2, 0, 1, 3],
        );
    }
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self::Left
    }
}

pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Default)]
pub struct Alignment {
    pub hor: HorizontalAlignment,
    pub ver: VerticalAlignment,
}

impl Alignment {
    pub const CENTER: Alignment = Alignment {
        hor: HorizontalAlignment::Center,
        ver: VerticalAlignment::Center,
    };
}

pub enum Size {
    Dimentions(Vec2),
    Scale(f32),
}

pub struct DrawSpriteParams {
    pub tint: Color,
    pub angle: f32,
    pub size: Size,
    pub alignment: Alignment,
}

impl Default for DrawSpriteParams {
    fn default() -> Self {
        Self {
            tint: Color::WHITE,
            angle: 0.0,
            size: Size::Scale(1.0),
            alignment: Alignment::default(),
        }
    }
}
