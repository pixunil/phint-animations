extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate gtk;
extern crate cairo;

use std::env;
use gtk::prelude::*;
use cairo::LineJoin;

mod graphics;
mod utils;

use graphics::Graphic;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).map(String::as_ref).unwrap_or("data/polius.json");

    let graphic = match Graphic::load(path) {
        Ok(graphic) => graphic,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

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

    canvas.connect_draw(move |canvas, ctx| {
        let gtk::Allocation {width, height, ..} = canvas.get_allocation();
        let (width, height) = (width as f64, height as f64);

        // pick the smaller of width and height, to have the whole picture fitted
        let size = f64::min(width, height);
        // normalize context and change vertical direction
        ctx.scale(size / 2.0, -size / 2.0);
        // align the point 0,0 to the middle
        ctx.translate(width / size, -height / size);

        ctx.set_line_join(LineJoin::Round);

        graphic.draw(ctx);

        gtk::Inhibit(false)
    });

    window.show_all();
    window.maximize();
    gtk::main();
}
