import time
from pprint import pprint
from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()
g = list(map(list, data.splitlines()))


cx, cy = 0, 0
for y in range(len(g)):
    for x in range(len(g[0])):
        if g[y][x] == '^':
            cx, cy = x, y
            break

initx, inity = cx, cy
rot = {
    '^': '>',
    '>': 'v',
    'v': '<',
    '<': '^',
}
dir = '^'
# v = {(cx, cy)}
# while True:
#     ox, oy = cx, cy
#     match dir:
#         case '^':
#             cy -= 1
#         case '>':
#             cx += 1
#         case 'v':
#             cy += 1
#         case '<':
#             cx -= 1
#     if not (0 <= cy < len(g)) or not (0 <= cx < len(g[0])):
#         break
#     if g[cy][cx] == '#':
#         dir = rot[dir]
#         cx, cy = ox, oy
#     v.add((cx, cy))
# print(len(v))


def ck():
    cx, cy = initx, inity
    dir = '^'
    v = {('^', cx, cy)}
    t = 0
    while True:
        t += 1
        ox, oy = cx, cy
        match dir:
            case '^':
                cy -= 1
            case '>':
                cx += 1
            case 'v':
                cy += 1
            case '<':
                cx -= 1
        if not (0 <= cy < len(g)) or not (0 <= cx < len(g[0])):
            break
        if g[cy][cx] == '#':
            dir = rot[dir]
            cx, cy = ox, oy
        if (dir, cx, cy) in v:
            return True
        v.add((dir, cx, cy))
    return False


def ne(dir, cx, cy):
    match dir:
        case '^':
            cy -= 1
        case '>':
            cx += 1
        case 'v':
            cy += 1
        case '<':
            cx -= 1
    return cx, cy

v = {('^', cx, cy): 0}
t = 0
while True:
    t += 1
    ox, oy = cx, cy
    match dir:
        case '^':
            cy -= 1
        case '>':
            cx += 1
        case 'v':
            cy += 1
        case '<':
            cx -= 1
    if not (0 <= cy < len(g)) or not (0 <= cx < len(g[0])):
        break
    if g[cy][cx] == '#':
        dir = rot[dir]
        cx, cy = ox, oy
    v[(dir, cx, cy)] = t

ans = set()

le = len(v)
for i, ((d, sx, sy), st) in enumerate(v.items()):
    j, i = ne(d, sx, sy)
    try:
        if g[i][j] != '#':
            g[i][j] = '#'
            if ck():
                ans.add((i, j))
            g[i][j] = '.'
    except:
        pass
print(len(ans))
