import sys
g = sys.stdin.read().split()

ans = 0
for i in range(len(g)):
    for j in range(len(g[i])):
        cnt = -1
        if g[i][j] != '@':
            continue
        for di in [-1, 0, 1]:
            for dj in [-1, 0, 1]:
                if 0 <= i+di < len(g) and 0 <= j+dj < len(g[0]):
                    if g[i+di][j+dj] == '@':
                        cnt += 1
        if cnt < 4:
            ans += 1
print(ans)
