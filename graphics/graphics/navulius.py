#!/usr/bin/env python3

from math import pi, radians
import drawElements

class InnerArc(drawElements.Arc):
    line_width = .05

    def __init__(self, angle):
        start = radians(angle)
        end = radians(angle + 45)

        drawElements.Arc.__init__(self, 0, 0, .3, start, end)

class SmallOuterArc(drawElements.Arc):
    line_width = .05

    def __init__(self, angle):
        start = radians(angle)
        end = radians(angle + 45)

        drawElements.Arc.__init__(self, 0, 0, .6, start, end)

class BigOuterArc(drawElements.Arc):
    def __init__(self, angle):
        start = radians(angle)
        end = radians(angle + 90)

        drawElements.Arc.__init__(self, 0, 0, .6, start, end)

class Shape(drawElements.Group):
    color = (0, 1, 0)

    def __init__(self):
        drawElements.Group.__init__(self)

        for i in range(4):
            angle = i * 90 + 22.5
            self.add(InnerArc(angle))

        for i in range(2):
            angle = i * 180 - 22.5
            self.add(SmallOuterArc(angle))

        for i in range(2):
            angle = i * 180 + 45
            self.add(BigOuterArc(angle))
