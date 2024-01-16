mod as_primitive;
#[allow(dead_code)]
pub(crate) mod math;
mod color;

pub use color::*;


pub fn test_func() {
    let p = math::Point::from((3, 2));
    println!("point: {:#?}", p);
    panic!("test end");
}