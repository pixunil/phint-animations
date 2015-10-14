#!/usr/bin/env python3

class lazy:
    def __init__(self, fget):
        self.fget = fget

    def __get__(self, instance, owner):
        value = self.fget(instance)
        setattr(instance, self.fget.__name__, value)

        return value
