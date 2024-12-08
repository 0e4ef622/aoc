import time
from pprint import pprint
from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()
g = list(map(list, data.splitlines()))
W = len(g[0])
H = len(g)

freq_p = defaultdict[str, list[tuple[int, int]]](list)
for i, r in enumerate(g):
    for j, c in enumerate(r):
        if c != '.':
            freq_p[c].append((j, i))

antinodes = set()
for f, p in freq_p.items():
    for i, p1 in enumerate(p):
        for p2 in p[i+1:]:
            dx, dy = p1[0] - p2[0], p1[1] - p2[1]
            antinodes.add((p1[0] + dx, p1[1] + dy));
            antinodes.add((p2[0] - dx, p2[1] - dy));


antinodes = list(filter(lambda p: 0 <= p[0] < W and 0 <= p[1] < H, antinodes))
print(len(antinodes))



antinodes = set()
for f, p in freq_p.items():
    for i, p1 in enumerate(p):
        for p2 in p[i+1:]:
            dx, dy = p1[0] - p2[0], p1[1] - p2[1]
            # g = M.gcd(dx, dy)
            # dx //= g
            # dy //= g
            for m in range(-60, 60):
                antinodes.add((p1[0] + m*dx, p1[1] + m*dy));


antinodes = list(filter(lambda p: 0 <= p[0] < W and 0 <= p[1] < H, antinodes))
print(len(antinodes))
