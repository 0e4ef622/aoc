from pprint import pprint
from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()
lines = data.splitlines()

def rot(g):
    g2 = [""]*len(g[0])
    for i, r in enumerate(g):
        for j, c in enumerate(r[::-1]):
            g2[j] += c
    return g2

def sft(g):
    return ["."*i + s + "."*(len(g[0])-i-1) for i, s in enumerate(g)]

ans = 0
for l in [
    *lines,
    *rot(lines),
    *rot(rot(lines)),
    *rot(rot(rot(lines))),
    *rot(sft(lines)),
    *rot(rot(rot(sft(lines)))),
    *rot(sft(lines[::-1])),
    *rot(rot(rot(sft(lines[::-1])))),
]:
    ans += len(re.findall("XMAS", l))

print(ans)
