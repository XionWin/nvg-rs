use super::{Point, Extent};
use crate::as_primitive::AsPrimitive;

#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    pub xy: Point,
    pub size: Extent,
}

impl Rect {
    pub fn new(xy: Point, size: Extent) -> Rect {
        Rect { xy, size }
    }

    pub fn intersect(self, rect: Rect) -> Rect {
        let Rect {
            xy: Point { x: ax, y: ay },
            size: Extent {
                width: aw,
                height: ah,
            },
        } = rect;

        let Rect {
            xy: Point { x: bx, y: by },
            size: Extent {
                width: bw,
                height: bh,
            },
        } = rect;

        let minx = ax.max(bx);
        let miny = ay.max(by);
        let maxx = (ax + aw).min(bx + bw);
        let maxy = (ay + ah).min(by + bh);
        Self::new(
            Point::new(minx, miny),
            Extent::new((maxx - minx).max(0.0), (maxy - miny).max(0.0)),
        )
    }

    pub fn grow(&self, width: f32, height: f32) -> Rect {
        Rect::new(
            self.xy.offset(-width / 2.0, -height / 2.0),
            Extent::new(self.size.width + width, self.size.height + height),
        )
    }
}

impl<T: AsPrimitive<f32>> From<(T, T, T, T)> for Rect {
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Rect::new((x.as_(), y.as_()).into(), (w.as_(), h.as_()).into())
    }
}