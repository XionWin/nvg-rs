use once_cell::sync::Lazy;

pub fn init(context: &kms_rs::Context) {
    colored_rs::print_debug!("gl_extensions: {:?}", gles_rs::get_string(gles_rs::StringName::Extensions));
    colored_rs::print_debug!("gl_version: {:?}", gles_rs::get_string(gles_rs::StringName::Version));
    colored_rs::print_debug!("gl_sharding Language Version: {:?}", gles_rs::get_string(gles_rs::StringName::ShadingLanguageVersion));
    colored_rs::print_debug!("gl_vendor: {:?}", gles_rs::get_string(gles_rs::StringName::Vendor));
    colored_rs::print_debug!("gl_renderer: {:?}", gles_rs::get_string(gles_rs::StringName::Renderer));
    gles_rs::viewport(0, 0, context.get_width(), context.get_height());
}

static STARTED_TICK: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
pub fn update(_context: &kms_rs::Context) {
    let started_tick = STARTED_TICK.to_owned();
    let angle = (std::time::SystemTime::now().duration_since(started_tick).unwrap().as_millis() / 20 % 360) as u32;
    let hsv = color_rs::HSV::new(angle as f32, 1f32, 0.1f32);
    let rgb: color_rs::RGB = hsv.into();
    let (r, g, b) = rgb.into();
    gles_rs::clear_color(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, 1f32);
    gles_rs::clear(gles_rs::GL_COLOR_BUFFER_BIT);
}