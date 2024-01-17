use crate::math::{Transform, Extent};

#[derive(Debug, Copy, Clone)]
pub struct Scissor {
    pub xform: Transform,
    pub extent: Extent,
}