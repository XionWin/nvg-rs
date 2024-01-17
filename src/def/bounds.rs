use crate::Point;

#[derive(Debug, Copy, Clone, Default)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn left_top(&self) -> Point {
        self.min
    }

    pub fn right_top(&self) -> Point {
        Point::new(self.max.x, self.min.y)
    }

    pub fn left_bottom(&self) -> Point {
        Point::new(self.min.x, self.max.y)
    }

    pub fn right_bottom(&self) -> Point {
        self.max
    }
}
