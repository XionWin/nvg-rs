#[derive(Debug, Copy, Clone)]
pub enum BasicCompositeOperation {
    SrcOver,
    SrcIn,
    SrcOut,
    Atop,
    DstOver,
    DstIn,
    DstOut,
    DstAtop,
    Lighter,
    Copy,
    Xor,
}