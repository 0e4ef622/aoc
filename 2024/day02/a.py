from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()

def allin(a):
    for x, y in zip(a, a[1:]):
        if not (1 <= y-x <= 3):
            return False
    return True
def allde(a):
    for x, y in zip(a, a[1:]):
        if not (1 <= x-y <= 3):
            return False
    return True

c = 0
for l in data.splitlines():
    *n, = map(int, l.split())
    c += allin(n) or allde(n)

print(c)


c = 0
for l in data.splitlines():
    *n, = map(int, l.split())
    for j in range(len(n)+1):
        nn = n[:j] + n[j+1:]
        if allin(nn) or allde(nn):
            c += 1
            break

print(c)
