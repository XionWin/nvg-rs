use crate::AsPrimitive;

#[derive(Debug, Copy, Clone, Default)]
pub struct Extent {
    pub width: f32,
    pub height: f32,
}

impl Extent {
    pub fn new(width: f32, height: f32) -> Extent {
        Extent { width, height }
    }
}

impl<T: AsPrimitive<f32>> From<(T, T)> for Extent {
    fn from((width, height): (T, T)) -> Self {
        Extent::new(width.as_(), height.as_())
    }
}