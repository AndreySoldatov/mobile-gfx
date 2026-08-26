use crate::{
    SpriteKey,
    color::Color,
    render::{RenderState, Vertex},
    utils::{circle_points_from, miter_vec, rect_points_from},
};
use glam::Vec2;

impl RenderState {
    pub fn white_pixel(&self) -> Vec2 {
        self.atlas.entry(self.white_pixel).uv.0
    }

    pub fn draw_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, fill: Color) {
        self.draw_triangle_ex(
            p0,
            p1,
            p2,
            DrawShapeParams {
                fill: Some(fill),
                stroke: None,
            },
        );
    }

    pub fn draw_triangle_lines(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, stroke: Stroke) {
        self.draw_triangle_ex(
            p0,
            p1,
            p2,
            DrawShapeParams {
                fill: None,
                stroke: Some(stroke),
            },
        );
    }

    pub fn draw_triangle_ex(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, params: DrawShapeParams) {
        if let Some(fill) = params.fill {
            let wp = self.white_pixel();
            self.append_vertices(
                &[
                    Vertex::new(p0, fill, wp),
                    Vertex::new(p1, fill, wp),
                    Vertex::new(p2, fill, wp),
                ],
                &[0, 1, 2],
            );
        }

        if let Some(stroke) = params.stroke {
            self.draw_line_segment(&[p0, p1, p2], true, stroke);
        }
    }

    pub fn draw_quad(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, c: Color) {
        let wp = self.white_pixel();
        self.append_vertices(
            &[
                Vertex::new(p0, c, wp),
                Vertex::new(p1, c, wp),
                Vertex::new(p2, c, wp),
                Vertex::new(p3, c, wp),
            ],
            &[0, 1, 2, 2, 3, 0],
        );
    }

    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, stroke: Stroke) {
        let n = Vec2::new(-(p1.y - p0.y), p1.x - p0.x).normalize_or_zero() * stroke.thickness * 0.5;
        let pq0 = p0 - n;
        let pq1 = p0 + n;
        let pq2 = p1 - n;
        let pq3 = p1 + n;
        self.draw_quad(pq0, pq2, pq3, pq1, stroke.color);
    }

    pub fn draw_line_segment(&mut self, points: &[Vec2], closed: bool, stroke: Stroke) {
        if points.len() < 2 {
            return;
        }

        let offset = self.get_index_offset();
        let h = stroke.thickness * 0.5;
        let wp = self.white_pixel();

        self.vertices.reserve(points.len() * 2);

        // First point
        if !closed {
            let n = Vec2::new(-(points[1].y - points[0].y), points[1].x - points[0].x)
                .normalize_or_zero()
                * h;
            self.vertices.extend_from_slice(&[
                Vertex::new(points[0] + n, stroke.color, wp),
                Vertex::new(points[0] - n, stroke.color, wp),
            ]);
        } else {
            let (mp0, mp1) = miter_vec(points[points.len() - 1], points[0], points[1], h);
            self.vertices.extend_from_slice(&[
                Vertex::new(mp0, stroke.color, wp),
                Vertex::new(mp1, stroke.color, wp),
            ]);
        }

        // Miter points
        for i in 1..(points.len() - 1) {
            let (mp0, mp1) = miter_vec(points[i - 1], points[i], points[i + 1], h);
            self.vertices.extend_from_slice(&[
                Vertex::new(mp0, stroke.color, wp),
                Vertex::new(mp1, stroke.color, wp),
            ]);
        }

        // Last point
        if !closed {
            let last = points.len() - 1;
            let n = Vec2::new(
                -(points[last - 1].y - points[last].y),
                points[last - 1].x - points[last].x,
            )
            .normalize_or_zero()
                * h;
            self.vertices.extend_from_slice(&[
                Vertex::new(points[last] - n, stroke.color, wp),
                Vertex::new(points[last] + n, stroke.color, wp),
            ]);
        } else {
            let (mp0, mp1) = miter_vec(
                points[points.len() - 2],
                points[points.len() - 1],
                points[0],
                h,
            );
            self.vertices.extend_from_slice(&[
                Vertex::new(mp0, stroke.color, wp),
                Vertex::new(mp1, stroke.color, wp),
            ]);
        }

        self.indices.reserve(
            if !closed {
                points.len() - 1
            } else {
                points.len()
            } * 6,
        );
        for i in 0..(points.len() as u32 - 1) {
            self.indices.extend_from_slice(&[
                i * 2 + offset,
                i * 2 + 2 + offset,
                i * 2 + 1 + offset,
                i * 2 + 1 + offset,
                i * 2 + 2 + offset,
                i * 2 + 3 + offset,
            ]);
        }
        if closed {
            self.indices.extend_from_slice(&[
                (points.len() as u32 - 1) * 2 + offset,
                offset,
                (points.len() as u32 - 1) * 2 + 1 + offset,
                (points.len() as u32 - 1) * 2 + 1 + offset,
                offset,
                offset + 1,
            ]);
        }
    }

    pub fn draw_rect_ex(&mut self, pos: Vec2, params: DrawRectParams) {
        let (p0, p1, p3, p2) = rect_points_from(pos, params.size, params.alignment, params.angle);

        if let Some(fill) = params.shape_params.fill {
            self.draw_quad(p0, p1, p2, p3, fill);
        }

        if let Some(stroke) = params.shape_params.stroke {
            let dir = (p1 - p0).normalize_or_zero();
            self.draw_line(
                p0 - dir * stroke.thickness * 0.5,
                p1 + dir * stroke.thickness * 0.5,
                stroke,
            );
            self.draw_line(p1, p2, stroke);
            self.draw_line(
                p2 + dir * stroke.thickness * 0.5,
                p3 - dir * stroke.thickness * 0.5,
                stroke,
            );
            self.draw_line(p3, p0, stroke);
        }
    }

    pub fn draw_rect(&mut self, pos: Vec2, size: Vec2, fill: Color) {
        self.draw_rect_ex(
            pos,
            DrawRectParams {
                shape_params: DrawShapeParams {
                    fill: Some(fill),
                    stroke: None,
                },
                size,
                ..Default::default()
            },
        );
    }

    pub fn draw_rect_lines(&mut self, pos: Vec2, size: Vec2, stroke: Stroke) {
        self.draw_rect_ex(
            pos,
            DrawRectParams {
                shape_params: DrawShapeParams {
                    fill: None,
                    stroke: Some(stroke),
                },
                size,
                ..Default::default()
            },
        );
    }

    pub fn draw_circle_ex(&mut self, pos: Vec2, radius: f32, params: DrawShapeParams) {
        let offset = self.get_index_offset();
        let cp = circle_points_from(pos, radius);

        if let Some(fill) = params.fill {
            // Points
            self.vertices.reserve(cp.len() + 1);
            for point in &cp {
                self.vertices
                    .push(Vertex::new(*point, fill, self.white_pixel()));
            }
            self.vertices
                .push(Vertex::new(pos, fill, self.white_pixel()));

            // Indices
            self.indices.reserve(cp.len() * 3);
            for i in 0..(cp.len() - 1) {
                let i = i as u32 + offset;
                self.indices
                    .extend_from_slice(&[i, cp.len() as u32 + offset, i + 1]);
            }
            self.indices.extend_from_slice(&[
                (cp.len() - 1) as u32 + offset,
                cp.len() as u32 + offset,
                offset,
            ]);
        }

        if let Some(stroke) = params.stroke {
            self.draw_line_segment(&cp, true, stroke);
        }
    }

    pub fn draw_circle(&mut self, pos: Vec2, radius: f32, fill: Color) {
        self.draw_circle_ex(
            pos,
            radius,
            DrawShapeParams {
                fill: Some(fill),
                stroke: None,
            },
        );
    }

    pub fn draw_circle_lines(&mut self, pos: Vec2, radius: f32, stroke: Stroke) {
        self.draw_circle_ex(
            pos,
            radius,
            DrawShapeParams {
                fill: None,
                stroke: Some(stroke),
            },
        );
    }

    pub fn draw_sprite_ex(&mut self, pos: Vec2, sprite: SpriteKey, params: DrawSpriteParams) {
        let dims = match params.size {
            Size::Dimentions(size) => size,
            Size::Scale(scale) => Vec2::new(
                sprite.width() as f32 * scale,
                sprite.height() as f32 * scale,
            ),
        };
        let (p0, p1, p2, p3) = rect_points_from(pos, dims, params.alignment, params.angle);

        let atlas_entry = self.atlas.entry(sprite.atlas_key);
        let uvx = Vec2::new(atlas_entry.uv.1.x, atlas_entry.uv.0.y);
        let uvy = Vec2::new(atlas_entry.uv.0.x, atlas_entry.uv.1.y);

        self.append_vertices(
            &[
                Vertex::new(p0, params.tint, atlas_entry.uv.0),
                Vertex::new(p1, params.tint, uvx),
                Vertex::new(p2, params.tint, uvy),
                Vertex::new(p3, params.tint, atlas_entry.uv.1),
            ],
            &[0, 3, 2, 0, 1, 3],
        );
    }

    pub fn draw_sprite(&mut self, pos: Vec2, sprite: SpriteKey) {
        self.draw_sprite_ex(pos, sprite, Default::default());
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Size {
    Dimentions(Vec2),
    Scale(f32),
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Stroke {
    pub color: Color,
    pub thickness: f32,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            thickness: 1.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DrawShapeParams {
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
}

impl Default for DrawShapeParams {
    fn default() -> Self {
        Self {
            fill: Some(Color::WHITE),
            stroke: Some(Stroke::default()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DrawRectParams {
    pub shape_params: DrawShapeParams,
    pub size: Vec2,
    pub angle: f32,
    pub alignment: Alignment,
}

impl Default for DrawRectParams {
    fn default() -> Self {
        Self {
            shape_params: DrawShapeParams::default(),
            size: Vec2::ZERO,
            angle: 0.0,
            alignment: Alignment::default(),
        }
    }
}
