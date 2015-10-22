#!/usr/bin/env python3

from utils import *
import drawElements
from context import Context
from export import ExportDialog
from graphics import graphics
from gi.repository import Gtk

class Window(Gtk.Window):
    def __init__(self):
        Gtk.Window.__init__(self, icon_name = "applications-graphics", title = "phint")

        self.connect("destroy", self.on_destroy)

        box = Gtk.Box(orientation = Gtk.Orientation.VERTICAL)
        self.add(box)

        bar = Gtk.Toolbar(orientation = Gtk.Orientation.HORIZONTAL)
        box.pack_start(bar, False, False, 0)

        export = Gtk.ToolButton(label = "Export", icon_name = "document-save")
        export.connect("clicked", self.export)
        bar.insert(export, -1)

        self.model = Gtk.ListStore(str)

        for name in graphics:
            self.model.append([name])

        self.combobox = Gtk.ComboBox.new_with_model(self.model)
        renderer = Gtk.CellRendererText()
        self.combobox.pack_start(renderer, True)
        self.combobox.add_attribute(renderer, "text", 0)
        self.combobox.set_active(0)
        self.combobox.set_id_column(0)

        self.combobox.connect("changed", self.on_graphic_changed)

        item = Gtk.ToolItem()
        item.add(self.combobox)
        bar.insert(item, -1)

        self.elements = drawElements.Shape()

        background = drawElements.Background()
        self.elements.shapes.append(background)

        graphic = self.combobox.get_active_id()
        # get the module and create the shape
        graphic = graphics[graphic].Shape()
        self.elements.shapes.append(graphic)

        self.canvas = Canvas(self)
        box.pack_start(self.canvas, True, True, 0)

        self.set_icon_name("applications-graphics")
        self.set_title("phint")
        self.maximize()
        self.show_all()

    def on_destroy(self, *args):
        Gtk.main_quit()

    def on_graphic_changed(self, combobox):
        graphic = combobox.get_active_id()
        # get the module and create the shape
        graphic = graphics[graphic].Shape()

        self.elements.shapes[1] = graphic

        self.canvas.queue_draw()

    def export(self, *args):
        name = self.combobox.get_active_id()
        self.export_dialog.run(self.elements, name)

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
