use std::f32::consts::PI;

use crate::{
    math::{Point, Extent, Transform, Rect},
    font::{FontId, Fonts, LayoutChar},
    renderer::{Renderer, TextureType},
    context::{Paint, LineJoin, LineCap, ImageId, ImageFlags, Align, Command, CompositeOperation}, 
    cache::PathCache
};

use super::{State, Solidity, Vertex, TextMetrics};

const KAPPA90: f32 = 0.5522847493;

pub struct Context<R: Renderer> {
    renderer: R,
    commands: Vec<Command>,
    last_position: Point,
    states: Vec<State>,
    cache: PathCache,
    tess_tol: f32,
    dist_tol: f32,
    fringe_width: f32,
    device_pixel_ratio: f32,
    fonts: Fonts,
    layout_chars: Vec<LayoutChar>,
    draw_call_count: usize,
    fill_triangles_count: usize,
    stroke_triangles_count: usize,
    text_triangles_count: usize,
}

impl<R: Renderer> Context<R> {
    pub fn create(mut renderer: R) -> anyhow::Result<Context<R>> {
        let fonts = Fonts::new(&mut renderer)?;
        Ok(Context {
            renderer,
            commands: Default::default(),
            last_position: Default::default(),
            states: vec![Default::default()],
            cache: Default::default(),
            tess_tol: 0.0,
            dist_tol: 0.0,
            fringe_width: 0.0,
            device_pixel_ratio: 0.0,
            fonts,
            layout_chars: Default::default(),
            draw_call_count: 0,
            fill_triangles_count: 0,
            stroke_triangles_count: 0,
            text_triangles_count: 0,
        })
    }

    fn set_device_pixel_ratio(&mut self, ratio: f32) {
        self.tess_tol = 0.25 / ratio;
        self.dist_tol = 0.01 / ratio;
        self.fringe_width = 1.0 / ratio;
        self.device_pixel_ratio = ratio;
    }

    pub fn begin_frame<E: Into<Extent>>(
        &mut self,
        window_extent: E,
        device_pixel_ratio: f32,
    ) -> anyhow::Result<()> {
        self.states.clear();
        self.states.push(Default::default());
        self.set_device_pixel_ratio(device_pixel_ratio);
        self.renderer
            .viewport(window_extent.into(), device_pixel_ratio)?;
        self.draw_call_count = 0;
        self.fill_triangles_count = 0;
        self.stroke_triangles_count = 0;
        self.text_triangles_count = 0;
        Ok(())
    }

    pub fn cancel_frame(&mut self) -> anyhow::Result<()> {
        self.renderer.cancel()?;
        Ok(())
    }

    pub fn end_frame(&mut self) -> anyhow::Result<()> {
        self.renderer.flush()?;
        Ok(())
    }

    pub fn save(&mut self) {
        if let Some(last) = self.states.last() {
            let last = last.clone();
            self.states.push(last);
        }
    }

    pub fn restore(&mut self) {
        if self.states.len() <= 1 {
            return;
        }
        self.states.pop();
    }

    fn state(&mut self) -> &State {
        self.states.last().unwrap()
    }

    fn state_mut(&mut self) -> &mut State {
        self.states.last_mut().unwrap()
    }

    pub fn reset(&mut self) {
        *self.state_mut() = Default::default();
    }

    pub fn shape_antialias(&mut self, enabled: bool) {
        self.state_mut().shape_antialias = enabled;
    }

    pub fn stroke_width(&mut self, width: f32) {
        self.state_mut().stroke_width = width * self.device_pixel_ratio;
    }

    pub fn miter_limit(&mut self, limit: f32) {
        self.state_mut().miter_limit = limit;
    }

    pub fn line_cap(&mut self, cap: LineCap) {
        self.state_mut().line_cap = cap;
    }

    pub fn line_join(&mut self, join: LineJoin) {
        self.state_mut().line_join = join;
    }

    pub fn global_alpha(&mut self, alpha: f32) {
        self.state_mut().alpha = alpha;
    }

