# Part 1 but works with gigantic input

def rsum(n: int, r: int) -> int:
    l = len(str(n))
    if l > 1:
        nine = 10**(l-1) - 1
        return (n*(n+1) // 2 - nine*(nine+1) // 2) * (1 + 10**l) + rsum(nine, r)
    else:
        return n*(n+1) // 2 * (1 + 10**l)

def stem2(n: int) -> int:
    if n < 10:
        return 0
    l = len(str(n))
    p = (10**(l//2))
    if l % 2 == 1:
        return p - 1
    else:
        head = n // p
        if head*p + head <= n:
            return head
        else:
            return head-1

ranges = input().split(",")
p1_ans = 0
for s in ranges:
    l, r = map(int, s.split("-"))
    p1_ans += rsum(stem2(r), 2) - rsum(stem2(l-1), 2)

print("Part 1:", p1_ans)
