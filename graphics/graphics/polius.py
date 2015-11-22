#!/usr/bin/env python3

from math import *
import drawElements

class Line(drawElements.Path):
    def __init__(self, angle):
        drawElements.Path.__init__(self)

        self.line(
            .8 * cos(radians(angle)) - .25 * cos(radians(angle - 30)),
            .8 * sin(radians(angle)) - .25 * sin(radians(angle - 30))
        )

        self.line(
            .8 * cos(radians(angle + 120)) - .25 * cos(radians(angle + 150)),
            .8 * sin(radians(angle + 120)) - .25 * sin(radians(angle + 150))
        )

class Arc(drawElements.Arc):
    def __init__(self, angle):
        x = .8 * cos(radians(angle))
        y = .8 * sin(radians(angle))

        start = radians(angle - 230)
        end = radians(angle - 130)

        drawElements.Arc.__init__(self, x, y, .25, start, end)

class Shape(drawElements.Group):
    color = (0, 0, 1)

    def __init__(self):
        drawElements.Group.__init__(self)

        for angle in range(90, 360, 120):
            self.add(
                Line(angle),
                Arc(angle)
            )
