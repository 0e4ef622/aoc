import sys
g = sys.stdin.read().split()

def rm(g):
    g2 = []
    r = 0
    for i in range(len(g)):
        g2.append([])
        for j in range(len(g[i])):
            cnt = -1
            if g[i][j] != '@':
                g2[-1].append(".")
                continue
            for di in [-1, 0, 1]:
                for dj in [-1, 0, 1]:
                    if 0 <= i+di < len(g) and 0 <= j+dj < len(g[0]):
                        if g[i+di][j+dj] == '@':
                            cnt += 1
            if cnt < 4:
                g2[-1].append(".")
                r += 1
            else:
                g2[-1].append("@")
    return r, g2

ans = 0
while True:
    r, g = rm(g)
    ans += r
    if r == 0:
        print(ans)
        break
