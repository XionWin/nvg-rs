use crate::{
    math::{Extent, Transform}, 
    color::Color, 
    renderer::Scissor,
    font::FontId,
    context::{Paint, LineJoin, LineCap, Align, CompositeOperationState, CompositeOperation, BasicCompositeOperation}
};

#[derive(Clone)]
pub struct State {
    pub(crate) composite_operation: CompositeOperationState,
    pub(crate) shape_antialias: bool,
    pub(crate) fill: Paint,
    pub(crate) stroke: Paint,
    pub(crate) stroke_width: f32,
    pub(crate) miter_limit: f32,
    pub(crate) line_join: LineJoin,
    pub(crate) line_cap: LineCap,
    pub(crate) alpha: f32,
    pub(crate) xform: Transform,
    pub(crate) scissor: Scissor,
    pub(crate) font_size: f32,
    pub(crate) letter_spacing: f32,
    pub(crate) line_height: f32,
    pub(crate) text_align: Align,
    pub(crate) font_id: FontId,
}

impl Default for State {
    fn default() -> Self {
        State {
            composite_operation: CompositeOperation::Basic(BasicCompositeOperation::SrcOver).into(),
            shape_antialias: true,
            fill: Color::rgb(1.0, 1.0, 1.0).into(),
            stroke: Color::rgb(0.0, 0.0, 0.0).into(),
            stroke_width: 1.0,
            miter_limit: 10.0,
            line_join: LineJoin::Miter,
            line_cap: LineCap::Butt,
            alpha: 1.0,
            xform: Transform::identity(),
            scissor: Scissor {
                xform: Default::default(),
                extent: Extent {
                    width: -1.0,
                    height: -1.0,
                },
            },
            font_size: 16.0,
            letter_spacing: 0.0,
            line_height: 1.0,
            text_align: Align::LEFT | Align::BASELINE,
            font_id: 0,
        }
    }
}