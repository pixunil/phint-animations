use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};
use std::f64::consts;
use cairo::Context;

use utils;

const TAU: f64 = 2.0 * consts::PI;

#[derive(Default, Clone, Copy, PartialEq, Debug, Serialize)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }

    pub fn on_circle(&self, radius: f64, angle: f64) -> Point {
        Point {
            x: self.x + radius * angle.cos(),
            y: self.y + radius * angle.sin()
        }
    }

    pub fn on_oval(&self, radiusx: f64, radiusy: f64, angle: f64) -> Point {
        Point {
            x: self.x + radiusx * angle.cos(),
            y: self.y + radiusy * angle.sin()
        }
    }

    pub fn lerp(self, target: Point, t: f64) -> Point {
        self + t * (target - self)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point::new(x, y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        Point {
            x: self * point.x,
            y: self * point.y
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scale: f64) -> Point {
        Point {
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, scale: f64) -> Point {
        Point {
            x: self.x / scale,
            y: self.y / scale
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Segment {
    Line(Line),
    Arc(Arc),
    OvalArc(OvalArc),
    BezierCurve(BezierCurve)
}

impl Segment {
    pub fn draw(&self, ctx: &Context, begin: bool) {
        match *self {
            Segment::Line(ref line) => line.draw(ctx, begin),
            Segment::Arc(ref arc) => arc.draw(ctx),
            Segment::OvalArc(ref arc) => arc.draw(ctx),
            Segment::BezierCurve(ref bezier) => bezier.draw(ctx, begin)
        }
    }

    pub fn count_beziers(&self) -> usize {
        match *self {
            Segment::Arc(ref arc) => {
                let phi = arc.end - arc.start;
                let count = (phi / consts::FRAC_PI_2) as f32;
                count.ceil().abs() as usize
            },
            Segment::OvalArc(ref arc) => {
                let phi = arc.end - arc.start;
                let count = (phi / consts::FRAC_PI_2).ceil().abs();
                count as usize
            },
            _ => 1
        }
    }

    pub fn to_beziers(&self, count: usize) -> Vec<BezierCurve> {
        match *self {
            Segment::Line(ref line) => {
                let diff = (line.end - line.start) / count as f64;

                (0..count).scan(line.start, |point, _| {
                    let start = *point;
                    *point += diff;
                    let line = Line::new(start, *point);
                    Some(line.into())
                }).collect()
            },
            Segment::Arc(ref arc) => {
                let phi = (arc.end - arc.start) / count as f64;

                (0..count).scan(arc.start, |angle, _| {
                    let start = *angle;
                    *angle += phi;
                    let arc = Arc::new(arc.center, arc.radius, start, *angle);
                    Some(arc.into())
                }).collect()
            },
            Segment::OvalArc(ref arc) => {
                let phi = (arc.end - arc.start) / count as f64;

                (0..count).scan(arc.start, |angle, _| {
                    let start = *angle;
                    *angle += phi;
                    let arc = OvalArc::new(arc.center, arc.radiusx, arc.radiusy, start, *angle);
                    Some(arc.into())
                }).collect()
            },
            Segment::BezierCurve(ref bezier) => {
                (0..count).scan(bezier.clone(), |bezier, i| {
                    let t = 1.0 / (count - i) as f64;
                    let hull = bezier.hull(t);
                    *bezier = BezierCurve::new(hull[0], hull[4], hull[7], hull[9]);
                    Some(bezier.clone())
                }).collect()
            }
        }
    }
}

impl From<Line> for Segment {
    fn from(value: Line) -> Segment {
        Segment::Line(value)
    }
}

impl From<Arc> for Segment {
    fn from(value: Arc) -> Segment {
        Segment::Arc(value)
    }
}

impl From<OvalArc> for Segment {
    fn from(value: OvalArc) -> Segment {
        Segment::OvalArc(value)
    }
}

impl From<BezierCurve> for Segment {
    fn from(value: BezierCurve) -> Segment {
        Segment::BezierCurve(value)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn new<P: Into<Point>>(start: P, end: P) -> Line {
        Line {
            start: start.into(),
            end: end.into()
        }
    }

    fn draw(&self, ctx: &Context, begin: bool) {
        if begin {
            ctx.move_to(self.start.x, self.start.y);
        } else {
            ctx.line_to(self.start.x, self.start.y);
        }

        ctx.line_to(self.end.x, self.end.y);
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Arc {
    center: Point,
    radius: f64,
    #[serde(deserialize_with = "utils::deserialize::angle")]
    start: f64,
    #[serde(deserialize_with = "utils::deserialize::angle")]
    end: f64
}

impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Arc {
        Arc {center, radius, start, end}
    }

    fn draw(&self, ctx: &Context) {
        if self.start < self.end {
            ctx.arc(self.center.x, self.center.y,
                self.radius, self.start, self.end);
        } else {
            ctx.arc_negative(self.center.x, self.center.y,
                self.radius, self.start, self.end);
        }
    }
}

impl Default for Arc {
    fn default() -> Arc {
        Arc {
            center: Point::default(),
            radius: 1.0,
            start: 0.0,
            end: TAU
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OvalArc {
    center: Point,
    radiusx: f64,
    radiusy: f64,
    #[serde(deserialize_with = "utils::deserialize::angle")]
    start: f64,
    #[serde(deserialize_with = "utils::deserialize::angle")]
    end: f64
}

impl OvalArc {
    fn new(center: Point, radiusx: f64, radiusy: f64, start: f64, end: f64) -> OvalArc {
        OvalArc {center, radiusx, radiusy, start, end}
    }

    fn draw(&self, ctx: &Context) {
        ctx.save();
        ctx.translate(self.center.x, self.center.y);
        ctx.scale(self.radiusx, self.radiusy);
        ctx.arc(0.0, 0.0, 1.0, self.start, self.end);
        ctx.restore();
    }
}

impl Default for OvalArc {
    fn default() -> OvalArc {
        OvalArc {
            center: Point::default(),
            radiusx: 1.0,
            radiusy: 1.0,
            start: 0.0,
            end: TAU
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BezierCurve {
    start: Point,
    control1: Point,
    control2: Point,
    end: Point
}

impl BezierCurve {
    fn new(start: Point, control1: Point, control2: Point, end: Point) -> BezierCurve {
        BezierCurve {start, control1, control2, end}
    }

    fn hull(&self, state: f64) -> Vec<Point> {
        // use "de Casteljau" iteration
        let mut points = vec![self.start, self.control1, self.control2, self.end];
        let mut hull = points.clone();

        // lerp between all points at each iteration, until only one point remains
        while points.len() > 1 {
            points = points.windows(2).map(|pair| {
                pair[0].lerp(pair[1], state)
            }).collect();

            hull.extend(points.iter().cloned())
        }

        hull
    }

    pub fn lerp(&self, other: &BezierCurve, t: f64) -> BezierCurve {
        BezierCurve {
            start: self.start.lerp(other.start, t),
            control1: self.control1.lerp(other.control1, t),
            control2: self.control2.lerp(other.control2, t),
            end: self.end.lerp(other.end, t)
        }
    }

    pub fn draw(&self, ctx: &Context, begin: bool) {
        if begin {
            ctx.move_to(self.start.x, self.start.y);
        } else {
            ctx.line_to(self.start.x, self.start.y);
        }

        ctx.curve_to(self.control1.x, self.control1.y,
            self.control2.x, self.control2.y, self.end.x, self.end.y);
    }
}

const C: f64 = 0.551915024494;

impl From<Line> for BezierCurve {
    fn from(value: Line) -> BezierCurve {
        let difference = (value.end - value.start) / 3.0;

        BezierCurve::new(value.start, value.start + difference,
            value.end - difference, value.end)
    }
}

impl From<Arc> for BezierCurve {
    fn from(value: Arc) -> BezierCurve {
        let mut fraction = (value.start - value.end) / consts::FRAC_PI_2;
        fraction *= value.radius * C;

        let start = value.center.on_circle(value.radius, value.start);
        let end = value.center.on_circle(value.radius, value.end);

        BezierCurve::new(start, start.on_circle(fraction, value.start - consts::FRAC_PI_2),
            end.on_circle(fraction, value.end + consts::FRAC_PI_2), end)
    }
}

impl From<OvalArc> for BezierCurve {
    fn from(value: OvalArc) -> BezierCurve {
        let fraction = (value.start - value.end) / consts::FRAC_PI_2;
        let dx = value.radiusx * C * fraction;
        let dy = value.radiusy * C * fraction;

        let start = value.center.on_oval(value.radiusx, value.radiusy, value.start);
        let end = value.center.on_oval(value.radiusx, value.radiusy, value.end);

        BezierCurve::new(start, start.on_oval(dx, dy, value.start - consts::FRAC_PI_2),
            end.on_oval(dx, dy, value.end + consts::FRAC_PI_2), end)
    }
}
