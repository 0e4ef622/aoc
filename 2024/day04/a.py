from pprint import pprint
from collections import *
from itertools import *
from functools import *
import sys, os, re, math as M

data = sys.stdin.read()
g = [""] + ["." + s + "." for s in data.splitlines()] + [""]

ans = 0
for i in range(len(g)):
    for j in range(len(g[i])):
        for di in [-1, 0, 1]:
            for dj in [-1, 0, 1]:
                if di != 0 or dj != 0:
                    try:
                        if g[i][j] + g[i+di][j+dj] + g[i+2*di][j+2*dj] + g[i+3*di][j+3*dj] == "XMAS":
                            ans += 1;
                    except:
                        pass
print(ans)

ans = 0
for i in range(len(g)):
    for j in range(len(g[i])):
        try:
            if (g[i-1][j-1] + g[i][j] + g[i+1][j+1] in ["MAS", "SAM"] and
                    g[i-1][j+1] + g[i][j] + g[i+1][j-1] in ["MAS", "SAM"]):
                ans += 1;
        except:
            pass

print(ans)
