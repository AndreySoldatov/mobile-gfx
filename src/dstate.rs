use std::f32::consts::PI;

use glam::Vec2;

pub struct DynamicState {
    pub p0: glam::Vec2,
    pub p1: glam::Vec2,
    pub p2: glam::Vec2,

    d0: glam::Vec2,
    d1: glam::Vec2,
    d2: glam::Vec2,
}

impl DynamicState {
    pub fn new() -> Self {
        Self {
            p0: Vec2::ONE,
            p1: Vec2::ONE,
            p2: Vec2::ONE,
            d0: Vec2::from_angle(rand::random::<f32>() * PI * 2.0),
            d1: Vec2::from_angle(rand::random::<f32>() * PI * 2.0),
            d2: Vec2::from_angle(rand::random::<f32>() * PI * 2.0),
        }
    }

    pub fn update_point(&mut self, size: Vec2) {
        if self.p0.x < 0.0 || self.p0.x > size.x {
            self.d0.x = -self.d0.x;
        }
        if self.p0.y < 0.0 || self.p0.y > size.y {
            self.d0.y = -self.d0.y;
        }
        self.p0 += self.d0;

        if self.p1.x < 0.0 || self.p1.x > size.x {
            self.d1.x = -self.d1.x;
        }
        if self.p1.y < 0.0 || self.p1.y > size.y {
            self.d1.y = -self.d1.y;
        }
        self.p1 += self.d1;

        if self.p2.x < 0.0 || self.p2.x > size.x {
            self.d2.x = -self.d2.x;
        }
        if self.p2.y < 0.0 || self.p2.y > size.y {
            self.d2.y = -self.d2.y;
        }
        self.p2 += self.d2;
    }
}