    pub fn transform(&mut self, xform: Transform) {
        let state = self.state_mut();
        state.xform = xform * state.xform;
    }

    pub fn reset_transform(&mut self) {
        self.state_mut().xform = Transform::identity();
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.transform(Transform::translate(tx, ty));
    }

    pub fn rotate(&mut self, angle: f32) {
        self.transform(Transform::rotate(angle));
    }

    pub fn skew_x(&mut self, angle: f32) {
        self.transform(Transform::skew_x(angle));
    }

    pub fn skew_y(&mut self, angle: f32) {
        self.transform(Transform::skew_y(angle));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.transform(Transform::scale(sx, sy));
    }

    pub fn current_transform(&mut self) -> Transform {
        self.state().xform
    }

    pub fn stroke_paint<T: Into<Paint>>(&mut self, paint: T) {
        let mut paint = paint.into();
        paint.xform *= self.state().xform;
        self.state_mut().stroke = paint;
    }

    pub fn fill_paint<T: Into<Paint>>(&mut self, paint: T) {
        let mut paint = paint.into();
        paint.xform *= self.state().xform;
        self.state_mut().fill = paint;
    }

    pub fn create_image<D: AsRef<[u8]>>(
        &mut self,
        flags: ImageFlags,
        data: D,
    ) -> anyhow::Result<ImageId> {
        let img = image::load_from_memory(data.as_ref())?;
        let img = img.to_rgba();
        let dimensions = img.dimensions();
        let img = self.renderer.create_texture(
            TextureType::RGBA,
            dimensions.0 as usize,
            dimensions.1 as usize,
            flags,
            Some(&img.into_raw()),
        )?;
        Ok(img)
    }

    pub fn create_image_from_file<P: AsRef<std::path::Path>>(
        &mut self,
        flags: ImageFlags,
        path: P,
    ) -> anyhow::Result<ImageId> {
        self.create_image(flags, std::fs::read(path)?)
    }

    pub fn update_image(&mut self, img: ImageId, data: &[u8]) -> anyhow::Result<()> {
        let (w, h) = self.renderer.texture_size(img.clone())?;
        self.renderer.update_texture(img, 0, 0, w, h, data)?;
        Ok(())
    }

    pub fn image_size(&self, img: ImageId) -> anyhow::Result<(usize, usize)> {
        let res = self.renderer.texture_size(img)?;
        Ok(res)
    }

    pub fn delete_image(&mut self, img: ImageId) -> anyhow::Result<()> {
        self.renderer.delete_texture(img)?;
        Ok(())
    }

    pub fn scissor<T: Into<Rect>>(&mut self, rect: T) {
        let rect = rect.into();
        let state = self.state_mut();
        let x = rect.xy.x;
        let y = rect.xy.y;
        let width = rect.size.width.max(0.0);
        let height = rect.size.height.max(0.0);
        state.scissor.xform = Transform::identity();
        state.scissor.xform.0[4] = x + width * 0.5;
        state.scissor.xform.0[5] = y + height * 0.5;
        state.scissor.xform *= state.xform;
        state.scissor.extent.width = width * 0.5;
        state.scissor.extent.height = height * 0.5;
    }

    pub fn intersect_scissor<T: Into<Rect>>(&mut self, rect: T) {
        let rect = rect.into();
        let state = self.state_mut();

        if state.scissor.extent.width < 0.0 {
            self.scissor(rect);
            return;
        }

        let Extent {
            width: ex,
            height: ey,
        } = state.scissor.extent;
        let invxorm = state.xform.inverse();
        let pxform = state.scissor.xform * invxorm;
        let tex = ex * pxform.0[0].abs() + ey * pxform.0[2].abs();
        let tey = ex * pxform.0[1].abs() + ey * pxform.0[3].abs();
        self.scissor(
            Rect::new(
                Point::new(pxform.0[4] - tex, pxform.0[5] - tey),
                Extent::new(tex * 2.0, tey * 2.0),
            )
            .intersect(rect),
        );
    }

