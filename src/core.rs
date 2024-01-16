use once_cell::sync::Lazy;

use crate::graphics::Graphic;

pub fn init(graphic: &Graphic) {
    let program = gles_rs::GfxProgram::new("resources/shaders/nvgv2.vert", "resources/shaders/nvgv2.frag");
    program.active();

    gles_rs::viewport(0, 0, graphic.get_width(), graphic.get_height());
}

static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(_graphic: &Graphic) {
    let started_tick = STARTED_TICK.to_owned();
    let angle = (std::time::SystemTime::now()
        .duration_since(started_tick)
        .unwrap()
        .as_millis()
        / 20
        % 360) as u32;
    let hsv = color_rs::HSV::new(angle as f32, 1f32, 0.25f32);
    let rgb: color_rs::RGB = hsv.into();
    let (r, g, b) = rgb.into();
    gles_rs::clear_color(
        r as f32 / 255f32,
        g as f32 / 255f32,
        b as f32 / 255f32,
        1f32,
    );
    gles_rs::clear(gles_rs::GL_COLOR_BUFFER_BIT);
}