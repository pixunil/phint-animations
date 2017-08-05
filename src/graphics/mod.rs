use std::cmp;
use std::path::Path;
use std::fs::File;
use std::error;
use serde_json;
use cairo::Context;

mod segments;

use self::segments::{Segment, BezierCurve};
pub use self::segments::{Point, Line, Arc};
use utils::{self, lerp};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Graphic {
    color: (f64, f64, f64),
    groups: Vec<Group>
}

impl Graphic {
    pub fn new(color: (f64, f64, f64)) -> Graphic {
        Graphic {color, groups: Vec::new()}
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Graphic, Box<error::Error>> {
        let file = File::open(path)?;
        let graphic = serde_json::from_reader(file)?;
        Ok(graphic)
    }

    fn count_beziers(&self) -> usize {
        self.groups.iter().map(Group::count_beziers).sum()
    }

    pub fn draw(&self, ctx: &Context) {
        ctx.set_source_rgb(self.color.0, self.color.1, self.color.2);

        for group in &self.groups {
            group.draw(ctx);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGraphic<'a> {
    pub start: BezierGraphic<'a>,
    pub target: BezierGraphic<'a>
}

impl<'a> MorphGraphic<'a> {
    pub fn new(start: &'a Graphic, target: &'a Graphic) -> MorphGraphic<'a> {
        let start_count = start.count_beziers();
        let target_count = target.count_beziers();
        let count = cmp::max(start_count, target_count);

        MorphGraphic {
            start: BezierGraphic::new(start, (start_count, count)),
            target: BezierGraphic::new(target, (target_count, count))
        }
    }

    pub fn draw(&self, ctx: &Context, t: f64) {
        ctx.set_source_rgb(
            lerp(self.start.graphic.color.0, self.target.graphic.color.0, t),
            lerp(self.start.graphic.color.1, self.target.graphic.color.1, t),
            lerp(self.start.graphic.color.2, self.target.graphic.color.2, t)
        );

        for (start, target) in self.start.beziers.iter().zip(&self.target.beziers) {
            start.draw(target, ctx, t);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BezierGraphic<'a> {
    pub graphic: &'a Graphic,
    beziers: Vec<BezierSegment<'a>>
}

impl<'a> BezierGraphic<'a> {
    fn new(graphic: &'a Graphic, count: (usize, usize)) -> BezierGraphic<'a> {
        let mut i = 0;
        let mut beziers = Vec::with_capacity(count.1);

        for group in &graphic.groups {
            for segment in &group.segments {
                let mut splits = segment.count_beziers();
                splits += count.1 / count.0 - 1;

                if i < count.1 % count.0 {
                    splits += 1;
                }

                let splits = segment.to_beziers(splits).into_iter()
                    .map(|bezier| BezierSegment::new(bezier, group));

                i += 1;
                beziers.extend(splits);
            }
        }

        BezierGraphic {graphic, beziers}
    }
}

#[derive(Clone, PartialEq, Debug)]
struct BezierSegment<'a> {
    bezier: BezierCurve,
    group: &'a Group
}

impl<'a> BezierSegment<'a> {
    fn new(bezier: BezierCurve, group: &'a Group) -> BezierSegment<'a> {
        BezierSegment {bezier, group}
    }

    fn draw(&self, target: &BezierSegment<'a>, ctx: &Context, t: f64) {
        self.bezier.lerp(&target.bezier, t).draw(ctx, true);

        let style = self.group.style.lerp(&target.group.style, t);
        let line_width = lerp(self.group.line_width, target.group.line_width, t);
        style.draw(ctx, line_width);
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize)]
pub struct Style {
    stroke: f64,
    fill: f64
}

impl Style {
    pub fn stroke() -> Style {
        Style {
            stroke: 1.0,
            fill: 0.0
        }
    }

    pub fn fill() -> Style {
        Style {
            stroke: 0.0,
            fill: 1.0
        }
    }

    fn draw(&self, ctx: &Context, line_width: f64) {
        if self.stroke > 0.0 {
            ctx.set_line_width(self.stroke * line_width);
            ctx.stroke();
        }

        if self.fill == 1.0 {
            ctx.fill();
        } else if self.fill > 0.0 {
            ctx.save();
            ctx.clip();
            ctx.paint_with_alpha(self.fill);
            ctx.restore();
        }
    }

    fn lerp(&self, target: &Style, t: f64) -> Style {
        Style {
            stroke: lerp(self.stroke, target.stroke, t),
            fill: lerp(self.fill, target.fill, t)
        }
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::stroke()
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Group {
    segments: Vec<Segment>,
    style: Style,
    #[serde(deserialize_with = "utils::deserialize::line_width")]
    line_width: f64,
    close: bool
}

impl Group {
    fn count_beziers(&self) -> usize {
        self.segments.iter().map(Segment::count_beziers).sum()
    }

    fn draw(&self, ctx: &Context) {
        let mut begin = true;

        for segment in &self.segments {
            segment.draw(ctx, begin);
            begin = false;
        }

        if self.close {
            ctx.close_path();
        }

        self.style.draw(ctx, self.line_width);
    }
}

impl Default for Group {
    fn default() -> Group {
        Group {
            segments: Vec::new(),
            style: Style::default(),
            line_width: 0.1,
            close: false
        }
    }
}

impl<T: Into<Segment>> From<T> for Group {
    fn from(value: T) -> Group {
        Group {
            segments: vec![value.into()],
            .. Group::default()
        }
    }
}