    pub fn reset_scissor(&mut self) {
        let state = self.state_mut();
        state.scissor.xform = Transform::default();
        state.scissor.extent.width = -1.0;
        state.scissor.extent.height = -1.0;
    }

    pub fn global_composite_operation(&mut self, op: CompositeOperation) {
        self.state_mut().composite_operation = op.into();
    }

    fn append_command(&mut self, cmd: Command) {
        let state = self.states.last().unwrap();
        let xform = &state.xform;
        match cmd {
            Command::MoveTo(pt) => {
                self.commands
                    .push(Command::MoveTo(xform.transform_point(pt)));
                self.last_position = pt;
            }
            Command::LineTo(pt) => {
                self.commands
                    .push(Command::LineTo(xform.transform_point(pt)));
                self.last_position = pt;
            }
            Command::BezierTo(pt1, pt2, pt3) => {
                self.last_position = pt3;
                self.commands.push(Command::BezierTo(
                    xform.transform_point(pt1),
                    xform.transform_point(pt2),
                    xform.transform_point(pt3),
                ));
            }
            _ => {
                self.commands.push(cmd);
            }
        }
    }

    pub fn begin_path(&mut self) {
        self.commands.clear();
        self.cache.clear();
    }

    pub fn move_to<P: Into<Point>>(&mut self, pt: P) {
        self.append_command(Command::MoveTo(pt.into()));
    }

    pub fn line_to<P: Into<Point>>(&mut self, pt: P) {
        self.append_command(Command::LineTo(pt.into()));
    }

    pub fn bezier_to<P: Into<Point>>(&mut self, cp1: P, cp2: P, pt: P) {
        self.append_command(Command::BezierTo(cp1.into(), cp2.into(), pt.into()));
    }

    pub fn quad_to<P: Into<Point>>(&mut self, cp: P, pt: P) {
        let x0 = self.last_position.x;
        let y0 = self.last_position.y;
        let cp = cp.into();
        let pt = pt.into();
        self.append_command(Command::BezierTo(
            Point::new(x0 + 2.0 / 3.0 * (cp.x - x0), y0 + 2.0 / 3.0 * (cp.y - y0)),
            Point::new(
                pt.x + 2.0 / 3.0 * (cp.x - pt.x),
                pt.y + 2.0 / 3.0 * (cp.y - pt.y),
            ),
            pt,
        ));
    }

    pub fn arc_to<P: Into<Point>>(&mut self, pt1: P, pt2: P, radius: f32) {
        let pt0 = self.last_position;

        if self.commands.is_empty() {
            return;
        }

        let pt1 = pt1.into();
        let pt2 = pt2.into();
        if pt0.equals(pt1, self.dist_tol)
            || pt1.equals(pt2, self.dist_tol)
            || pt1.dist_pt_seg(pt0, pt2) < self.dist_tol * self.dist_tol
            || radius < self.dist_tol
        {
            self.line_to(pt1);
            return;
        }

        let d0 = Point::new(pt0.x - pt1.x, pt0.y - pt1.y);
        let d1 = Point::new(pt2.x - pt1.x, pt2.y - pt1.y);
        let a = (d0.x * d1.x + d0.y * d1.y).cos();
        let d = radius / (a / 2.0).tan();

        if d > 10000.0 {
            self.line_to(pt1);
            return;
        }

        let (cx, cy, a0, a1, dir) = if Point::cross(d0, d1) > 0.0 {
            (
                pt1.x + d0.x * d + d0.y * radius,
                pt1.y + d0.y * d + -d0.x * radius,
                d0.x.atan2(-d0.y),
                -d1.x.atan2(d1.y),
                Solidity::Hole,
            )
        } else {
            (
                pt1.x + d0.x * d + -d0.y * radius,
                pt1.y + d0.y * d + d0.x * radius,
                -d0.x.atan2(d0.y),
                d1.x.atan2(-d1.y),
                Solidity::Solid,
            )
        };

        self.arc(Point::new(cx, cy), radius, a0, a1, dir);
    }

