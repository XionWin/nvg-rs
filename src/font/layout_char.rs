use rusttype::PositionedGlyph;

use crate::math::Bounds;

use super::FontId;

#[derive(Debug)]
pub struct LayoutChar {
    pub(crate) id: FontId,
    pub x: f32,
    pub next_x: f32,
    pub c: char,
    pub idx: usize,
    pub(crate) glyph: PositionedGlyph<'static>,
    pub uv: Bounds,
    pub bounds: Bounds,
}