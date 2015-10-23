#!/usr/bin/env python3

import cairo
import math

class Gradient:
    def __init__(self):
        self.gradient = cairo.RadialGradient(0, 0, 0, 0, 0, 1)
        self.gradient.add_color_stop_rgb(0, *self.start)
        self.gradient.add_color_stop_rgb(1, *self.end)

    def draw(self, ctx):
        ctx.source = self.gradient
        ctx.paint()

class Shape:
    x = 0
    y = 0

    width = 1
    height = 1

    style = "stroke"
    line_width = .1
    line_join = "round"

    def begin(self, ctx):
        ctx.save()
        ctx.translate(self.x, self.y)
        ctx.scale(self.width, self.height)

    def finish(self, ctx):
        ctx.restore()

        if self.style == "stroke":
            ctx.line_width = self.line_width
            ctx.line_join = self.line_join
            ctx.stroke()
        elif self.style == "fill":
            ctx.fill()

class Line(Shape):
    def __init__(self):
        self.points = []

    def add_point(self, x, y):
        self.points.append((x, y))

    def draw(self, ctx):
        self.begin(ctx)

        ctx.move_to(*self.points[0])

        for point in self.points[1:]:
            ctx.line_to(*point)

        self.finish(ctx)

class Arc(Shape):
    start = 0
    end = math.pi * 2

    def __init__(self, x, y, radius, start = 0, end = math.pi * 2):
        self.x = x
        self.y = y
        self.width = radius
        self.height = radius
        self.start = start
        self.end = end

    def draw(self, ctx):
        self.begin(ctx)

        ctx.arc(0, 0, 1, self.start, self.end)

        self.finish(ctx)

class Oval(Arc):
    def __init__(self, x, y, rx, ry, start = 0, end = math.pi * 2):
        self.x = x
        self.y = y
        self.width = rx
        self.height = ry
        self.start = start
        self.end = end

class Group(list):
    color = None

    def add(self, *shapes):
        for shape in shapes:
            self.append(shape)

    def draw(self, ctx):
        if self.color:
            ctx.source = self.color

        for shape in self:
            shape.draw(ctx)