    pub fn close_path(&mut self) {
        self.commands.push(Command::Close);
    }

    pub fn path_solidity(&mut self, dir: Solidity) {
        self.commands.push(Command::Solidity(dir));
    }

    pub fn arc<P: Into<Point>>(&mut self, cp: P, radius: f32, a0: f32, a1: f32, dir: Solidity) {
        let cp = cp.into();
        let move_ = self.commands.is_empty();

        let mut da = a1 - a0;
        if dir == Solidity::Hole {
            if da.abs() >= PI * 2.0 {
                da = PI * 2.0;
            } else {
                while da < 0.0 {
                    da += PI * 2.0;
                }
            }
        } else {
            if da.abs() >= PI * 2.0 {
                da = -PI * 2.0;
            } else {
                while da > 0.0 {
                    da -= PI * 2.0;
                }
            }
        }

        let ndivs = ((da.abs() / (PI * 0.5) + 0.5) as i32).min(5).max(1);
        let hda = (da / (ndivs as f32)) / 2.0;
        let mut kappa = (4.0 / 3.0 * (1.0 - hda.cos()) / hda.sin()).abs();

        if dir == Solidity::Solid {
            kappa = -kappa;
        }

        let mut px = 0.0;
        let mut py = 0.0;
        let mut ptanx = 0.0;
        let mut ptany = 0.0;

        for i in 0..=ndivs {
            let a = a0 + da * ((i as f32) / (ndivs as f32));
            let dx = a.cos();
            let dy = a.sin();
            let x = cp.x + dx * radius;
            let y = cp.y + dy * radius;
            let tanx = -dy * radius * kappa;
            let tany = dx * radius * kappa;

            if i == 0 {
                if move_ {
                    self.append_command(Command::MoveTo(Point::new(x, y)));
                } else {
                    self.append_command(Command::LineTo(Point::new(x, y)));
                }
            } else {
                self.append_command(Command::BezierTo(
                    Point::new(px + ptanx, py + ptany),
                    Point::new(x - tanx, y - tany),
                    Point::new(x, y),
                ));
            }
            px = x;
            py = y;
            ptanx = tanx;
            ptany = tany;
        }
    }

    pub fn rect<T: Into<Rect>>(&mut self, rect: T) {
        let rect = rect.into();
        self.append_command(Command::MoveTo(Point::new(rect.xy.x, rect.xy.y)));
        self.append_command(Command::LineTo(Point::new(
            rect.xy.x,
            rect.xy.y + rect.size.height,
        )));
        self.append_command(Command::LineTo(Point::new(
            rect.xy.x + rect.size.width,
            rect.xy.y + rect.size.height,
        )));
        self.append_command(Command::LineTo(Point::new(
            rect.xy.x + rect.size.width,
            rect.xy.y,
        )));
        self.append_command(Command::Close);
    }

    pub fn rounded_rect<T: Into<Rect>>(&mut self, rect: T, radius: f32) {
        let rect = rect.into();
        self.rounded_rect_varying(rect, radius, radius, radius, radius);
    }

