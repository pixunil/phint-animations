#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate gtk;
extern crate cairo;

use std::error::Error;
use std::fs::File;
use gtk::prelude::*;
use cairo::LineJoin;

mod graphics;

use graphics::{Graphic, Point, Line, Arc};

macro_rules! angle {
    ($angle:expr) => {
        ($angle as f64).to_radians()
    }
}

fn build() -> Graphic {
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

    polius
}

fn init() -> Result<Graphic, Box<Error>> {
    let file = File::open("data/polius.json")?;
    let graphic = serde_json::from_reader(file)?;
    Ok(graphic)
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("phint");
    window.set_icon_name("applications-graphics");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let canvas = gtk::DrawingArea::new();
    window.add(&canvas);

    let polius = match init() {
        Ok(graphic) => graphic,
        Err(_) => build()
    };

    canvas.connect_draw(move |canvas, ctx| {
        let gtk::Allocation {width, height, ..} = canvas.get_allocation();
        let (width, height) = (width as f64, height as f64);

        // pick the smaller of width and height, to have the whole picture fitted
        let size = f64::min(width, height);
        // normalize context and change vertical direction
        ctx.scale(size / 2.0, -size / 2.0);
        // align the point 0,0 to the middle
        ctx.translate(width / size, -height / size);

        ctx.set_line_width(0.1);
        ctx.set_line_join(LineJoin::Round);

        polius.draw(ctx);

        gtk::Inhibit(false)
    });

    window.show_all();
    window.maximize();
    gtk::main();
}
