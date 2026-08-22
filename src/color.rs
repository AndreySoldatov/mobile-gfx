#[derive(Clone, Copy)]
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

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const DARK_GRAY: Color = Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
    };

    pub const BRIGHT_GRAY: Color = Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
    };

    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    pub const RED: Color = Color {
        r: 1.0,
        g: 0.188,
        b: 0.188,
    };

    pub const GREEN: Color = Color {
        r: 0.15,
        g: 1.0,
        b: 0.2,
    };

    pub const BLUE: Color = Color {
        r: 0.15,
        g: 0.3,
        b: 1.0,
    };

    pub const LIGHT_BLUE: Color = Color {
        r: 0.25,
        g: 0.6,
        b: 1.0,
    };

    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 0.9,
        b: 0.15,
    };

    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.45,
        b: 0.05,
    };

    pub const PURPLE: Color = Color {
        r: 0.5,
        g: 0.2,
        b: 1.0,
    };
}

pub fn srgb_to_linear_channel(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

pub fn srgb_to_linear(c: Color) -> Color {
    Color {
        r: srgb_to_linear_channel(c.r),
        g: srgb_to_linear_channel(c.g),
        b: srgb_to_linear_channel(c.b),
    }
}
