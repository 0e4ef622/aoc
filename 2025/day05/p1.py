import sys

inp = sys.stdin.read()

[ranges, ids] = inp.split("\n\n")
ranges = [[*map(int, x.split("-"))] for x in ranges.split()]
ids = list(map(int, ids.split()))

ans = 0
for id in ids:
    found = False
    for l, r in ranges:
        if l <= id <= r:
            found = True
            break
    ans += found
print(ans)
