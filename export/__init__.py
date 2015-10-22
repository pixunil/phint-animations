#!/usr/bin/env python3

from importlib import import_module
from gi.repository import Gtk

types = {}

for filetype in ("png", "svg"):
    types[filetype] = import_module("." + filetype, "export")

class SpinButton(Gtk.SpinButton):
    def __init__(self):
        Gtk.SpinButton.__init__(self)

        self.set_range(1, 2 ** 16)
        self.set_increments(1, 10)
        self.set_value(16)

    @property
    def value(self):
        return self.get_value_as_int()

class ExportDialog(Gtk.FileChooserDialog):
    def __init__(self, window):
        Gtk.FileChooserDialog.__init__(
            self,
            transient_for = window,
            title = "Export",
            action = Gtk.FileChooserAction.SAVE,
            buttons = (
                Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
                Gtk.STOCK_SAVE, Gtk.ResponseType.OK
            ),
            do_overwrite_confirmation = True
        )

        # add options to justify the size of the output
        grid = Gtk.Grid(column_homogeneous = True)
        self.get_content_area().pack_start(grid, True, False, 0)

        grid.attach(Gtk.Label("Width"), 0, 0, 1, 1)
        self.width = SpinButton()
        grid.attach(self.width, 1, 0, 1, 1)

        grid.attach(Gtk.Label("Height"), 0, 1, 1, 1)
        self.height = SpinButton()
        grid.attach(self.height, 1, 1, 1, 1)

        grid.show_all()

        for extension, exporter in types.items():
            filter = Gtk.FileFilter()
            filter.add_pattern("*." + extension)
            filter.set_name(exporter.name)
            # save the extension value to this object, as it will be read later
            filter.extension = extension
            self.add_filter(filter)

    def run(self, elements, name):
        self.set_title("Export " + name)
        self.set_current_name(name)

        result = Gtk.FileChooserDialog.run(self)

        if result == Gtk.ResponseType.OK:
            filename = self.get_filename()
            # get the file extension the user typed in
            extension = filename.split(".")[-1]

            # get the extension selected in the filter, if the typed in extension is not valid
            if extension not in types:
                extension = self.get_filter().extension
                filename += "." + extension

            types[extension].export(self.width.value, self.height.value, filename, elements)

        self.hide()

    def validate(self, *args):
        ending = self.get_filename().split(".")[-1]

        if ending in types:
            self.response(Gtk.ResponseType.OK)
        else:
            self.response(Gtk.ResponseType.CANCEL)
