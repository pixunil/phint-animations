#!/usr/bin/env python3

import sys
import os

from graphics import graphics, backgrounds
import export

from drawElements import Group

gradients = {}

for name, background in backgrounds.items():
    gradients[name] = background.Background()

directory = sys.argv[1]

width = 1366
height = 768

elements = Group()
elements.add(None, None)

for name, graphic in graphics.items():
    elements[1] = graphic.Shape()

    for sub, background in gradients.items():
        elements[0] = background

        filename = "{}-{}.png".format(name, sub)
        filename = os.path.join(directory, filename)

        export.png.export(width, height, filename, elements)
