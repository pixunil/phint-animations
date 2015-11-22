#!/usr/bin/env python3

from importlib import import_module
from collections import OrderedDict

graphics = OrderedDict()

for name in (
    "ulonius", "navulius", "polius",
    "solius"
):
    graphics[name] = import_module("." + name, "graphics.graphics")
