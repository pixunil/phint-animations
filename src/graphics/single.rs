use std::slice::Iter;
use std::path::Path;
use std::fs::File;
use std::error;
use serde_json;
use cairo::Context;

use utils;
use super::Style;
use super::segments::Segment;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Graphic {
    color: (f64, f64, f64),
    groups: Vec<Group>
}

impl Graphic {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Graphic, Box<error::Error>> {
        let file = File::open(path)?;
        let graphic = serde_json::from_reader(file)?;
        Ok(graphic)
    }

    pub fn color(&self) -> &(f64, f64, f64) {
        &self.color
    }

    pub fn group(&self, index: usize) -> &Group {
        &self.groups[index]
    }

    pub fn groups(&self) -> Iter<Group> {
        self.groups.iter()
    }

    pub fn count_beziers(&self) -> usize {
        self.groups().map(Group::count_beziers).sum()
    }

    pub fn draw(&self, ctx: &Context) {
        ctx.set_source_rgb(self.color.0, self.color.1, self.color.2);

        for group in &self.groups {
            group.draw(ctx);
        }
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
    pub fn segments(&self) -> Iter<Segment> {
        self.segments.iter()
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn line_width(&self) -> f64 {
        self.line_width
    }

    pub fn close(&self) -> bool {
        self.close
    }

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

        ctx.set_line_width(self.line_width);
        self.style.paint(ctx);
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
