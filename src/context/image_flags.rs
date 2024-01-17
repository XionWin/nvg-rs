bitflags! {
    pub struct ImageFlags: u32 {
        const GENERATE_MIPMAPS = 0x1;
        const REPEATX = 0x2;
        const REPEATY = 0x4;
        const FLIPY	= 0x8;
        const PREMULTIPLIED = 0x10;
        const NEAREST = 0x20;
    }
}