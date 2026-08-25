use glam::Vec2;

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
