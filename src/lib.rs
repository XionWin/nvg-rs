#[macro_use]
extern crate bitflags;

#[allow(dead_code)]
mod as_primitive;
#[allow(dead_code)]
pub mod math;
#[allow(dead_code)]
pub mod renderer;
#[allow(dead_code)]
pub mod font;
#[allow(dead_code)]
pub mod cache;
#[allow(dead_code)]
pub mod context;
#[allow(dead_code)]
pub mod color;

pub(crate) use as_primitive::*;

pub fn test_func() {

}