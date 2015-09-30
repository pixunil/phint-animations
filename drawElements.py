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
        ctx.rectangle(-ctx.width, -ctx.height, 2 * ctx.width, 2 * ctx.height)
        ctx.fill()

class Line:
    def __init__(self):
        self.points = []

    def addPoint(self, x, y):
        self.points.append((x, y))

    def draw(self, ctx):
        ctx.moveTo(*self.points[0])

        for point in self.points[1:]:
            ctx.lineTo(*point)

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

class Shape:
    color = None

    def __init__(self):
        self.shapes = []

    def draw(self, ctx):
        if self.color:
            ctx.source = self.color

        for shape in self.shapes:
            shape.draw(ctx)
