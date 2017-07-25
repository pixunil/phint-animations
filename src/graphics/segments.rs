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
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BezierCurve {
    start: Point,
    control1: Point,
    control2: Point,
    end: Point
}
