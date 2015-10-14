#!/usr/bin/env python3

from context import Context
from cairo import SVGSurface

name = "SVG Vector"

def export(width, height, filename, elements):
    surface = SVGSurface(filename, width, height)
    ctx = Context(width, height, surface)

    elements.draw(ctx)

    surface.finish()
