#!/usr/bin/env python3

import drawElements
from context import Context
from graphics import polius
from gi.repository import Gtk

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

class Canvas(Gtk.DrawingArea):
    def __init__(self):
        Gtk.DrawingArea.__init__(self)

        self.background = drawElements.Background()
        self.polius = polius.Shape()

        self.connect("draw", self.onDraw)

    def onDraw(self, widget, ctx):
        width = self.get_allocated_width()
        height = self.get_allocated_height()

        # use the simplified context
        ctx = Context(width, height, ctx)

        self.background.draw(ctx)
        self.polius.draw(ctx)

if __name__ == "__main__":
    Window()
    Gtk.main()
