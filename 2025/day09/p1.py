import sys

inp = sys.stdin.read().split()

inp = [(a, b) for a, b in [map(int, x.split(',')) for x in inp]]

ans = 0
for i in range(len(inp)):
    for j in range(i+1, len(inp)):
        ans = max(ans, (abs(inp[i][0] - inp[j][0]) + 1) * (abs(inp[i][1] - inp[j][1]) + 1))
print(ans)
