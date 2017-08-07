use serde::ser::{Serialize, Serializer};

use graphics::Style;

impl Serialize for Style {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            Style::Stroke => "stroke",
            Style::Fill => "fill"
        })
    }
}
