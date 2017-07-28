use std::path::Path;
use std::fs::File;
use std::error;
use serde_json;
use cairo::Context;

mod segments;

pub use self::segments::{Point, Line, Arc};
use utils;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Graphic {
    color: (f64, f64, f64),
    groups: Vec<Group>
}

impl Graphic {
    pub fn draw(&self, ctx: &Context) {
        ctx.set_source_rgb(self.color.0, self.color.1, self.color.2);

        for group in &self.groups {
            group.draw(ctx);
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Graphic, Box<error::Error>> {
        let file = File::open(path)?;
        let graphic = serde_json::from_reader(file)?;
        Ok(graphic)
    }
}

impl Graphic {
    pub fn new(color: (f64, f64, f64)) -> Graphic {
        Graphic {color, groups: Vec::new()}
    }

    pub fn add<T: Into<Group>>(&mut self, group: T) {
        self.groups.push(group.into());
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    Stroke,
    Fill
}

impl Default for Style {
    fn default() -> Style {
        Style::Stroke
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Group {
    segments: Vec<segments::Segment>,
    style: Style,
    #[serde(deserialize_with = "utils::deserialize::line_width")]
    line_width: f64,
    close: bool
}

impl Group {
    fn draw(&self, ctx: &Context) {
        ctx.set_line_width(self.line_width);

        let mut begin = true;

        for segment in &self.segments {
            segment.draw(ctx, begin);
            begin = false;
        }

        if self.close {
            ctx.close_path();
        }

        match self.style {
            Style::Stroke => ctx.stroke(),
            Style::Fill => ctx.fill()
        }
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

impl<T: Into<segments::Segment>> From<T> for Group {
    fn from(value: T) -> Group {
        Group {
            segments: vec![value.into()],
            .. Group::default()
        }
    }
}
