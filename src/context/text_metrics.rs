#[derive(Copy, Clone)]
pub struct TextMetrics {
    pub ascender: f32,
    pub descender: f32,
    pub line_gap: f32,
}

impl TextMetrics {
    pub fn line_height(&self) -> f32 {
        self.ascender - self.descender + self.line_gap
    }
}