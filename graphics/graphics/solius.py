#!/usr/bin/env python3

import drawElements

class UpperCaret(drawElements.Line):
    line_width = .05

    def __init__(self):
        drawElements.Line.__init__(self)

        self.add_point(-.1, .55)
        self.add_point(0, .6)
        self.add_point(.1, .55)

class Oval(drawElements.Oval):
    def __init__(self):
        drawElements.Oval.__init__(self, 0, 0, .6, .4)

class Dot(drawElements.Arc):
    def __init__(self):
        drawElements.Arc.__init__(self, 0, 0, .05)

class LowerLine(drawElements.Line):
    line_width = .05

    def __init__(self):
        drawElements.Line.__init__(self)

        self.add_point(-.1, -.55)
        self.add_point(.1, -.55)

class Shape(drawElements.Shape):
    color = (1, 1, 0)

    def __init__(self):
        drawElements.Shape.__init__(self)

        self.shapes = (
            UpperCaret(),
            Oval(),
            Dot(),
            LowerLine()
        )
