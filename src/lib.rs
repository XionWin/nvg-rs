mod as_primitive;
#[allow(dead_code)]
mod math;

pub(crate) use math::*;


pub fn test_func() {
    let p = Point::from((3, 2));
    println!("point: {:#?}", p);
    panic!("test end");
}