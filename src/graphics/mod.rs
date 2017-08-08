use cairo::Context;

mod segments;
mod single;
mod morph;

pub use self::segments::Point;
pub use self::single::Graphic;
pub use self::morph::MorphGraphic;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Style {
    Stroke,
    Fill
}

impl Style {
    fn paint(self, ctx: &Context) {
        match self {
            Style::Stroke => Context::stroke(ctx),
            Style::Fill => Context::fill(ctx)
        }
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::Stroke
    }
}
