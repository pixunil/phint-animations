#!/usr/bin/env python3

import drawElements

class UpperCaret(drawElements.Path):
    line_width = .05

    def __init__(self):
        drawElements.Path.__init__(self)

        self.line(-.1, .55)
        self.line(0, .6)
        self.line(.1, .55)

class Oval(drawElements.Oval):
    def __init__(self):
        drawElements.Oval.__init__(self, 0, 0, .6, .4)

class Dot(drawElements.Arc):
    style = "fill"

    def __init__(self):
        drawElements.Arc.__init__(self, 0, 0, .05)

class LowerLine(drawElements.Path):
    line_width = .05

    def __init__(self):
        drawElements.Path.__init__(self)

        self.line(-.1, -.55)
        self.line(.1, -.55)

class Shape(drawElements.Group):
    color = (1, 1, 0)

    def __init__(self):
        drawElements.Group.__init__(self)

        self.add(
            UpperCaret(),
            Oval(),
            Dot(),
            LowerLine()
        )