    pub fn rounded_rect_varying<T: Into<Rect>>(
        &mut self,
        rect: T,
        lt: f32,
        rt: f32,
        rb: f32,
        lb: f32,
    ) {
        let rect = rect.into();
        if lt < 0.1 && rt < 0.1 && lb < 0.1 && rb < 0.1 {
            self.rect(rect);
        } else {
            let halfw = rect.size.width.abs() * 0.5;
            let halfh = rect.size.height.abs() * 0.5;
            let rxlb = lb.min(halfw) * rect.size.width.signum();
            let rylb = lb.min(halfh) * rect.size.height.signum();
            let rxrb = rb.min(halfw) * rect.size.width.signum();
            let ryrb = rb.min(halfh) * rect.size.height.signum();
            let rxrt = rt.min(halfw) * rect.size.width.signum();
            let ryrt = rt.min(halfh) * rect.size.height.signum();
            let rxlt = lt.min(halfw) * rect.size.width.signum();
            let rylt = lt.min(halfh) * rect.size.height.signum();

            self.append_command(Command::MoveTo(Point::new(rect.xy.x, rect.xy.y + rylt)));
            self.append_command(Command::LineTo(Point::new(
                rect.xy.x,
                rect.xy.y + rect.size.height - rylb,
            )));
            self.append_command(Command::BezierTo(
                Point::new(
                    rect.xy.x,
                    rect.xy.y + rect.size.height - rylb * (1.0 - KAPPA90),
                ),
                Point::new(
                    rect.xy.x + rxlb * (1.0 - KAPPA90),
                    rect.xy.y + rect.size.height,
                ),
                Point::new(rect.xy.x + rxlb, rect.xy.y + rect.size.height),
            ));
            self.append_command(Command::LineTo(Point::new(
                rect.xy.x + rect.size.width - rxrb,
                rect.xy.y + rect.size.height,
            )));
            self.append_command(Command::BezierTo(
                Point::new(
                    rect.xy.x + rect.size.width - rxrb * (1.0 - KAPPA90),
                    rect.xy.y + rect.size.height,
                ),
                Point::new(
                    rect.xy.x + rect.size.width,
                    rect.xy.y + rect.size.height - ryrb * (1.0 - KAPPA90),
                ),
                Point::new(
                    rect.xy.x + rect.size.width,
                    rect.xy.y + rect.size.height - ryrb,
                ),
            ));
            self.append_command(Command::LineTo(Point::new(
                rect.xy.x + rect.size.width,
                rect.xy.y + ryrt,
            )));
            self.append_command(Command::BezierTo(
                Point::new(
                    rect.xy.x + rect.size.width,
                    rect.xy.y + ryrt * (1.0 - KAPPA90),
                ),
                Point::new(
                    rect.xy.x + rect.size.width - rxrt * (1.0 - KAPPA90),
                    rect.xy.y,
                ),
                Point::new(rect.xy.x + rect.size.width - rxrt, rect.xy.y),
            ));
            self.append_command(Command::LineTo(Point::new(rect.xy.x + rxlt, rect.xy.y)));
            self.append_command(Command::BezierTo(
                Point::new(rect.xy.x + rxlt * (1.0 - KAPPA90), rect.xy.y),
                Point::new(rect.xy.x, rect.xy.y + rylt * (1.0 - KAPPA90)),
                Point::new(rect.xy.x, rect.xy.y + rylt),
            ));
            self.append_command(Command::Close);
        }
    }

    pub fn ellipse<P: Into<Point>>(&mut self, center: P, radius_x: f32, radius_y: f32) {
        let center = center.into();
        self.append_command(Command::MoveTo(Point::new(center.x - radius_x, center.y)));
        self.append_command(Command::BezierTo(
            Point::new(center.x - radius_x, center.y + radius_y * KAPPA90),
            Point::new(center.x - radius_x * KAPPA90, center.y + radius_y),
            Point::new(center.x, center.y + radius_y),
        ));
        self.append_command(Command::BezierTo(
            Point::new(center.x + radius_x * KAPPA90, center.y + radius_y),
            Point::new(center.x + radius_x, center.y + radius_y * KAPPA90),
            Point::new(center.x + radius_x, center.y),
        ));
        self.append_command(Command::BezierTo(
            Point::new(center.x + radius_x, center.y - radius_y * KAPPA90),
            Point::new(center.x + radius_x * KAPPA90, center.y - radius_y),
            Point::new(center.x, center.y - radius_y),
        ));
        self.append_command(Command::BezierTo(
            Point::new(center.x - radius_x * KAPPA90, center.y - radius_y),
            Point::new(center.x - radius_x, center.y - radius_y * KAPPA90),
            Point::new(center.x - radius_x, center.y),
        ));
        self.append_command(Command::Close);
    }

