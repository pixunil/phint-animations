#!/usr/bin/env python3

import cairo
import math

class contextProperty:
    def __init__(self, func):
        self.name = func.__name__

    def __get__(self, instance, owner):
        # call the cairo getter
        getter = getattr(instance.ctx, "get_" + self.name)
        return getter()

    def __set__(self, instance, value):
        # call the cairo setter
        setter = getattr(instance.ctx, "set_" + self.name)
        setter(value)

class contextEnum(contextProperty):
    def __init__(self, func):
        contextProperty.__init__(self, func)
        upper_name = self.name.upper()

        self.options = {}

        # take available values from the function
        for option in func():
            value = getattr(cairo, upper_name + "_" + option.upper())
            self.options[option] = value

    def __get__(self, instance, owner):
        value = contextProperty.__get__(self, instance, owner)

        for option in self.options:
            if self.options[option] == value:
                return option

        raise AssertionError("%s has an unknown value %s" % (self.name, value))

    def __set__(self, instance, value):
        # lookup for the value in the dictionary
        try:
            value = self.options[value]
        except KeyError:
            # replace the KeyError with a helpful message
            message = "%s got unknown value %s, choose from %s" % (self.name, value, ", ".join(self.options.keys()))
            raise ValueError(message) from None

        contextProperty.__set__(self, instance, value)

class contextScalar(contextProperty):
    def __init__(self, func):
        contextProperty.__init__(self, func)
        self.type = func()

    def __set__(self, instance, value):
        # force the value to have the saved type
        contextProperty.__set__(self, instance, self.type(value))

class Context:
    def __init__(self, width, height, ctx = None):
        if isinstance(ctx, cairo.Context):
            self.surface = ctx.get_target()
            self.ctx = ctx
        elif isinstance(ctx, cairo.Surface):
            surface = ctx
            self.surface = surface
            self.ctx = cairo.Context(surface)
        else:
            format = ctx
            if format is None:
                format = cairo.FORMAT_ARGB32
            self.surface = cairo.ImageSurface(format, width, height)
            self.ctx = cairo.Context(self.surface)

        # pick the smaller of width and height, to have the whole picture fitted
        size = min(width, height)

        # normalize context and change vertical direction
        self.scale(size / 2, -size / 2)
        # align the point 0,0 to the middle
        self.translate(width / size, -height / size)
        self.line_width = .1
        self.line_join = "round"

        self.width = width / size
        self.height = height / size

    @property
    def source(self):
        return self.ctx.get_source()

    @source.setter
    def source(self, source):
        if isinstance(source, cairo.Pattern):
            self.ctx.set_source(source)
            return
        if isinstance(source, (tuple, list)):
            if len(source) == 3:
                (red, green, blue) = source
                self.ctx.set_source_rgb(red, green, blue)
                return
            if len(source) == 4:
                (red, green, blue, alpha) = source
                self.ctx.set_source_rgba(red, green, blue, alpha)
                return

        raise ValueError("Source should have either three members (rgb), four (rgba) or be a cairo.Pattern object")

    @contextScalar
    def line_width():
        return float

    @contextEnum
    def line_join():
        return ("miter", "round", "bevel")

    @contextEnum
    def line_cap():
        return ("butt", "round", "square")

    def move_to(self, x, y):
        self.ctx.move_to(x, y)

    def line_to(self, x, y):
        self.ctx.line_to(x, y)

    def rectangle(self, x, y, width, height):
        self.ctx.rectangle(x, y, width, height)

    def arc(self, x, y, radius, start = 0, end = math.pi * 2, clockwise = False):
        if clockwise:
            self.ctx.arc_negative(x, y, radius, start, end)
        else:
            self.ctx.arc(x, y, radius, start, end)

    def fill(self):
        self.ctx.fill()

    def stroke(self):
        self.ctx.stroke()

    def paint(self):
        self.ctx.paint()

    def save(self):
        self.ctx.save()

    def restore(self):
        self.ctx.restore()

    def scale(self, sx, sy = None):
        if sy is None:
            sy = sx

        self.ctx.scale(sx, sy)

    def translate(self, x, y):
        self.ctx.translate(x, y)
