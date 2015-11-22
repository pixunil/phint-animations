#!/usr/bin/env python3

import drawElements
from math import pi

class Curve(drawElements.Path):
    def __init__(self, direction):
        drawElements.Path.__init__(self)

        if direction == -1:
            end = 0
            clockwise = True
        else:
            end = pi
            clockwise = False

        self.arc(direction * .4, .4, .2, pi / 2, end, clockwise)
        self.line(direction * .2, -.5)

class Bar(drawElements.Path):
    def __init__(self, direction):
        drawElements.Path.__init__(self)

        self.line(direction * .4, .3)
        self.line(direction * .4, -.4)

class Side(drawElements.Group):
    def __init__(self, direction):
        self.add(
            Curve(direction),
            Bar(direction)
        )

class Shape(drawElements.Group):
    color = (1, 0, 0)

    def __init__(self):
        drawElements.Group.__init__(self)

        self.add(
            Side(-1),
            Side(1)
        )
