#!/usr/bin/env python3

from context import Context

name = "PNG Image"

def export(width, height, filename, elements):
    ctx = Context(width, height)

    elements.draw(ctx)

    ctx.surface.write_to_png(filename)
