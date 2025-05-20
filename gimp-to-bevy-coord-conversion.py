#!/bin/python
import sys

x = float(sys.argv[1])
y = float(sys.argv[2])

SCREEN_WIDTH = 1366.0
SCREEN_HEIGHT = 768.0
print(-SCREEN_WIDTH/2.0 + x)
print(SCREEN_HEIGHT/2.0 - y)
