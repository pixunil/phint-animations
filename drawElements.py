#!/usr/bin/env python3

import cairo

class Background:
    grey = 0

    def __init__(self):
        self.gradient = cairo.RadialGradient(0, 0, 0, 0, 0, 1)
        self.gradient.add_color_stop_rgb(0, 1, 1, 1)
        self.gradient.add_color_stop_rgb(1, .78, .78, .78)

    def draw(self, ctx, sizes):
        ctx.set_source(self.gradient)
        ctx.rectangle(-sizes.width, -sizes.height, 2 * sizes.width, 2 * sizes.height)
        ctx.fill()
