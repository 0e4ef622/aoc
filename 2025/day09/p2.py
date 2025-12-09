from bisect import bisect_left
from collections import deque
import sys

inp = sys.stdin.read().split()

inp = [(a, b) for a, b in [map(int, x.split(',')) for x in inp]]

xc = {p[0] for p in inp}
yc = {p[1] for p in inp}
xc = [-1] + sorted(xc) + [int(1e9)]
yc = [-1] + sorted(yc) + [int(1e9)]
# print(f"{xc = }")
# print(f"{yc = }")
g = [[0] * len(xc) for _ in yc]

# draw the polygon
for p0, p1 in zip(inp, inp[1:] + [inp[0]]):
    p0c = bisect_left(xc, p0[0])
    p0r = bisect_left(yc, p0[1])
    p1c = bisect_left(xc, p1[0])
    p1r = bisect_left(yc, p1[1])
    for r in range(min(p0r, p1r), max(p0r, p1r)+1):
        for c in range(min(p0c, p1c), max(p0c, p1c)+1):
            g[r][c] = 1

# fill the outside with 2's
queue = deque([(0, 0)])
visited = {(0, 0)}
while len(queue):
    r, c = queue.popleft()
    g[r][c] = 2
    def enq(r, c):
        if (r, c) not in visited and 0 <= r < len(yc) and 0 <= c < len(xc):
            visited.add((r, c))
            if g[r][c] == 0:
                queue.append((r, c))
    enq(r+1, c)
    enq(r-1, c)
    enq(r, c+1)
    enq(r, c-1)

# replace 0 with 1
# and 2 with 0
for r in g:
    for i in range(len(r)):
        if r[i] == 0:
            r[i] = 1
        elif r[i] == 2:
            r[i] = 0

# for r in g:
#     print(''.join(map(str, r)))
# print()

pfs = [[0] * (len(xc)+1) for _ in range(len(yc)+1)]
for r in range(len(yc)):
    for c in range(len(xc)):
        pfs[r+1][c+1] += pfs[r+1][c] + g[r][c]

for c in range(len(xc)):
    for r in range(len(yc)):
        pfs[r+1][c+1] += pfs[r][c+1]

# for r in pfs:
#     print(''.join(map(lambda n: f"{n:x}", r)))

ans = 0
for i in range(len(inp)):
    for j in range(i+1, len(inp)):
        ic = bisect_left(xc, inp[i][0])
        ir = bisect_left(yc, inp[i][1])
        jc = bisect_left(xc, inp[j][0])
        jr = bisect_left(yc, inp[j][1])
        l = min(ic, jc)
        r = max(ic, jc)
        u = min(ir, jr)
        d = max(ir, jr)
        cca = pfs[d+1][r+1] - pfs[u][r+1] - pfs[d+1][l] + pfs[u][l]
        if cca == (d-u+1) * (r-l+1):
            a = (abs(inp[i][0] - inp[j][0]) + 1) * (abs(inp[i][1] - inp[j][1]) + 1)
            ans = max(ans, a)
print(ans)
