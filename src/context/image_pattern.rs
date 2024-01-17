use crate::{math::{Point, Extent}, context::ImageId};

#[derive(Debug, Copy, Clone)]
pub struct ImagePattern {
    pub center: Point,
    pub size: Extent,
    pub angle: f32,
    pub img: ImageId,
    pub alpha: f32,
}