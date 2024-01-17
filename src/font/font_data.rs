use rusttype::Font;

use super::FontId;

pub struct FontData {
    pub(crate) font: Font<'static>,
    pub(crate) fallback_fonts: Vec<FontId>,
}