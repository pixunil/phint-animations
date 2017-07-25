mod segments;

pub use self::segments::{Point, Line, Arc};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Graphic {
    color: (f64, f64, f64),
    groups: Vec<Group>
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
pub struct Group {
    segments: Vec<segments::Segment>,
    close: bool
}

impl<T: Into<segments::Segment>> From<T> for Group {
    fn from(value: T) -> Group {
        Group {
            segments: vec![value.into()],
            close: false
        }
    }
}
