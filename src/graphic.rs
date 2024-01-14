#[derive(Debug)]
pub struct Graphic {
    width: libc::c_int,
    height: libc::c_int,
}

impl Graphic {
    pub fn new(width: libc::c_int, height: libc::c_int) -> Self {
        Self {
            width,
            height
         }
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }
}