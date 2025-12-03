from functools import cache


@cache
def max_joltage(l: str, i: int, n: int):
    if i == len(l) and n != 0:
        return float("-inf")
    if n == 0:
        return 0
    return max(max_joltage(l, i+1, n), int(l[i]) * 10**(n-1) + max_joltage(l, i+1, n-1))

ans = 0
while True:
    try:
        l = input()
    except:
        break
    ans += max_joltage(l, 0, 12)
print(ans)
