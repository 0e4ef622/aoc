from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()

d = [[*map(int, s.split())] for s in data.splitlines()]
l1 = sorted([x[0] for x in d])
l2 = sorted([x[1] for x in d])
s = 0
for a, b in zip(l1, l2):
    s += abs(a-b)
print(s)

cc = Counter(l2)
s = 0
for x in l1:
    s += x*cc[x]

print(s)
