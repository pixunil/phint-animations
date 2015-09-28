#!/usr/bin/env python3

from gi.repository import Gtk
import drawElements

class Window(Gtk.Window):
    def __init__(self):
        Gtk.Window.__init__(self)

        self.connect("destroy", self.onDestroy)

        canvas = Canvas()
        self.add(canvas)

        self.set_icon_name("applications-graphics")
        self.set_title("phint")
        self.maximize()
        self.show_all()

    def onDestroy(self, *args):
        Gtk.main_quit()

class Sizes:
    def __init__(self, width, height):
        self.width = width
        self.height = height

class Canvas(Gtk.DrawingArea):
    def __init__(self):
        Gtk.DrawingArea.__init__(self)

        self.background = drawElements.Background()

        self.connect("draw", self.onDraw)

    def onDraw(self, widget, ctx):
        width = self.get_allocated_width()
        height = self.get_allocated_height()
        # pick the smaller of width and height, to have the whole picture fitted
        size = min(width, height)

        # save the scaled values for the draw elements
        sizes = Sizes(width / size, height / size)

        ctx.scale(size / 2, size / 2)
        # align the point 0,0 to the middle
        ctx.translate(sizes.width, sizes.height)

        self.background.draw(ctx, sizes)

if __name__ == "__main__":
    Window()
    Gtk.main()
