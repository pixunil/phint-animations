#!/usr/bin/env python3

from utils import *
import drawElements
from context import Context
from export import ExportDialog
from graphics import polius
from gi.repository import Gtk

class Window(Gtk.Window):
    def __init__(self):
        Gtk.Window.__init__(self, icon_name = "applications-graphics", title = "phint")

        self.connect("destroy", self.on_destroy)

        box = Gtk.Box(orientation = Gtk.Orientation.VERTICAL)
        self.add(box)

        bar = Gtk.Toolbar()
        box.pack_start(bar, False, False, 0)

        export = Gtk.ToolButton(label = "Export", icon_name = "document-save")
        export.connect("clicked", self.export)
        bar.insert(export, -1)

        background = drawElements.Background()
        content = polius.Shape()

        self.elements = drawElements.Shape()
        self.elements.shapes = (background, content)

        canvas = Canvas(self)
        box.pack_start(canvas, True, True, 0)

        self.set_icon_name("applications-graphics")
        self.set_title("phint")
        self.maximize()
        self.show_all()

    def on_destroy(self, *args):
        Gtk.main_quit()

    def export(self, *args):
        self.export_dialog.run(self.elements, "polius")

    @lazy
    def export_dialog(self):
        return ExportDialog(self)

class Canvas(Gtk.DrawingArea):
    def __init__(self, window):
        Gtk.DrawingArea.__init__(self)

        self.window = window

        self.connect("draw", self.on_draw)

    def on_draw(self, widget, ctx):
        width = self.get_allocated_width()
        height = self.get_allocated_height()

        # use the simplified context
        ctx = Context(width, height, ctx)

        self.window.elements.draw(ctx)

if __name__ == "__main__":
    Window()
    Gtk.main()
