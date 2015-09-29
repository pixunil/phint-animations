#!/usr/bin/env python3

import cairo
import math

lineJoinMap = {
    "miter": cairo.LINE_JOIN_MITER,
    "round": cairo.LINE_JOIN_ROUND,
    "bevel": cairo.LINE_JOIN_BEVEL
}

class Context:
    def __init__(self, width, height, ctx):
        self.ctx = ctx

        # pick the smaller of width and height, to have the whole picture fitted
        size = min(width, height)

        # normalize context and change vertical direction
        self.scale(size / 2, -size / 2)
        # align the point 0,0 to the middle
        self.translate(width / size, -height / size)
        self.lineWidth = .1
        self.lineJoin = "round"

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

    @property
    def lineWidth(self):
        return self.ctx.get_line_width()

    @lineWidth.setter
    def lineWidth(self, lineWidth):
        self.ctx.set_line_width(lineWidth)

    @property
    def lineJoin(self):
        lineJoin = self.ctx.get_line_join()
        for key, value in lineJoinMap:
            if value == lineJoin:
                return key

    @lineJoin.setter
    def lineJoin(self, lineJoin):
        try:
            lineJoin = lineJoinMap[lineJoin]
        except:
            raise ArgumentError("Invalid line join")

        self.ctx.set_line_join(lineJoin)

    def moveTo(self, x, y):
        self.ctx.move_to(x, y)

    def lineTo(self, x, y):
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
