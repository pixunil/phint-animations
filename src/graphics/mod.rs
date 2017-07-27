use cairo::Context;

mod segments;

pub use self::segments::{Point, Line, Arc};

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
}

impl Graphic {
    pub fn new(color: (f64, f64, f64)) -> Graphic {
        Graphic {color, groups: Vec::new()}
    }

    pub fn add<T: Into<Group>>(&mut self, group: T) {
        self.groups.push(group.into());
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    segments: Vec<segments::Segment>,
    close: bool
}

impl Group {
    fn draw(&self, ctx: &Context) {
        let mut begin = true;

        for segment in &self.segments {
            segment.draw(ctx, begin);
            begin = false;
        }

        if self.close {
            ctx.close_path();
        }

        ctx.stroke();
    }
}

impl Default for Group {
    fn default() -> Group {
        Group {
            segments: Vec::new(),
            close: false
        }
    }
}

impl<T: Into<segments::Segment>> From<T> for Group {
    fn from(value: T) -> Group {
        Group {
            segments: vec![value.into()],
            close: false
        }
    }
}
