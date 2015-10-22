#!/usr/bin/env python3

from utils import *
import drawElements
from context import Context
from export import ExportDialog
from graphics import graphics, backgrounds
from gi.repository import Gtk

class ToolComboBox(Gtk.ToolItem):
    def __init__(self, iterable, callback):
        Gtk.ToolItem.__init__(self)

        model = Gtk.ListStore(str)

        for name in iterable:
            model.append([name])

        self.combobox = Gtk.ComboBox.new_with_model(model)
        self.add(self.combobox)

        renderer = Gtk.CellRendererText()
        self.combobox.pack_start(renderer, True)
        self.combobox.add_attribute(renderer, "text", 0)
        self.combobox.set_id_column(0)
        self.combobox.set_active(0)

        self.callback = callback
        self.combobox.connect("changed", self.on_changed)
        callback(self.name)

    @property
    def name(self):
        return self.combobox.get_active_id()

    def on_changed(self, combobox):
        self.callback(self.name)

class Window(Gtk.Window):
    def __init__(self):
        Gtk.Window.__init__(self, icon_name = "applications-graphics", title = "phint")

        self.connect("destroy", self.on_destroy)

        self.elements = drawElements.Shape()
        self.elements.shapes = [None, None]

        box = Gtk.Box(orientation = Gtk.Orientation.VERTICAL)
        self.add(box)

        bar = Gtk.Toolbar(orientation = Gtk.Orientation.HORIZONTAL)
        box.pack_start(bar, False, False, 0)

        self.canvas = Canvas(self)
        box.pack_start(self.canvas, True, True, 0)

        export = Gtk.ToolButton(label = "Export", icon_name = "document-save")
        export.connect("clicked", self.export)
        bar.insert(export, -1)

        self.graphic_chooser = ToolComboBox(graphics, self.on_graphic_changed)
        bar.insert(self.graphic_chooser, -1)

        self.background_chooser = ToolComboBox(backgrounds, self.on_background_changed)
        bar.insert(self.background_chooser, -1)

        self.set_icon_name("applications-graphics")
        self.set_title("phint")
        self.maximize()
        self.show_all()

    def on_destroy(self, *args):
        Gtk.main_quit()

    def on_graphic_changed(self, graphic):
        # get the module and create the shape
        graphic = graphics[graphic].Shape()

        self.elements.shapes[1] = graphic

        self.canvas.queue_draw()

    def on_background_changed(self, background):
        # get the module and create the background
        background = backgrounds[background].Background()

        self.elements.shapes[0] = background

        self.canvas.queue_draw()

    def export(self, *args):
        name = "{}-{}".format(self.graphic_chooser.name, self.background_chooser.name)

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
