import sys

inp = sys.stdin.read()

[ranges, ids] = inp.split("\n\n")
ranges = [[*map(int, x.split("-"))] for x in ranges.split()]
ids = list(map(int, ids.split()))

ranges.sort()
evs = []

for l, r in ranges:
    evs.append((l, 1))
    evs.append((r+1, -1))

evs.sort()
acc = 0
ans = 0
prev = 0
for (i, a) in evs:
    if acc:
        ans += i - prev
    acc += a
    prev = i

print(ans)
