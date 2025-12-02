import re
ranges = input().split(",")

ans = 0
for s in ranges:
    l, r = map(int, s.split("-"))
    for i in range(l, r+1):
        s = str(i)
        if m := re.fullmatch(r"(\d+)\1", s):
            ans += i
print(ans)
