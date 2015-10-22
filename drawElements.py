#!/usr/bin/env python3

import cairo
import math

class Background:
    def __init__(self):
        self.gradient = cairo.RadialGradient(0, 0, 0, 0, 0, 1)
        self.gradient.add_color_stop_rgb(0, 1, 1, 1)
        self.gradient.add_color_stop_rgb(1, .78, .78, .78)

    def draw(self, ctx):
        ctx.source = self.gradient
        ctx.paint()

class Line:
    line_width = .1

    def __init__(self):
        self.points = []

    def add_point(self, x, y):
        self.points.append((x, y))

    def draw(self, ctx):
        ctx.line_width = self.line_width

        ctx.move_to(*self.points[0])

        for point in self.points[1:]:
            ctx.line_to(*point)

        ctx.stroke()

class Arc:
    def __init__(self, x, y, radius, start = 0, end = math.pi * 2):
        self.x = x
        self.y = y
        self.radius = radius
        self.start = start
        self.end = end

    def draw(self, ctx):
        ctx.arc(self.x, self.y, self.radius, self.start, self.end)
        ctx.stroke()

class Oval:
    def __init__(self, x, y, rx, ry):
        self.x = x
        self.y = y
        self.rx = rx
        self.ry = ry

    def draw(self, ctx):
        ctx.save()
        ctx.translate(self.x, self.y)
        ctx.scale(self.rx, self.ry)

        ctx.arc(0, 0, 1)

        ctx.restore()
        ctx.stroke()

class Shape:
    color = None

    def __init__(self):
        self.shapes = []

    def draw(self, ctx):
        if self.color:
            ctx.source = self.color

        for shape in self.shapes:
            shape.draw(ctx)