    pub fn circle<P: Into<Point>>(&mut self, center: P, radius: f32) {
        self.ellipse(center.into(), radius, radius);
    }

    pub fn fill(&mut self) -> anyhow::Result<()> {
        let state = self.states.last_mut().unwrap();
        let mut fill_paint = state.fill.clone();

        self.cache
            .flatten_paths(&self.commands, self.dist_tol, self.tess_tol);
        if self.renderer.edge_antialias() && state.shape_antialias {
            self.cache
                .expand_fill(self.fringe_width, LineJoin::Miter, 2.4, self.fringe_width);
        } else {
            self.cache
                .expand_fill(0.0, LineJoin::Miter, 2.4, self.fringe_width);
        }

        fill_paint.inner_color.a *= state.alpha;
        fill_paint.outer_color.a *= state.alpha;

        self.renderer.fill(
            &fill_paint,
            state.composite_operation,
            &state.scissor,
            self.fringe_width,
            self.cache.bounds,
            &self.cache.paths,
        )?;

        for path in &self.cache.paths {
            if path.num_fill > 2 {
                self.fill_triangles_count += path.num_fill - 2;
            }
            if path.num_stroke > 2 {
                self.fill_triangles_count += path.num_stroke - 2;
            }
            self.draw_call_count += 2;
        }

        Ok(())
    }

    pub fn stroke(&mut self) -> anyhow::Result<()> {
        let state = self.states.last_mut().unwrap();
        let scale = state.xform.average_scale();
        let mut stroke_width = (state.stroke_width * scale).clamp(0.0, 200.0);
        let mut stroke_paint = state.stroke.clone();

        if stroke_width < self.fringe_width {
            let alpha = (stroke_width / self.fringe_width).clamp(0.0, 1.0);
            stroke_paint.inner_color.a *= alpha * alpha;
            stroke_paint.outer_color.a *= alpha * alpha;
            stroke_width = self.fringe_width;
        }

        stroke_paint.inner_color.a *= state.alpha;
        stroke_paint.outer_color.a *= state.alpha;

        self.cache
            .flatten_paths(&self.commands, self.dist_tol, self.tess_tol);

        if self.renderer.edge_antialias() && state.shape_antialias {
            self.cache.expand_stroke(
                stroke_width * 0.5,
                self.fringe_width,
                state.line_cap,
                state.line_join,
                state.miter_limit,
                self.tess_tol,
            );
        } else {
            self.cache.expand_stroke(
                stroke_width * 0.5,
                0.0,
                state.line_cap,
                state.line_join,
                state.miter_limit,
                self.tess_tol,
            );
        }

        self.renderer.stroke(
            &stroke_paint,
            state.composite_operation,
            &state.scissor,
            self.fringe_width,
            stroke_width,
            &self.cache.paths,
        )?;

        for path in &self.cache.paths {
            self.fill_triangles_count += path.num_stroke - 2;
            self.draw_call_count += 1;
        }

        Ok(())
    }

    pub fn create_font_from_file<N: Into<String>, P: AsRef<std::path::Path>>(
        &mut self,
        name: N,
        path: P,
    ) -> anyhow::Result<FontId> {
        self.create_font(name, std::fs::read(path)?)
    }

    pub fn create_font<N: Into<String>, D: Into<Vec<u8>>>(
        &mut self,
        name: N,
        data: D,
    ) -> anyhow::Result<FontId> {
        self.fonts.add_font(name, data)
    }

    pub fn find_font<N: AsRef<str>>(&self, name: N) -> Option<FontId> {
        self.fonts.find(name.as_ref())
    }

    pub fn add_fallback_fontid(&mut self, base: FontId, fallback: FontId) {
        self.fonts.add_fallback(base, fallback);
    }

    pub fn add_fallback_font<N1: AsRef<str>, N2: AsRef<str>>(&mut self, base: N1, fallback: N2) {
        if let (Some(base), Some(fallback)) = (self.find_font(base), self.find_font(fallback)) {
            self.fonts.add_fallback(base, fallback);
        }
    }

