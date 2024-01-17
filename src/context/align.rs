bitflags! {
    pub struct Align: u32 {
        const LEFT = 0x1;
        const CENTER = 0x2;
        const RIGHT = 0x4;
        const TOP = 0x8;
        const MIDDLE = 0x10;
        const BOTTOM = 0x20;
        const BASELINE = 0x40;
    }
}