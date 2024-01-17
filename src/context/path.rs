use crate::context::{Solidity, Vertex};

#[derive(Debug, Copy, Clone)]
pub struct Path {
    pub(crate) first: usize,
    pub(crate) count: usize,
    pub(crate) closed: bool,
    pub(crate) num_bevel: usize,
    pub(crate) solidity: Solidity,
    pub(crate) fill: *mut Vertex,
    pub(crate) num_fill: usize,
    pub(crate) stroke: *mut Vertex,
    pub(crate) num_stroke: usize,
    pub convex: bool,
}

impl Path {
    pub fn get_fill(&self) -> &[Vertex] {
        if self.fill.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.fill, self.num_fill) }
        }
    }

    pub fn get_stroke(&self) -> &[Vertex] {
        if self.stroke.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.stroke, self.num_stroke) }
        }
    }
}