    pub fn font_size(&mut self, size: f32) {
        self.state_mut().font_size = size;
    }

    pub fn text_letter_spacing(&mut self, spacing: f32) {
        self.state_mut().letter_spacing = spacing;
    }

    pub fn text_line_height(&mut self, line_height: f32) {
        self.state_mut().line_height = line_height;
    }

    pub fn text_align(&mut self, align: Align) {
        self.state_mut().text_align = align;
    }

    pub fn fontid(&mut self, id: FontId) {
        self.state_mut().font_id = id;
    }

    pub fn font<N: AsRef<str>>(&mut self, name: N) {
        if let Some(id) = self.find_font(name) {
            self.state_mut().font_id = id;
        }
    }

    pub fn text<S: AsRef<str>, P: Into<Point>>(&mut self, pt: P, text: S) -> anyhow::Result<()> {
        let state = self.states.last().unwrap();
        let scale = state.xform.font_scale() * self.device_pixel_ratio;
        let xform = &state.xform;
        let invscale = 1.0 / scale;
        let pt = pt.into();

        self.fonts.layout_text(
            &mut self.renderer,
            text.as_ref(),
            state.font_id,
            (pt.x * scale, pt.y * scale).into(),
            state.font_size * scale,
            state.text_align,
            state.letter_spacing * scale,
            true,
            &mut self.layout_chars,
        )?;

        self.cache.vertexes.clear();

        for lc in &self.layout_chars {
            let lt = xform.transform_point(Point::new(
                lc.bounds.min.x * invscale,
                lc.bounds.min.y * invscale,
            ));
            let rt = xform.transform_point(Point::new(
                lc.bounds.max.x * invscale,
                lc.bounds.min.y * invscale,
            ));
            let lb = xform.transform_point(Point::new(
                lc.bounds.min.x * invscale,
                lc.bounds.max.y * invscale,
            ));
            let rb = xform.transform_point(Point::new(
                lc.bounds.max.x * invscale,
                lc.bounds.max.y * invscale,
            ));

            self.cache
                .vertexes
                .push(Vertex::new(lt.x, lt.y, lc.uv.min.x, lc.uv.min.y));
            self.cache
                .vertexes
                .push(Vertex::new(rb.x, rb.y, lc.uv.max.x, lc.uv.max.y));
            self.cache
                .vertexes
                .push(Vertex::new(rt.x, rt.y, lc.uv.max.x, lc.uv.min.y));

            self.cache
                .vertexes
                .push(Vertex::new(lt.x, lt.y, lc.uv.min.x, lc.uv.min.y));
            self.cache
                .vertexes
                .push(Vertex::new(lb.x, lb.y, lc.uv.min.x, lc.uv.max.y));
            self.cache
                .vertexes
                .push(Vertex::new(rb.x, rb.y, lc.uv.max.x, lc.uv.max.y));
        }

        let mut paint = state.fill.clone();
        paint.image = Some(self.fonts.img.clone());
        paint.inner_color.a *= state.alpha;
        paint.outer_color.a *= state.alpha;

        self.renderer.triangles(
            &paint,
            state.composite_operation,
            &state.scissor,
            &self.cache.vertexes,
        )?;
        Ok(())
    }

    pub fn text_metrics(&self) -> TextMetrics {
        let state = self.states.last().unwrap();
        let scale = state.xform.font_scale() * self.device_pixel_ratio;
        self.fonts
            .text_metrics(state.font_id, state.font_size * scale)
    }

    pub fn text_size<S: AsRef<str>>(&self, text: S) -> Extent {
        let state = self.states.last().unwrap();
        let scale = state.xform.font_scale() * self.device_pixel_ratio;
        self.fonts.text_size(
            text.as_ref(),
            state.font_id,
            state.font_size * scale,
            state.letter_spacing * scale,
        )
    }
}