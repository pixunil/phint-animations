#!/usr/bin/env python3

import cairo

class Background:
    grey = 0

    def __init__(self):
        self.gradient = cairo.RadialGradient(0, 0, 0, 0, 0, 1)
        self.gradient.add_color_stop_rgb(0, 1, 1, 1)
        self.gradient.add_color_stop_rgb(1, .78, .78, .78)

    def draw(self, ctx):
        ctx.source = self.gradient
        ctx.rectangle(-ctx.width, -ctx.height, 2 * ctx.width, 2 * ctx.height)
        ctx.fill()
