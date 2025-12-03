def max_joltage(l):
    ans = -1
    for i in range(0, len(l)-1):
        for j in range(i+1, len(l)):
            ans = max(ans, int(l[i])*10 + int(l[j]))
    return ans

ans = 0
while True:
    try:
        l = input()
    except:
        break
    print(max_joltage(l))
    ans += max_joltage(l)
print(ans)
