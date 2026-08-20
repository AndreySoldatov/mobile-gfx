use std::collections::HashMap;

use bdf_parser::Encoding;
use image::{Rgba, RgbaImage};
use slotmap::SlotMap;

use crate::atlas::AtlasKey;

pub(crate) struct Font {
    glyphs: HashMap<char, AtlasKey>,
}

fn rgba_image_from_glyph(glyph: &bdf_parser::Glyph) -> RgbaImage {
    let mut image = RgbaImage::new(
        glyph.bounding_box.size.x as u32,
        glyph.bounding_box.size.y as u32,
    );

    for y in 0..(glyph.bounding_box.size.y as usize) {
        for x in 0..(glyph.bounding_box.size.x as usize) {
            if glyph.pixel(x, y).unwrap() {
                image.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255]));
            } else {
                image.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
            }
        }
    }

    image
}

impl Font {
    pub(crate) fn load_from_bdf(
        atlas_staging: &mut SlotMap<AtlasKey, RgbaImage>,
        bdf: &str,
    ) -> anyhow::Result<Self> {
        let bdf = bdf_parser::Font::parse(bdf)?;
        let mut glyphs = HashMap::new();

        for glyph in bdf.glyphs.iter() {
            if let Encoding::Standard(c) = glyph.encoding {
                let c: char = c.try_into()?;
                glyphs.insert(c, atlas_staging.insert(rgba_image_from_glyph(glyph)));
            }
        }

        Ok(Self { glyphs })
    }

    pub(crate) fn get(&self, c: char) -> AtlasKey {
        if self.glyphs.contains_key(&c) {
            self.glyphs[&c]
        } else {
            self.glyphs[&' ']
        }
    }
}
