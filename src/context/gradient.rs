use crate::{
    color::Color,
    math::{Point, Rect},
};

#[derive(Debug, Copy, Clone)]
pub enum Gradient {
    Linear {
        start: Point,
        end: Point,
        start_color: Color,
        end_color: Color,
    },
    Radial {
        center: Point,
        in_radius: f32,
        out_radius: f32,
        inner_color: Color,
        outer_color: Color,
    },
    Box {
        rect: Rect,
        radius: f32,
        feather: f32,
        inner_color: Color,
        outer_color: Color,
    },
}
