use cairo::Context;

#[derive(Default, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point::new(x, y)
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
pub struct Arc {
    center: Point,
    radius: f64,
    start: f64,
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct OvalArc {
    center: Point,
    radiusx: f64,
    radiusy: f64,
    start: f64,
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BezierCurve {
    start: Point,
    control1: Point,
    control2: Point,
    end: Point
}

impl BezierCurve {
    fn draw(&self, ctx: &Context, begin: bool) {
        if begin {
            ctx.move_to(self.start.x, self.start.y);
        } else {
            ctx.line_to(self.start.x, self.start.y);
        }

        ctx.curve_to(self.control1.x, self.control1.y,
            self.control2.x, self.control2.y, self.end.x, self.end.y);
    }
}
