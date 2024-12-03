from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()

r = 0
for m in re.finditer(r"mul\((\d+),(\d+)\)", data):
    r += int(m.group(1)) * int(m.group(2))
print(r)

r=0
e=True
for m in re.finditer(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)", data):
    if m.group().startswith("mul") and e:
        r += int(m.group(1)) * int(m.group(2))
    elif m.group().startswith("don't"):
        e = False
    elif m.group().startswith("do"):
        e = True
print(r)
