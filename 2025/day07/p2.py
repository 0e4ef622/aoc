from collections import defaultdict
import sys

inp = sys.stdin.read().split()
w = len(inp[0])
h = len(inp)


p = {inp[0].find("S"): 1}
ans=0
for i in range(1, h):
    p2 = defaultdict(int)
    for j in p:
        if inp[i][j] == "^":
            p2[j+1] += p[j]
            p2[j-1] += p[j]
        else:
            p2[j] += p[j]
    p = p2
print(sum(p.values()))
