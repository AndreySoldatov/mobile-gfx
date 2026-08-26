use glam::Vec2;

use crate::shapes::{Alignment, HorizontalAlignment, VerticalAlignment};

pub(crate) const EPS: f32 = 0.0001;
pub(crate) const PI: f32 = 3.141592;
pub(crate) const TAU: f32 = PI * 2.0;

pub(crate) fn factor(ps: Vec2, ls: Vec2) -> f32 {
    (ps / ls).floor().min_element().max(1.0)
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub tl: Vec2,
    pub wh: Vec2,
}

pub(crate) fn integer_fit(ps: Vec2, ls: Vec2) -> Rect {
    let scale = factor(ps, ls);
    let size = ls * scale;
    Rect {
        tl: (ps - size) * 0.5,
        wh: size,
    }
}

pub(crate) fn contains(rect: Rect, pos: Vec2) -> bool {
    pos.x > rect.tl.x
        && pos.x < rect.tl.x + rect.wh.x
        && pos.y > rect.tl.y
        && pos.y < rect.tl.y + rect.wh.y
}

pub(crate) fn rect_points_from(
    pos: Vec2,
    size: Vec2,
    alignment: Alignment,
    angle: f32,
) -> (Vec2, Vec2, Vec2, Vec2) {
    let mut p0 = Vec2::new(
        match alignment.hor {
            HorizontalAlignment::Left => 0.0,
            HorizontalAlignment::Center => -size.x * 0.5,
            HorizontalAlignment::Right => -size.x,
        },
        match alignment.ver {
            VerticalAlignment::Top => 0.0,
            VerticalAlignment::Center => -size.y * 0.5,
            VerticalAlignment::Bottom => -size.y,
        },
    );

    let mut p1 = p0 + Vec2::X * size;
    let mut p2 = p0 + Vec2::Y * size;
    let mut p3 = p0 + size;

    if angle > EPS {
        p0 = p0.rotate_angle(angle);
        p1 = p1.rotate_angle(angle);
        p2 = p2.rotate_angle(angle);
        p3 = p3.rotate_angle(angle);
    }

    (p0 + pos, p1 + pos, p2 + pos, p3 + pos)
}

pub(crate) fn circle_points_from(pos: Vec2, radius: f32) -> Vec<Vec2> {
    let count = radius as u32;
    let mut res = Vec::with_capacity(count as usize);
    for i in 0..count {
        let angle = (i as f32 / count as f32) * TAU;
        res.push(Vec2::new(angle.cos() * radius, angle.sin() * radius) + pos);
    }
    res
}

pub(crate) fn miter_vec(p0: Vec2, p1: Vec2, p2: Vec2, h: f32) -> (Vec2, Vec2) {
    let d0 = (p1 - p0).normalize_or_zero();
    let d1 = (p2 - p1).normalize_or_zero();
    let n0 = Vec2::new(-d0.y, d0.x);
    let n1 = Vec2::new(-d1.y, d1.x);

    let m = (n0 + n1).normalize();
    let l = h / m.dot(n0);

    (p1 + m * l, p1 - m * l)
}
