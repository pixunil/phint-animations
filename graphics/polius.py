#!/usr/bin/env python3

from math import *

class Shape:
    def draw(self, ctx):
        ctx.source = (0, 0, 1)

        for angle in range(90, 360, 120):
            x = .8 * cos(radians(angle))
            y = .8 * sin(radians(angle))

            start = radians(angle - 230)
            end = radians(angle - 130)

            ctx.arc(x, y, .25, start, end)

            x -= .25 * cos(radians(angle - 30))
            y -= .25 * sin(radians(angle - 30))
            ctx.moveTo(x, y)

            x = .8 * cos(radians(angle + 120)) - .25 * cos(radians(angle + 150))
            y = .8 * sin(radians(angle + 120)) - .25 * sin(radians(angle + 150))
            ctx.lineTo(x, y)

            ctx.stroke()
