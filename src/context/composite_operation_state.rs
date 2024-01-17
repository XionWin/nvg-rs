use crate::context::BlendFactor;

#[derive(Debug, Copy, Clone)]
pub struct CompositeOperationState {
    pub src_rgb: BlendFactor,
    pub dst_rgb: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
}