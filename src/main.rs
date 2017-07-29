extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate gtk;
extern crate cairo;

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::ffi::OsStr;
use gtk::prelude::*;
use cairo::LineJoin;

mod graphics;
mod utils;

use graphics::Graphic;

fn scan<P: AsRef<Path>>(path: P) -> io::Result<(Vec<String>, Vec<Graphic>)> {
    let mut names = Vec::new();
    let mut graphics = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let graphic = match Graphic::load(&path) {
                Ok(graphic) => graphic,
                Err(err) => {
                    println!("error while processing {}: {}", path.to_string_lossy(), err);
                    continue;
                }
            };

            let name = match path.file_stem().and_then(OsStr::to_str) {
                Some(name) => name,
                None => {
                    println!("error while processing {}: invalid name", path.to_string_lossy());
                    continue;
                }
            };

            names.push(name.into());
            graphics.push(graphic);
        }
    }

    Ok((names, graphics))
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).map(String::as_ref).unwrap_or("data");

    let (names, graphics) = scan(path).unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("phint");
    window.set_icon_name("applications-graphics");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&container);

    let bar = gtk::Toolbar::new();
    container.pack_start(&bar, false, false, 0);

    let graphic_chooser = gtk::ComboBoxText::new();
    let item = gtk::ToolItem::new();
    bar.insert(&item, -1);
    item.add(&graphic_chooser);

    for name in &names {
        graphic_chooser.append_text(name);
    }

    let canvas = gtk::DrawingArea::new();
    container.pack_start(&canvas, true, true, 0);

    let canvas_redraw = canvas.clone();
    graphic_chooser.connect_changed(move |_| canvas_redraw.queue_draw());
    graphic_chooser.set_active(0);

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

        let graphic = &graphics[graphic_chooser.get_active() as usize];
        graphic.draw(ctx);

        gtk::Inhibit(false)
    });

    window.show_all();
    window.maximize();
    gtk::main();
}
