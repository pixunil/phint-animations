#!/usr/bin/env python3

from importlib import import_module
from collections import OrderedDict

backgrounds = OrderedDict()

for name in ("pixinent", "unient", "linient"):
    backgrounds[name] = import_module("." + name, "graphics.backgrounds")
