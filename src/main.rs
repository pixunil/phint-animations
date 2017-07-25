#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod graphics;

use graphics::{Graphic, Point, Line, Arc};

macro_rules! angle {
    ($angle:expr) => {
        ($angle as f64).to_radians()
    }
}

fn main() {
    let center = Point::default();
    let mut polius = Graphic::new((0.0, 0.0, 1.0));
    let mut angle = 0;

    while angle < 360 {
        let start = center.on_circle(0.8, angle!(angle))
            .on_circle(-0.25, angle!(angle - 30));

        angle += 120;

        let end = center.on_circle(0.8, angle!(angle))
            .on_circle(-0.25, angle!(angle + 30));

        let line = Line::new(start, end);
        polius.add(line);

        let center = center.on_circle(0.8, angle!(angle));
        let start = angle!(angle - 230);
        let end = angle!(angle - 130);
        let arc = Arc::new(center, 0.25, start, end);
        polius.add(arc);
    }

    println!("{}", serde_json::to_string_pretty(&polius).unwrap());
}
